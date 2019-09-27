use crate::parameters::*;
use crate::synth_device::*;
use crate::state_variable_filter::*;
use crate::envelope::*;
use crate::helpers::SignalIOParams;
use crate::helpers;
use wctr_signal_ops::signals::{OpIn, Op, OpIOSpec, Event};

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
    dev_params:             SynthDeviceParams,
    params:                 SignalIOParams,
    osc1_waveform:          f32,
    osc1_pulse_width:       f32,
    osc1_volume:            f32,
    osc1_detune_coarse:     f32,
    osc1_detune_fine:       f32,
    osc2_waveform:          f32,
    osc2_pulse_width:       f32,
    osc2_volume:            f32,
    osc2_detune_coarse:     f32,
    osc2_detune_fine:       f32,
    osc3_waveform:          f32,
    osc3_pulse_width:       f32,
    osc3_volume:            f32,
    osc3_detune_coarse:     f32,
    osc3_detune_fine:       f32,
    noise_volume:           f32,
    filter_type:            FilterType,
    filter_freq:            f32,
    filter_resonance:       f32,
    filter_mod_amt:         f32,
    amp_attack:             f32,
    amp_decay:              f32,
    amp_sustain:            f32,
    amp_release:            f32,
    mod_attack:             f32,
    mod_decay:              f32,
    mod_sustain:            f32,
    mod_release:            f32,
    pitch_attack:           f32,
    pitch_decay:            f32,
    pitch_sustain:          f32,
    pitch_release:          f32,
    pitch_env_amt:          f32,
}

impl SlaughterParams {
    pub fn new() -> Self {
        let mut p = SignalIOParams::new();

        // TODO: init SynthDeviceParams

        p.input("o1_vol",     0.0, 1.0, 1.0);
        p.input("o2_vol",     0.0, 1.0, 1.0);
        p.input("o3_vol",     0.0, 1.0, 1.0);
        p.input("nse_vol",    0.0, 1.0, 1.0);
        p.input("o1_wav",     0.0, 1.0, 0.0);
        p.input("o2_wav",     0.0, 1.0, 0.0);
        p.input("o3_wav",     0.0, 1.0, 0.0);
        p.input("o1_pw",      0.0, 1.0, 0.5);
        p.input("o2_pw",      0.0, 1.0, 0.5);
        p.input("o3_pw",      0.0, 1.0, 0.5);
        p.input("o1_detc",    0.0, 1.0, 0.0);
        p.input("o2_detc",    0.0, 1.0, 0.0);
        p.input("o3_detc",    0.0, 1.0, 0.0);
        p.input("o1_detf",    0.0, 1.0, 0.0);
        p.input("o2_detf",    0.0, 1.0, 0.0);
        p.input("o3_detf",    0.0, 1.0, 0.0);
        p.input("f_typ",      0.0, 1.0, 0.0);
        p.input("f_freq",     0.0, 20000.0 - 20.0, 20000.0 - 20.0);
        p.input("f_res",      0.0, 1.0, 1.0);
        p.input("f_mod",      0.0, 1.0, 0.5);
        p.input("amp_a",      0.0, 1.0, 1.0);
        p.input("amp_d",      0.0, 1.0, 5.0);
        p.input("amp_s",      0.0, 1.0, 0.5);
        p.input("amp_r",      0.0, 1.0, 1.5);
        p.input("mod_a",      0.0, 1.0, 1.0);
        p.input("mod_d",      0.0, 1.0, 5.0);
        p.input("mod_s",      0.0, 1.0, 1.0);
        p.input("mod_r",      0.0, 1.0, 1.5);
        p.input("pit_a",      0.0, 1.0, 1.0);
        p.input("pit_d",      0.0, 1.0, 5.0);
        p.input("pit_s",      0.0, 1.0, 0.5);
        p.input("pit_r",      0.0, 1.0, 1.5);
        p.input("pit_eamt",   0.0, 1.0, 0.0);

        SlaughterParams {
            // TODO: Assign SynthDeviceParams!
            osc1_volume:        p.v(0),
            osc2_volume:        p.v(1),
            osc3_volume:        p.v(2),
            noise_volume:       p.v(3),
            osc1_waveform:      p.v(4),
            osc2_waveform:      p.v(5),
            osc3_waveform:      p.v(6),
            osc1_pulse_width:   p.v(7),
            osc2_pulse_width:   p.v(8),
            osc3_pulse_width:   p.v(9),
            osc1_detune_coarse: p.v(10),
            osc2_detune_coarse: p.v(11),
            osc3_detune_coarse: p.v(12),
            osc1_detune_fine:   p.v(13),
            osc2_detune_fine:   p.v(14),
            osc3_detune_fine:   p.v(15),
            filter_type:        p.v(16).into(),
            filter_freq:        p.v(17),
            filter_resonance:   p.v(18),
            filter_mod_amt:     p.v(19),
            amp_attack:         p.v(20),
            amp_decay:          p.v(21),
            amp_sustain:        p.v(22),
            amp_release:        p.v(23),
            mod_attack:         p.v(24),
            mod_decay:          p.v(25),
            mod_sustain:        p.v(26),
            mod_release:        p.v(27),
            pitch_attack:       p.v(28),
            pitch_decay:        p.v(29),
            pitch_sustain:      p.v(30),
            pitch_release:      p.v(31),
            pitch_env_amt:      p.v(32),

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
    sample_rate: f64,
    osc1:       Oscillator,
    osc2:       Oscillator,
    osc3:       Oscillator,
    filter:     Filter,
    amp_env:    Envelope,
    mod_env:    Envelope,
    pitch_env:  Envelope,
    rg:         helpers::RandGen,
}

impl SlaughterVoice {
    pub fn coarse_detune(&self, detune: f32) -> f64 {
        (detune as f64 * 24.99).floor()
    }
}

impl Voice<SlaughterParams> for SlaughterVoice {
    fn new(sample_rate: f64) -> Self {
        let mut rg = helpers::RandGen::new_with_time();
        SlaughterVoice {
            sample_rate,
            osc1:       Oscillator::new(sample_rate, &mut rg),
            osc2:       Oscillator::new(sample_rate, &mut rg),
            osc3:       Oscillator::new(sample_rate, &mut rg),
            filter:     Filter::new(sample_rate),
            amp_env:    Envelope::new(sample_rate),
            mod_env:    Envelope::new(sample_rate),
            pitch_env:  Envelope::new(sample_rate),
            rg,
        }
    }
    fn note_on(&mut self, data: &mut VoiceData, params: &mut SlaughterParams, note: i32, velocity: i32, detune: f32, pan: f32) {
        println!("SLAUGHTER NOTE ON: {} => {}", note, helpers::note_to_freq(note.into()));
        data.note_on(note, 0, detune, pan);

        self.amp_env.attack     = params.amp_attack;
        self.amp_env.decay      = params.amp_decay;
        self.amp_env.sustain    = params.amp_sustain;
        self.amp_env.release    = params.amp_release;
        self.amp_env.trigger();

        self.mod_env.attack     = params.mod_attack;
        self.mod_env.decay      = params.mod_decay;
        self.mod_env.sustain    = params.mod_sustain;
        self.mod_env.release    = params.mod_release;
        self.mod_env.trigger();

        self.pitch_env.attack     = params.pitch_attack;
        self.pitch_env.decay      = params.pitch_decay;
        self.pitch_env.sustain    = params.pitch_sustain;
        self.pitch_env.release    = params.pitch_release;
        self.pitch_env.trigger();
    }
    fn note_off(&mut self, data: &mut VoiceData, params: &mut SlaughterParams) {
        data.note_off();
        data.is_on = false; // TODO: REMOVE ME IF WE HAVE ENVELOPES!
        self.amp_env.off();
        self.mod_env.off();
        self.pitch_env.off();
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

        //let vibrato_freq = data.vibrato_freq / self.sample_rate;

        self.filter.set_type(params.filter_type);
        self.filter.set_q(params.filter_resonance);

        let amp       = -16.0 * helpers::volume_to_scalar(params.master_level);
        let pan_left  = helpers::pan_to_scalar_left(data.pan);
        let pan_right = helpers::pan_to_scalar_right(data.pan);

        let osc1_detune = self.coarse_detune(params.osc1_detune_coarse)
                          + (params.osc1_detune_fine as f64);
        let osc2_detune = self.coarse_detune(params.osc2_detune_coarse)
                          + (params.osc2_detune_fine as f64);
        let osc3_detune = self.coarse_detune(params.osc3_detune_coarse)
                          + (params.osc3_detune_fine as f64);

        let osc1_volume_scalar = params.osc1_volume * params.osc1_volume;
        let osc2_volume_scalar = params.osc2_volume * params.osc2_volume;
        let osc3_volume_scalar = params.osc3_volume * params.osc3_volume;
        let noise_scalar       = params.noise_volume * params.noise_volume;

//        let mut fi = false;
//        let mut f : f32 = 0.0;
//        let mut l : f32 = 0.0;

        for i in 0..sample_num {
            self.filter.set_freq(
                helpers::clamp(
                    params.filter_freq, 0.0, 20000.0 - 20.0));
            let mut s =
                self.osc1.next(
                    base_note, params.osc1_waveform, params.osc1_pulse_width);

//            if !fi { f = s; fi = true; }
//            l = s;

            let osc_mix = s;
            s = self.filter.next(osc_mix);
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
        self.params.osc1_volume        = self.params.params.inputs[0].calc(regs);
        self.params.osc2_volume        = self.params.params.inputs[1].calc(regs);
        self.params.osc3_volume        = self.params.params.inputs[2].calc(regs);
        self.params.noise_volume       = self.params.params.inputs[3].calc(regs);
        self.params.osc1_waveform      = self.params.params.inputs[4].calc(regs);
        self.params.osc2_waveform      = self.params.params.inputs[5].calc(regs);
        self.params.osc3_waveform      = self.params.params.inputs[6].calc(regs);
        self.params.osc1_pulse_width   = 1.0 - self.params.params.inputs[7].calc(regs);
        self.params.osc2_pulse_width   = 1.0 - self.params.params.inputs[8].calc(regs);
        self.params.osc3_pulse_width   = 1.0 - self.params.params.inputs[9].calc(regs);
        self.params.osc1_detune_coarse = self.params.params.inputs[10].calc(regs);
        self.params.osc2_detune_coarse = self.params.params.inputs[11].calc(regs);
        self.params.osc3_detune_coarse = self.params.params.inputs[12].calc(regs);
        self.params.osc1_detune_fine   = self.params.params.inputs[13].calc(regs);
        self.params.osc2_detune_fine   = self.params.params.inputs[14].calc(regs);
        self.params.osc3_detune_fine   = self.params.params.inputs[15].calc(regs);
        self.params.filter_type        = self.params.params.inputs[16].calc(regs).into();
        self.params.filter_freq        =
            helpers::param_to_frequency(self.params.params.inputs[17].calc(regs));
        self.params.filter_resonance   = 1.0 - self.params.params.inputs[18].calc(regs);
        self.params.filter_mod_amt     = self.params.params.inputs[19].calc(regs);
        self.params.amp_attack         =
            helpers::scalar_to_env_value(self.params.params.inputs[20].calc(regs));
        self.params.amp_decay          =
            helpers::scalar_to_env_value(self.params.params.inputs[21].calc(regs));
        self.params.amp_sustain        = self.params.params.inputs[22].calc(regs);
        self.params.amp_release        =
            helpers::scalar_to_env_value(self.params.params.inputs[23].calc(regs));
        self.params.mod_attack         =
            helpers::scalar_to_env_value(self.params.params.inputs[24].calc(regs));
        self.params.mod_decay          =
            helpers::scalar_to_env_value(self.params.params.inputs[25].calc(regs));
        self.params.mod_sustain        = self.params.params.inputs[26].calc(regs);
        self.params.mod_release        =
            helpers::scalar_to_env_value(self.params.params.inputs[27].calc(regs));
        self.params.pitch_attack       =
            helpers::scalar_to_env_value(self.params.params.inputs[28].calc(regs));
        self.params.pitch_decay        =
            helpers::scalar_to_env_value(self.params.params.inputs[29].calc(regs));
        self.params.pitch_sustain      = self.params.params.inputs[30].calc(regs);
        self.params.pitch_release      =
            helpers::scalar_to_env_value(self.params.params.inputs[31].calc(regs));
        self.params.pitch_env_amt      =
            (self.params.params.inputs[32].calc(regs) - 0.5) * 2.0 * 36.0;

        // TODO: init and copy SynthDeviceParams to SynthDevice here!
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
