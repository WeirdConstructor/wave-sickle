pub struct AllPass {
    feedback:   f32,
    buffer:     Vec<f32>,
    buffer_idx: usize,
}

impl AllPass {
    pub fn new() -> Self {
        AllPass {
            feedback: 0.0,
            buffer: vec![0.0],
            buffer_idx: 0,
        }
    }

    pub fn set_feedback(&mut self, fb: f32) {
        self.feedback = fb;
    }

    pub fn get_feedback(&self) -> f32 {
        self.feedback
    }

    pub fn set_buffer_size(&mut self, mut size: usize) {
        if size < 1 { size = 1; }
        self.buffer.clear();
        self.buffer.resize(size, 0.0);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let buffer_out = self.buffer[self.buffer_idx];
        self.buffer[self.buffer_idx] = input + (buffer_out * self.feedback);
        self.buffer_idx = (self.buffer_idx + 1) % self.buffer.len();

        -input + buffer_out
    }
}
