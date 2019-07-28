static FAST_COS_TAB_LOG2_SIZE : usize = 10;
static FAST_COS_TAB_SIZE : usize      = 1 << FAST_COS_TAB_LOG2_SIZE; // =1024
static mut FAST_COS_TAB : [f64; 1024] = [0.0; 1024];

pub fn init_cos_tab() {
    for i in 0..FAST_COS_TAB_SIZE {
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
    fast_cos(x - std::f64::consts::PI)
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
    let phase_scale  = 1.0 / (std::f64::consts::PI * 2.0);
    let phase        = 1.0 + x * phase_scale;

    let phase_as_u64 = phase.to_bits();
    let exponent     = (phase_as_u64 >> 52) - 1023;

    let fract_bits   = 32 - FAST_COS_TAB_LOG2_SIZE;
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
        let right        = FAST_COS_TAB[(index as usize + 1) % 1024];
        let fract_mix    = (fract as f64) * (1.0 / (fract_scale as f64));

        return left + (right - left) * fract_mix;
    }
}
