use crate::synth_device::*;
//use crate::parameters::*;
//use crate::helpers;

struct SlaughterParams {
}

#[derive(Debug, Clone, Copy)]
struct SlaughterVoice {
}

impl Voice<SlaughterParams> for SlaughterVoice {
    fn new(sample_rate: f64) -> Self {
        SlaughterVoice { }
    }
    fn note_on(&mut self, data: &mut VoiceData, params: &mut SlaughterParams, note: i32, velocity: i32, detune: f32, pan: f32) {
    }
    fn note_off(&mut self, data: &mut VoiceData, params: &mut SlaughterParams) {
    }
    fn note_slide(&mut self, data: &mut VoiceData, params: &mut SlaughterParams, note: i32) {
    }
    fn get_note(&mut self, data: &mut VoiceData, params: &mut SlaughterParams) -> f64 {
        0.0
    }
    fn run(&mut self,
           data: &mut VoiceData,
           params: &mut SlaughterParams,
           song_pos: f64,
           out_offs: usize,
           outputs: &mut [Vec<f64>]) {
    }
}

fn new_slaugher(sample_rate: f64) -> SynthDevice<SlaughterVoice, SlaughterParams> {
    let params = SlaughterParams { };
    let sd : SynthDevice<SlaughterVoice, SlaughterParams> = 
    SynthDevice::new(sample_rate, params);
    sd
}
