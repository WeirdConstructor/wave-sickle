use crate::parameters::*;
use crate::helpers;

#[derive(Debug, PartialEq, Copy, Clone)]
enum EventType {
    None,
    NoteOn,
    NoteOff,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Event {
    typ:           EventType,
    delta_samples: i32,
    note:          i32,
    velocity:      i32,
}

impl Event {
    fn new() -> Event {
        Event {
            typ:           EventType::None,
            delta_samples: 0,
            note:          0,
            velocity:      0,
        }
    }

    fn clear(&mut self) {
        self.typ = EventType::None;
    }
}

trait ParameterSet {
    fn enumerate(&self) -> Vec<Parameter>;
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct VoiceData {
    pub sample_rate:      f64,
    pub is_on:            bool,
    pub note:             i32,
    pub detune:           f32,
    pub pan:              f32,
    pub vibrato_phase:    f64,
        slide_active:     bool,
        slide_delta:      f64,
        slide_samples:    i32,
        destination_note: i32,
        current_note:     f64,
}

impl VoiceData {
    fn new(sample_rate: f64) -> VoiceData {
        VoiceData {
            sample_rate,
            is_on:            false,
            note:             0,
            detune:           0.0,
            pan:              0.5,
            vibrato_phase:    0.0,
            slide_active:     false,
            slide_delta:      0.0,
            slide_samples:    0,
            destination_note: 0,
            current_note:     0.0,
        }
    }

    fn note_on(&mut self, note: i32, velocity: i32, detune: f32, pan: f32) {
        self.is_on        = true;
        self.note         = note;
        self.detune       = detune;
        self.pan          = pan;
        self.current_note = note as f64;
        self.slide_active = false;
    }

    fn note_off(&mut self) {
    }

    fn note_slide<V>(&mut self, data: &SynthDevice<V>, note: i32)
        where V: Voice + ParameterSet {

        self.slide_active     = true;
        self.destination_note = note;

        let slide_time        = 10.0 * helpers::pow(data.slide as f64, 4.0);
        self.slide_delta      = (note as f64 - self.current_note)
                                / self.sample_rate * slide_time;
        self.slide_samples    = (self.sample_rate * slide_time) as i32;
    }

    fn get_note(&mut self) -> f64 {
        if self.slide_active {
            self.current_note += self.slide_delta;
            self.slide_samples -= 1;
            if self.slide_samples < 0 {
                self.note         = self.destination_note;
                self.slide_active = false;
                self.current_note = self.destination_note as f64;
            }
        }

        self.current_note
    }
}

trait Voice: Copy + Clone {
    fn new(sample_rate: f64) -> Self;
    fn note_on(&mut self, data: &mut VoiceData, note: i32, velocity: i32, detune: f32, pan: f32);
    fn note_off(&mut self, data: &mut VoiceData);
    fn note_slide(&mut self, data: &mut VoiceData, note: i32);
    fn get_note(&mut self, data: &mut VoiceData) -> f64;
    fn run(&mut self,
           data: &mut VoiceData,
           param: &mut ParameterData,
           song_pos: f64,
           out_offs: usize,
           outputs: &mut [Vec<f64>]);
}

struct SynthDevice<V>
    where V: Voice + ParameterSet {

    sample_rate:    f64,
    voices_unisono: i32,
    voices_detune:  f32,
    voices_pan:     f32,
    vibrato_freq:   f64,
    vibrato_amount: f32,
    rise:           f32,
    slide:          f32,
    voice_mode:     VoiceMode,
    mono_active:    bool,
    note_log:       [i32; 128],
    note_count:     i32,
    active_notes:   [bool; 128],
    voice_data:     [VoiceData; 256],
    voices:         [V; 256],
    events:         [Event; 256],
}

fn clear_outputs(outputs: &mut [Vec<f64>]) {
    for out in outputs.iter_mut() {
        for sample in out.iter_mut() {
            *sample = 0.0;
        }
    }
}

macro_rules! voice_data_zip {
    ($self: ident) => { $self.voices.iter_mut().zip($self.voice_data.iter_mut()) }
}

macro_rules! detuned_notes_on {
    ($self: ident, $e: ident, $j: ident) => {
        for (v, vd) in voice_data_zip!($self) {
            if $j <= 0 { break; }

            if !vd.is_on {
                $j -= 1;
                let f = if $self.voices_unisono > 1 {
                    $j as f32 / ($self.voices_unisono as f32 - 1.0)
                } else {
                    $j as f32
                };

                v.note_on(
                    vd, $e.note, $e.velocity,
                    f * $self.voices_detune,
                    (f - 0.5) * ($self.voices_pan * 2.0 - 1.0) + 0.5);
            }
        }
    }
}

impl<V: Voice + ParameterSet> SynthDevice<V> {
    fn new(sample_rate: f64) -> Self {
        SynthDevice {
            sample_rate,
            voices_unisono: 1,
            voices_detune:  0.0,
            voices_pan:     0.5,
            vibrato_freq:   helpers::param_to_vibrato_freq(0.0),
            vibrato_amount: 0.0,
            rise:           0.0,
            slide:          0.0,
            voice_mode:     VoiceMode::Polyphonic,
            mono_active:    false,
            note_count:     0,
            active_notes:   [false; 128],
            note_log:       [0; 128],
            voice_data:     [VoiceData::new(sample_rate); 256],
            voices:         [V::new(sample_rate); 256],
            events:         [Event::new(); 256],
        }
    }

    pub fn run(&mut self, mut song_pos: f64, param: &mut ParameterData,
               inputs: &mut [Vec<f64>], outputs: &mut [Vec<f64>]) {

        let mut num_samples = outputs[0].len();
        let orig_num_samples = num_samples;
        clear_outputs(outputs);
        let mut out_offs = 0;

        while num_samples > 0 {
            let mut samples_to_next_event = num_samples as i32;

            for e in self.events.iter_mut() {
                if e.typ == EventType::None {
                    continue;
                }

                match e.typ {
                    EventType::NoteOn => {
                        let mut j = self.voices_unisono;
                        match self.voice_mode {
                            VoiceMode::Polyphonic => {
                                detuned_notes_on!(self, e, j);
                            },
                            VoiceMode::MonoLegatoTrill => {
                                self.active_notes[e.note as usize] = true;
                                self.note_log[self.note_count as usize] = e.note;

                                if !self.mono_active { // no current note active, start new one
                                    self.mono_active = true;
                                    detuned_notes_on!(self, e, j);

                                } else { // mono note active, slide to new note
                                    for (v, vd) in voice_data_zip!(self) {
                                        if vd.is_on {
                                            v.note_slide(vd, e.note);
                                        }
                                    }
                                }
                            },
                        }
                    },
                    EventType::NoteOff => {
                        match self.voice_mode {
                            VoiceMode::Polyphonic => {
                                for (v, vd) in voice_data_zip!(self) {
                                    if vd.is_on && vd.note == e.note {
                                        v.note_off(vd);
                                    }
                                }
                            },
                            VoiceMode::MonoLegatoTrill => {
                                self.active_notes[e.note as usize] = false;
                                let log_note =
                                    self.note_log[(self.note_count - 1) as usize];
                                if e.note == log_note {
                                    while self.note_count > 0 {

                                        if self.active_notes[
                                            self.note_log[
                                                (self.note_count - 1)
                                                as usize]
                                            as usize] {

                                            for (v, vd) in voice_data_zip!(self) {
                                                if vd.is_on {
                                                    v.note_slide(
                                                        vd,
                                                        self.note_log[
                                                            (self.note_count - 1)
                                                            as usize]);
                                                }
                                            }
                                            break;
                                        }

                                        self.note_count -= 1;
                                    }

                                    if self.note_count == 0 {
                                        self.mono_active = false;
                                        for an in self.active_notes.iter_mut() {
                                            *an = false;
                                        }

                                        for (v, vd) in voice_data_zip!(self) {
                                            if vd.is_on {
                                                v.note_off(vd);
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    },
                    _ => (),
                }

                if e.delta_samples == 0 {
                    // XXX: do amazing things here!
                } else if e.delta_samples < samples_to_next_event {
                    samples_to_next_event = e.delta_samples;
                }
            }

            for (v, vd) in self.voices.iter_mut().zip(self.voice_data.iter_mut()) {
                if vd.is_on {
                    v.run(vd, param, song_pos, out_offs, outputs);
                }
            }

            for e in self.events.iter_mut() {
                if e.typ != EventType::None {
                    e.delta_samples -= samples_to_next_event;
                }
            }

            song_pos    += samples_to_next_event as f64 / self.sample_rate;
            out_offs    += samples_to_next_event as usize;
            num_samples -= samples_to_next_event as usize;
        }
    }

    fn all_notes_off(&mut self)
    {
        for (vd, v) in self.voice_data.iter_mut().zip(self.voices.iter_mut()) {
            if vd.is_on { v.note_off(vd); }
        }
        self.mono_active = false;
        self.note_count = 0;
        for an in self.active_notes.iter_mut() {
            *an = false;
        }
        self.clear_events();
    }

    // XXX: Invariant: note_on must only be called with increasing
    //      delta_samples. Otherwise the algorithm in run() will
    //      not behave well. The invariant is, that the self.events
    //      array is sorted by ascending delta_samples.
    fn note_on(&mut self, note: i32, velocity: i32, delta_samples: i32) {
        for ev in self.events.iter_mut() {
            if ev.typ == EventType::None {
                ev.typ           = EventType::NoteOn;
                ev.delta_samples = delta_samples;
                ev.note          = note;
                ev.velocity      = velocity;
                break;
            }
        }
    }

    fn note_off(&mut self, note: i32, delta_samples: i32) {
        for ev in self.events.iter_mut() {
            if ev.typ == EventType::None {
                ev.typ           = EventType::NoteOff;
                ev.delta_samples = delta_samples;
                ev.note          = note;
                break;
            }
        }
    }

    fn set_voice_mode(&mut self, vm: VoiceMode) {
        if self.voice_mode == vm {
            return;
        }

        self.all_notes_off();
        for vd in self.voice_data.iter_mut() {
            vd.is_on = false;
        }
        self.voice_mode = vm;
    }

    fn get_voice_mode(&self) -> VoiceMode { self.voice_mode }

    fn clear_events(&mut self) {
        for e in self.events.iter_mut() { e.clear(); }
    }
}

//struct SynthDevice<V>
//    where V: Voice + ParameterSet {
//    voice_data:   [VoiceData; 256],
//    voices:       [V; 256],
//}

// SynthDevice -> Manages voices and note on/off events
// Slaughter   -> Holds parameters
// Voice -> Generates the sounds, has a poitner to Slaugher/ParameterHolder

// Voice becomes SlaugherVoice with a common set of (runtime changing) parameters.

// Problem: Parameters are maybe changing while the sound is playing!
//          We can't have mutable and non mutable references at the same time.
//          We have multiple references to a single set of parameters.
//          We don't have access at the same time => RefCell?!
//          => Rc/RefCell means there is an indirect access each time a voice
//             does something. But copying the parameter data into
//             the voices on each change is too wasteful too.

