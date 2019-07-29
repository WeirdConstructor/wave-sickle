#[derive(Debug, PartialEq, Clone, Copy)]
enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Envelope {
    pub state:         EnvelopeState,
    pub attack:        f32,
    pub decay:         f32,
    pub sustain:       f32,
    pub release:       f32,
        sample_rate:   f64,
        pos:           f32,
        release_value: f32,
}

impl Envelope {
    pub fn new(sample_rate: f64) -> Self {
        Envelope {
            sample_rate,
            state:         EnvelopeState::Finished,
            attack:        1.0,
            decay:         5.0,
            sustain:       0.5,
            release:       1.5,
            pos:           0.0,
            release_value: 0.0,
        }
    }

    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
        self.pos   = 0.0;
    }

    pub fn off(&mut self) {
        self.release_value = self.get_value();
        self.state         = EnvelopeState::Release;
        self.pos           = 0.0;
    }

    pub fn get_value(&self) -> f32 {
        match self.state {
            EnvelopeState::Attack => self.pos / self.attack,
            EnvelopeState::Decay => {
                let mut f : f32 = 1.0 - self.pos / self.decay;
                f *= f;
                1.0 * f + self.sustain * (1.0 - f)
            },
            EnvelopeState::Sustain => self.sustain,
            EnvelopeState::Release => {
                let mut f = 1.0 - self.pos / self.release;
                f *= f;
                self.release_value * f
            },
            EnvelopeState::Finished => 0.0,
        }
    }

    pub fn next(&mut self) {
        let pos_delta = (1000.0 / self.sample_rate) as f32;
        match self.state {
            EnvelopeState::Attack => {
                self.pos += pos_delta;
                if self.pos >= self.attack {
                    self.state = EnvelopeState::Decay;
                    self.pos -= self.attack;
                }
            },
            EnvelopeState::Decay => {
                self.pos += pos_delta;
                if self.pos >= self.decay {
                    self.state = EnvelopeState::Sustain;
                }
            },
            EnvelopeState::Release => {
                self.pos += pos_delta;
                if self.pos >= self.release {
                    self.state = EnvelopeState::Finished;
                }
            },
            EnvelopeState::Sustain => (),
            EnvelopeState::Finished => (),
        }
    }
}
