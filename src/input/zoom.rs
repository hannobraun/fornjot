use std::{collections::VecDeque, time::Instant};

pub struct Zoom {
    pub events: VecDeque<(Instant, f32)>,
    pub zoom_speed: f32,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            zoom_speed: 0.0,
        }
    }
}
