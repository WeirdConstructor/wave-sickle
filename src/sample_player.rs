pub enum InterpolationMode {
    Nearest,
    Linear,
}

pub enum LoopMode {
    Disabled,
    Repeat,
    PingPong,
}

pub enum LoopBoundaryMode {
    FromSample,
    Manual,
}

struct SamplePlayer {
    pub sample_rate:        f64,
    pub is_active:          bool,
    pub sample_start:       f32,
    pub reverse:            bool,
    pub loop_mode:          LoopMode,
    pub loop_boundary_mode: LoopBoundaryMode,
    pub loop_start:         f32,
    pub loop_length:        f32,
    pub interpolation_mode: InterpolationMode,
    pub sample_data:        Vec<f32>,
    pub sample_loop_start:  usize,
    pub sample_loop_length: usize,
        sample_pos:         f64,
        sample_delta:       f64,
        rounded_loop_start: usize,
        rounded_loop_len:   usize,
        rounded_loop_end:   usize,
        reverse_:           bool,
}

impl SamplePlayer {
    pub fn new(sample_rate: f64) -> Self {
        SamplePlayer {
            sample_rate,
            reverse:            false,
            loop_mode:          LoopMode::Repeat,
            loop_boundary_mode: LoopBoundaryMode::FromSample,
            loop_start:         0.0,
            loop_length:        1.0,
            sample_loop_start:  0,
            sample_loop_length: 0,
            loop_start:         0,
            loop_length:        0,
            interpolation_mode: InterpolationMode::Linear,
            sample_data:        Vec::new(),
            is_active:          false,
        }
    }

    pub fn calc_pitch(note: f64) {
        let freq_delta = helpers::pow(2.0, (note / 12.0));
        self.sample_delta =
            if !self.reverse_ { freq_delta } else { -freq_delta };
    }

    pub fn init_pos() {
        self.reverse_ = self.reverse;
        self.is_active = true;
        self.sample_pos =
            if !self.reverse_ {
                self.sample_start as f64
                * ((self.sample_data.len() - 1) as f64)
            } else {
                (1.0 - self.sample_start as f64)
                * ((self.sample_data.len() - 1) as f64)
            };
    }
}
