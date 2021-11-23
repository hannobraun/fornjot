use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub struct Zoom {
    events: VecDeque<(Instant, f32)>,
    target_speed: f32,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            target_speed: 0.0,
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
    ///
    /// See [`ZOOM_INPUT_WINDOW`].
    pub fn discard_old_events(&mut self, now: Instant) {
        while let Some((time, _)) = self.events.front() {
            if now.duration_since(*time) > ZOOM_INPUT_WINDOW {
                self.events.pop_front();
                continue;
            }

            break;
        }
    }

    /// Update the zoom speed based on active zoom events
    pub fn update_speed(&mut self) {
        // TASK: Limit zoom speed depending on distance to model surface.
        // TASK: Reduce zoom speed gradually, don't kill it instantly. It seems
        //       jarring.
        self.target_speed = self.events.iter().map(|(_, event)| event).sum();
    }

    /// Access the current zoom speed
    pub fn speed(&self) -> f32 {
        self.target_speed
    }
}

const ZOOM_INPUT_WINDOW: Duration = Duration::from_millis(500);
