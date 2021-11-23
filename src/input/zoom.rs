use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

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

    /// Push an input delta from the mouse wheel or track pad
    ///
    /// Expects the delta to be normalized, so using the mouse wheel and track
    /// pad lead to the same zoom feel.
    pub fn push_input_delta(&mut self, delta: f32, now: Instant) {
        let new_event = delta * 0.1;

        // If this input is opposite to previous inputs, discard previous inputs
        // to stop ongoing zoom.
        if let Some((_, event)) = self.events.front() {
            if event.signum() != new_event.signum() {
                self.events.clear();
                return;
            }
        }

        self.events.push_back((now, new_event));
    }

    /// Discard zoom events that fall out of the zoom input time window
    pub fn discard_old_events(&mut self, now: Instant) {
        const ZOOM_INPUT_WINDOW: Duration = Duration::from_millis(500);
        while let Some((time, _)) = self.events.front() {
            if now.duration_since(*time) > ZOOM_INPUT_WINDOW {
                self.events.pop_front();
                continue;
            }

            break;
        }
    }
}
