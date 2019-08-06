pub struct AllPassDelay {
    a1: f32,
    zm1: f32,
}

impl AllPassDelay {
    pub fn new() -> Self {
        AllPassDelay {
            a1: 0.0,
            zm1: 0.0,
        }
    }

    pub fn delay(&mut self, delay: f32) {
        self.a1 = (1.0 - delay) / (1.0 + delay);
    }

    pub fn update(&mut self, in_sample: f32) -> f32 {
        let y = in_sample * -self.a1 + self.zm1;
        self.zm1 = y * self.a1 + in_sample;
        y
    }
}
