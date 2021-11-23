use std::{collections::VecDeque, time::Instant};

pub struct Zoom {
    pub zoom_events: VecDeque<(Instant, f32)>,
    pub zoom_speed: f32,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            zoom_events: VecDeque::new(),
            zoom_speed: 0.0,
        }
    }
}
