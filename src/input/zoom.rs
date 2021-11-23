use std::{collections::VecDeque, time::Instant};

pub struct Zoom {
    pub events: VecDeque<(Instant, f32)>,
    pub speed: f32,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            speed: 0.0,
        }
    }
}
