use crate::synth_device::*;
use wctr_signal_ops::signals::DemOp;
//use crate::parameters::*;
//use crate::helpers;

// The params should be in the voice's terms for best performance.
// Index them via enum and method calls.
//
// The DemOp should set fixed values directly and remember any
// computed values with their output field index.
//
// => Do we need a Rc/RefCell combination?
//      => DemOp's are owned by the Simulator, so we need some kind of
//         reference between the DemOp and the SlaughterParams in the SynthDevice.
// => Who Owns the slaughter SynthDevice? Isn't that client responsible?
//      => There will be an AudioSimulator, that holds these devices, according
//         to the simulator group!?
//      => Or should we extend signal_ops to care about audio too?

// XXX: We should impl DemOp for the SynthDevice<SlaughterVoice, SlaughterParams>
//      type and package that up for the Simulator.
//      Add a DemOp trait flag for detecting audio ops, they return the "audio bus" index
//      they render from/belong to, the to bus is a second parameter.
//      Render order is by increasing audio bus index and in the order the ops
//      were added.
//      DemOp trait gets a render() function, that takes a complete array of all
//      allocated busses.

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
