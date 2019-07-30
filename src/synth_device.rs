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
    fn run(&mut self, data: &mut VoiceData, song_pos: f64, param: &mut ParameterData, outputs: &mut [f64]);
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

