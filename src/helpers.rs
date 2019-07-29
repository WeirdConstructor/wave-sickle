#![macro_use]

static FAST_COS_TAB_LOG2_SIZE : usize = 9;
static FAST_COS_TAB_SIZE : usize      = 1 << FAST_COS_TAB_LOG2_SIZE; // =512
static mut FAST_COS_TAB : [f64; 513] = [0.0; 513];

pub fn init_cos_tab() {
    for i in 0..(FAST_COS_TAB_SIZE+1) {
        let phase : f64 =
            (i as f64)
            * ((std::f64::consts::PI * 2.0)
               / (FAST_COS_TAB_SIZE as f64));
        unsafe {
            // XXX: note: mutable statics can be mutated by multiple
            //      threads: aliasing violations or data races
            //      will cause undefined behavior
            FAST_COS_TAB[i] = phase.cos();
        }
    }
}

pub fn fast_sin(x: f64) -> f64 {
    fast_cos(x - (std::f64::consts::PI / 2.0))
}

pub fn square_135(phase: f64) -> f64 {
      fast_sin(phase)
    + fast_sin(phase * 3.0) / 3.0
    + fast_sin(phase * 5.0) / 5.0
}

pub fn square_35(phase: f64) -> f64 {
      fast_sin(phase * 3.0) / 3.0
    + fast_sin(phase * 5.0) / 5.0
}

pub fn mix(v1: f32, v2: f32, mix: f32) -> f32 {
    v1 * (1.0 - mix) + v2 * mix
}

pub fn clamp(f: f32, min: f32, max: f32) -> f32 {
         if f < min { min }
    else if f > max { max }
    else            {   f }
}

pub fn pow(x: f64, y: f64)  -> f64 { x.powf(y) }
pub fn powf(x: f32, y: f32) -> f32 { x.powf(y) }

pub fn fast_cos(mut x: f64) -> f64 {
    x = x.abs(); // cosine is symmetrical around 0, let's get rid of negative values

    // normalize range from 0..2PI to 1..2
    let phase_scale  = 1.0_f64 / (std::f64::consts::PI * 2.0_f64);
    let phase        = 1.0_f64 + x * phase_scale;

    let phase_as_u64 : u64 = unsafe { std::mem::transmute::<f64, u64>(phase) };//  phase.to_bits();
    let exponent     = (phase_as_u64 >> 52) - 1023;

    let fract_bits : u32  = 32 - FAST_COS_TAB_LOG2_SIZE as u32;
    let fract_scale  = 1 << fract_bits;
    let fract_mask   = fract_scale - 1;


    let significand  = ((phase_as_u64 << exponent) >> (52 - 32)) as u32;
    let index        = significand >> fract_bits;
    let fract : i32  = (significand as i32) & fract_mask;

    unsafe {
        // XXX: note: mutable statics can be mutated by multiple
        //      threads: aliasing violations or data races
        //      will cause undefined behavior
        let left         = FAST_COS_TAB[index as usize];
        let right        = FAST_COS_TAB[index as usize + 1];
        let fract_mix    = (fract as f64) * (1.0 / (fract_scale as f64));

        return left + (right - left) * fract_mix;
    }
}

fn note_to_freq(note: f64) -> f64 {
    440.0 * pow(2.0, (note - 69.0) / 12.0)
}

fn db_to_scalar(db: f32) -> f32 {
    powf(2.0, db / 6.0)
}

fn env_value_to_scalar(value: f32) -> f32 {
    (value - 1.0).sqrt() / 5000.0
}

fn scalar_to_env_value(scalar: f32) -> f32 {
    scalar * scalar * 5000.0 + 1.0
}

fn volume_to_scalar(volume: f32) -> f32 {
    let v = volume * 0.4;
    v * v
}

fn scalar_to_volume(scalar: f32) -> f32 {
    scalar.sqrt() / 0.4
}

fn param_to_boolean(value: f32) -> bool { value >= 0.5 }
fn boolean_to_param(b: bool)    -> f32  { if b { 1.0 } else { 0.0 } }

fn param_to_frequency(param: f32) -> f32 {
    20.0 + (20000.0 - 20.0) * param * param
}

fn frequency_to_param(freq: f32) -> f32 {
    ((freq - 20.0) / (20000.0 - 20.0)).sqrt()
}

fn param_to_q(param: f32) -> f32 {
    if param < 0.5 {
        param / 0.5 * (1.0 - 0.33) + 0.33
    } else {
        (param - 0.5) / 0.5 * 11.0 + 1.0
    }
}

fn q_to_param(q: f32) -> f32 {
    if q < 1.0 {
        (q - 0.33) / (1.0 - 0.33) * 0.5
    } else {
        (q - 1.0) / 11.0 * 0.5 + 0.5
    }
}

fn param_to_db(param: f32, range: f32) -> f32 {
    (param * 2.0 - 1.0) * range
}

fn db_to_param(db: f32, range: f32) -> f32 {
    (db / range + 1.0) / 2.0
}

fn param_to_resonance(param: f32) -> f32 {
    param * 0.99 + 0.01
}

fn resonance_to_param(resonance: f32) -> f32 {
    (resonance - 0.01) / 0.99
}

fn param_to_unisono(param: f32) -> i32 {
    (param * 15.0) as i32 + 1
}

fn unisono_to_param(unisono: i32) -> f32 {
    (unisono - 1) as f32 / 15.0
}

fn param_to_vibrato_freq(param: f32) -> f64 {
    (pow(param as f64, 2.0) + 0.1) * 70.0
}

fn vibrato_freq_to_param(vf: f64) -> f32 {
    let d = vf / 70.0 - 0.1;
    if d >= 0.0 { d.sqrt() as f32 } else { 0.0 }
}

fn pan_to_scalar_left(pan: f32) -> f32 {
    (1.0 - pan).sqrt()
}

fn pan_to_scalar_right(pan: f32) -> f32 {
    pan.sqrt()
}

#[macro_export]
macro_rules! recalc_setter {
    ($fun_name: ident, $g: ident, $typ: ident) => {
        pub fn $fun_name(&mut self, v: $typ) {
            if self.$g == v { return; }
            self.$g = v;
            self.recalculate = true;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sin_cos() {
        init_cos_tab();

//        assert_eq!(fast_sin(std::f64::consts::PI), -1.0);
//        assert_eq!(fast_sin(std::f64::consts::PI * 2.0), 1.0);
//        assert_eq!(fast_cos(std::f64::consts::PI), -1.0);
//        assert_eq!(fast_cos(std::f64::consts::PI * 2.0), 1.0);
    }
}
