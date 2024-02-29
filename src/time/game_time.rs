use instant::Instant;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameTime {
    total: f64,
    delta: f32,

    cycle: Instant,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            total: 0.0,
            delta: 1.0 / 60.0,

            cycle: Instant::now(),
        }
    }
}

impl GameTime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let diff = now - self.cycle;
        self.cycle = now;

        self.total += diff.as_secs_f64();
        self.delta = diff.as_secs_f32();
    }
}
