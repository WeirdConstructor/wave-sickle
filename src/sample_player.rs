use crate::helpers;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InterpolationMode {
    Nearest,
    Linear,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LoopMode {
    Disabled,
    Repeat,
    PingPong,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LoopBoundaryMode {
    FromSample,
    Manual,
}

#[derive(Debug, PartialEq, Clone)]
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
    pub sample_loop_start:  i32,
    pub sample_loop_length: i32,
        sample_pos:         f64,
        sample_delta:       f64,
        rounded_loop_start: i32,
        rounded_loop_len:   i32,
        rounded_loop_end:   i32,
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
            rounded_loop_start: 0,
            rounded_loop_len:   0,
            rounded_loop_end:   0,
            sample_delta:       0.0,
            sample_pos:         0.0,
            sample_start:       0.0,
            reverse_:           false,
            interpolation_mode: InterpolationMode::Linear,
            sample_data:        Vec::new(),
            is_active:          false,
        }
    }

    pub fn calc_pitch(&mut self, note: f64) {
        let freq_delta = helpers::pow(2.0, note / 12.0);
        self.sample_delta =
            if !self.reverse_ { freq_delta } else { -freq_delta };
    }

    pub fn init_pos(&mut self) {
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

    pub fn run_prep(&mut self) {
        match self.loop_boundary_mode {
            LoopBoundaryMode::FromSample => {
                self.rounded_loop_start = self.sample_loop_start;
                self.rounded_loop_len   = self.sample_loop_length;
            },
            LoopBoundaryMode::Manual => {
                self.rounded_loop_start =
                    (self.sample_data.len() as f32 * self.loop_start) as i32;
                self.rounded_loop_len =
                    (self.sample_data.len() as f32 * self.loop_length) as i32;
            },
        }

        if self.rounded_loop_len < 1 {
            self.rounded_loop_len = 1;
        }

        if self.rounded_loop_start >= self.sample_data.len() as i32 {
            self.rounded_loop_start = self.sample_data.len() as i32 - 1;
        }

        if self.rounded_loop_start < 0 {
            self.rounded_loop_start = 0;
        }

        self.rounded_loop_end = self.rounded_loop_start + self.rounded_loop_len;

        if self.rounded_loop_end > self.sample_data.len() as i32 {
            self.rounded_loop_end = self.sample_data.len() as i32;
            self.rounded_loop_len = self.rounded_loop_end - self.rounded_loop_start;
        }
    }

    pub fn next(&mut self) -> f32 {
        let sample_len = self.sample_data.len() as i32;
        let sample_pos_floor = self.sample_pos.floor();
        let sample_pos_fract = self.sample_pos - sample_pos_floor;

        let rounded_sample_pos = sample_pos_floor as i32;
        if rounded_sample_pos < 0 || rounded_sample_pos >= sample_len {
            self.is_active = false;
            return 0.0;
        }

        let mut sample : f32 = 
            match self.interpolation_mode {
                InterpolationMode::Nearest => {
                    self.sample_data[rounded_sample_pos as usize]
                },
                InterpolationMode::Linear => {
                    let left = self.sample_data[rounded_sample_pos as usize];
                    let right_index =
                        if self.loop_mode == LoopMode::Repeat
                           && (rounded_sample_pos + 1) == self.rounded_loop_end {
                            self.rounded_loop_start
                        } else {
                            rounded_sample_pos + 1
                        };
                    let right =
                        if right_index < sample_len {
                            self.sample_data[right_index as usize]
                        } else {
                            0.0
                        };

                    (   left as f64 * (1.0 - sample_pos_fract)
                     + right as f64 * sample_pos_fract)
                    as f32
                },
            };

        self.sample_pos += self.sample_delta;

        match self.loop_mode {
            LoopMode::Repeat => {
                if self.sample_delta > 0.0 {
                    while self.sample_pos >= self.rounded_loop_end as f64 {
                        self.sample_pos -= self.rounded_loop_len as f64;
                    }
                } else {
                    while self.sample_pos < self.rounded_loop_end as f64 {
                        self.sample_pos += self.rounded_loop_len as f64;
                    }
                }
            },
            LoopMode::PingPong => {
                self.sample_pos =
                    if self.sample_delta > 0.0
                       && self.sample_pos >= self.rounded_loop_end as f64 {

                        (self.rounded_loop_end - 1) as f64
                    } else {
                        self.rounded_loop_start as f64
                    };
                self.sample_delta = -self.sample_delta;
                self.reverse_ = !self.reverse_;
            },
            LoopMode::Disabled => (),
        }

        sample
     }
}
