use crate::parameters::*;
use crate::helpers;

trait ParameterSet {
    fn enumerate(&self) -> Vec<Parameter>;
}

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
            pan:              0.0,
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

    fn note_slide(&mut self, data: &SynthData, note: i32) {
        self.slide_active     = true;
        self.destination_note = note;

        let slide_time        = 10.0 * helpers::pow(data.slide, 4.0);
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

trait Voice {
    fn note_on(&mut self, data: &mut VoiceData, note: i32, velocity: i32, detune: f32, pan: f32);
    fn note_off(&mut self, data: &mut VoiceData);
    fn note_slide(&mut self, data: &mut VoiceData, note: i32);
    fn get_note(&mut self, data: &mut VoiceData) -> f64;
    fn run(&mut self, data: &mut VoiceData, song_pos: f64, param: &mut ParameterData, outputs: &mut [f64]);
}

struct SynthData {
    voices_unisono: i32,
    voices_detune: f32,
    voices_pan: f32,
    vibrato_freq: f64,
    vibrato_amount: f32,
    rise: f32,
    slide: f32,
}

struct SynthDevice<T, V>
    where V: Voice + ParameterSet {
    voice_data:   [VoiceData; 256],
    voices:       [V; 256],
    mono_active:  bool,
    note_log:     [i32; 128],
    note_count:   i32,
    active_notes: [bool; 128],
    voice_mode:   VoiceMode,
}

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

