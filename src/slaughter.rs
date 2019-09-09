use crate::synth_device::*;
use crate::helpers::SignalIOParams;
use crate::helpers;
use wctr_signal_ops::signals::{OpIn, Op, OpPort, OpIOSpec, Event};

//use crate::parameters::*;

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
    params:                 SignalIOParams,
    osc1_waveform:          f32,
    osc1_pulse_width:       f32,
    osc1_volume:            f32,
    osc1_detune_coarse:     f32,
    osc1_detune_fine:       f32,
}

impl SlaughterParams {
    pub fn new() -> Self {
        let mut p = SignalIOParams::new();

        p.input("o1_wav",     0.0, 1.0, 0.0);
        p.input("o1_pw",      0.0, 1.0, 0.5);
        p.input("o1_detc",    0.0, 1.0, 0.0);
        p.input("o1_detf",    0.0, 1.0, 0.0);
        p.input("o1_vol",     0.0, 1.0, 1.0);

        SlaughterParams {
            osc1_waveform:      p.v(0),
            osc1_pulse_width:   p.v(1),
            osc1_detune_coarse: p.v(2),
            osc1_detune_fine:   p.v(3),
            osc1_volume:        p.v(4),
            params:             p,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Oscillator {
    sample_rate: f64,
    phase:      f64,
    integral:   f64,
}

impl Oscillator {
    pub fn new(sample_rate: f64, rg: &mut helpers::RandGen) -> Self {
        Oscillator {
            sample_rate,
            phase:      rg.next_open01() * 2.0 * 3.141592,
            integral:   0.0,
        }
    }

    pub fn next(&mut self, note: f64, waveform: f32, pulse_width: f32) -> f32 {
        let phase_max : f64 =
            self.sample_rate * 0.5 / helpers::note_to_freq(note);
        let dc_offs : f64 = -0.498 / phase_max;

        //d// println!("FO {} {} {}", self.phase, phase_max, pulse_width);
        let mut phase2 : f64 =
            ((self.phase + 2.0 * phase_max * (pulse_width as f64))
             % (phase_max * 2.0)) - phase_max;
        self.phase = (self.phase + 1.0) % (phase_max * 2.0);
        let mut tmp_phase : f64 = self.phase - phase_max;

        let epsilon : f64 = 0.0000001;
        let blit1 : f64 =
            if tmp_phase > epsilon || tmp_phase < -epsilon {
                tmp_phase *= 3.141592;
                //d// println!("IN: {} => {}", tmp_phase,  helpers::fast_sin(tmp_phase));
                helpers::fast_sin(tmp_phase) / tmp_phase
            } else {
                1.0
            };
        let blit2 : f64 =
            if phase2 > epsilon || phase2 < -epsilon {
                phase2 *= 3.141592;
                helpers::fast_sin(phase2) / phase2
            } else {
                1.0
            };

        //d// println!("B1={} B2={}", blit1, blit2);

        self.integral =
            0.998 * self.integral
            + dc_offs * (1.0 - (waveform as f64))
            + blit1
            - blit2 * (waveform as f64);

//        println!("NEX: {}", self.integral);
        self.integral as f32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SlaughterVoice {
    osc1: Oscillator,
    osc2: Oscillator,
    osc3: Oscillator,
    rg: helpers::RandGen,
}

impl SlaughterVoice {
//    pub fn coarse_detune(detune: f32) -> f64 {
//        (self.detune 
//    }

}

impl Voice<SlaughterParams> for SlaughterVoice {
    fn new(sample_rate: f64) -> Self {
        let mut rg = helpers::RandGen::new_with_time();
        SlaughterVoice {
            osc1: Oscillator::new(sample_rate, &mut rg),
            osc2: Oscillator::new(sample_rate, &mut rg),
            osc3: Oscillator::new(sample_rate, &mut rg),
            rg,
        }
    }
    fn note_on(&mut self, data: &mut VoiceData, params: &mut SlaughterParams, note: i32, velocity: i32, detune: f32, pan: f32) {
        println!("SLAUGHTER NOTE ON: {} => {}", note, helpers::note_to_freq(note.into()));
        data.note_on(note, 0, detune, pan);
    }
    fn note_off(&mut self, data: &mut VoiceData, params: &mut SlaughterParams) {
        data.note_off();
        data.is_on = false; // TODO: REMOVE ME IF WE HAVE ENVELOPES!
    }
    fn note_slide(&mut self, data: &mut VoiceData, params: &mut SlaughterParams, slide: f32, note: i32) {
        data.note_slide(slide, note);
    }
    fn get_note(&mut self, data: &mut VoiceData, params: &mut SlaughterParams) -> f64 {
        data.get_note() as f64
    }
    fn run(&mut self,
           data: &mut VoiceData,
           params: &mut SlaughterParams,
           _song_pos: f64,
           sample_num: usize,
           out_offs: usize,
           outputs: &mut [f32]) {

        let base_note : f64 = data.get_note();

//        let mut fi = false;
//        let mut f : f32 = 0.0;
//        let mut l : f32 = 0.0;

        for i in 0..sample_num {
            let s =
                self.osc1.next(
                    base_note, params.osc1_waveform, params.osc1_pulse_width);
//            if !fi { f = s; fi = true; }
//            l = s;
            outputs[out_offs + (i * 2)]     = s as f32;
            outputs[out_offs + (i * 2) + 1] = s as f32;
            //d// println!("S {}", s);
        }
//        println!("OUT[{}]: {} => {}", base_note, f, l);
    }
}

impl Op for SynthDevice<SlaughterVoice, SlaughterParams> {
    fn io_spec(&self, index: usize) -> OpIOSpec {
        OpIOSpec {
            inputs:           self.params.params.ports.clone(),
            input_values:     self.params.params.inputs.clone(),
            input_defaults:   self.params.params.defaults.clone(),
            outputs:          vec![],
            output_regs:      vec![],
            audio_out_groups: vec![],
            index,
        }
    }

    fn event(&mut self, ev: &Event) {
        //d// println!("SLAU EVENT: {:?}", ev);
        match ev {
            Event::NoteOn(n)  => { self.note_on(*n as i32, 0, 0); },
            Event::NoteOff(n) => { self.note_off(*n as i32, 0); },
        }
    }

    fn init_regs(&mut self, _start_reg: usize, _regs: &mut [f32]) { }

    fn get_output_reg(&mut self, _name: &str) -> Option<usize> { None }

    fn set_input(&mut self, name: &str, to: OpIn, as_default: bool) -> bool {
        self.params.params.set(name, to, as_default)
    }

    fn exec(&mut self, t: f32, regs: &mut [f32]) {
        self.params.osc1_waveform      = self.params.params.inputs[0].calc(regs);
        self.params.osc1_pulse_width   = 1.0 - self.params.params.inputs[1].calc(regs);
        self.params.osc1_detune_coarse = self.params.params.inputs[2].calc(regs);
        self.params.osc1_detune_fine   = self.params.params.inputs[3].calc(regs);
        self.params.osc1_volume        = self.params.params.inputs[4].calc(regs);

//        let a = self.values[0].calc(regs);
//        let p = self.values[1].calc(regs);
//        let v = self.values[2].calc(regs);
//        let f = self.values[3].calc(regs);
//        regs[self.out] = a * (((f * t) + p).sin() + v);
        //d// println!("OUT: {}, {}", regs[self.out], self.out);
    }

    fn render(&mut self, num_samples: usize, offs: usize, input_idx: usize, bufs: &mut Vec<Vec<f32>>)
    {
        let mut f : [f32; 1] = [0.0; 1];
        //d// println!("RENDER #samples={}", num_samples);
        self.run(0.0, num_samples, &mut f, &mut bufs[input_idx][..]);
    }
}

pub fn new_slaughter(sample_rate: f64) -> SynthDevice<SlaughterVoice, SlaughterParams> {
    //d// println!("NEW SLAUGHTER!");
    let params = SlaughterParams::new();
    let sd : SynthDevice<SlaughterVoice, SlaughterParams> =
        SynthDevice::new(sample_rate, params);
    sd
}
