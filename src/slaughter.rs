use crate::synth_device::*;
use wctr_signal_ops::signals::{OpIn, Op, OpPort, OpIOSpec};

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

pub struct SlaughterParams {
    
}

#[derive(Debug, Clone, Copy)]
pub struct SlaughterVoice {
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

impl Op for SynthDevice<SlaughterVoice, SlaughterParams> {
    fn io_spec(&self, index: usize) -> OpIOSpec {
        OpIOSpec {
            inputs: vec![
//                OpPort::new("amp",    0.0, 9999.0),
//                OpPort::new("phase", -2.0 * std::f32::consts::PI,
//                                      2.0 * std::f32::consts::PI),
//                OpPort::new("vert",  -9999.0,  9999.0),
//                OpPort::new("freq",      0.0, 11025.0),
            ],
            input_values: vec![],
            input_defaults: vec![],
            outputs: vec![],
            output_regs: vec![],
            audio_out_groups: vec![],
            index,
        }
    }

    fn init_regs(&mut self, _start_reg: usize, _regs: &mut [f32]) { }

    fn get_output_reg(&mut self, _name: &str) -> Option<usize> { None }

    fn set_input(&mut self, name: &str, to: OpIn, as_default: bool) -> bool {
//        let s = if as_default { &mut self.defaults } else { &mut self.values };
        match name {
//            "amp"   => { s[0] = to; true },
//            "phase" => { s[1] = to; true },
//            "vert"  => { s[2] = to; true },
//            "freq"  => { s[3] = to; true },
            _       => false,
        }
    }

    fn exec(&mut self, t: f32, regs: &mut [f32]) {
//        let a = self.values[0].calc(regs);
//        let p = self.values[1].calc(regs);
//        let v = self.values[2].calc(regs);
//        let f = self.values[3].calc(regs);
//        regs[self.out] = a * (((f * t) + p).sin() + v);
        //d// println!("OUT: {}, {}", regs[self.out], self.out);
    }
}

pub fn new_slaughter(sample_rate: f64) -> SynthDevice<SlaughterVoice, SlaughterParams> {
    let params = SlaughterParams { };
    let sd : SynthDevice<SlaughterVoice, SlaughterParams> = 
        SynthDevice::new(sample_rate, params);
    sd
}
