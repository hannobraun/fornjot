use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub struct Zoom {
    events: VecDeque<(Instant, f32)>,

    target_speed: f32,
    current_speed: f32,

    last_direction: Direction,
}

impl Zoom {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),

            target_speed: 0.0,
            current_speed: 0.0,

            last_direction: Direction::None,
        }
    }

    /// Push an input delta from the mouse wheel or track pad
    ///
    /// Expects the delta to be normalized, so using the mouse wheel and track
    /// pad lead to the same zoom feel.
    pub fn push_input_delta(&mut self, delta: f32, now: Instant) {
        let new_event = delta * 0.01;

        // TASK: If zoom speed was non-zero just a short time ago, don't accept
        //       a zoom event in the opposite direction. A zoom event that's
        //       timed like that, is likely meant as a breaking operation.

        // If this input is opposite to previous inputs, discard previous inputs
        // to stop ongoing zoom.
        if let Some(&(_, event)) = self.events.front() {
            if Direction::from(event).is_opposite(&Direction::from(new_event)) {
                self.events.clear();

                // Make sure that this breaks the zoom instantly.
                self.current_speed = 0.0;

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
            if now.duration_since(*time) > INPUT_WINDOW {
                self.events.pop_front();
                continue;
            }

            break;
        }
    }

    /// Update the zoom speed based on active zoom events
    pub fn update_speed(&mut self) {
        // TASK: Limit zoom speed depending on distance to model surface.
        self.target_speed = self.events.iter().map(|(_, event)| event).sum();

        // Compute current speed from target speed. Gradually move towards
        // target speed, but snap to target speed once the difference becomes
        // minuscule. That latter attribute helps track the last zoom direction.
        let speed_delta = self.target_speed - self.current_speed;
        self.current_speed = if speed_delta.abs() >= 0.01 {
            self.current_speed + speed_delta / 8.
        } else {
            self.target_speed
        };

        // Track last zoom direction.
        self.last_direction = Direction::from(self.current_speed);
    }

    /// Access the current zoom speed
    pub fn speed(&self) -> f32 {
        self.current_speed
    }
}

enum Direction {
    Pos,
    Neg,
    None,
}

impl Direction {
    fn is_opposite(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Pos, Self::Neg) => true,
            (Self::Neg, Self::Pos) => true,
            _ => false,
        }
    }
}

impl From<f32> for Direction {
    fn from(speed: f32) -> Self {
        if speed > 0.0 {
            return Self::Pos;
        }
        if speed < 0.0 {
            return Self::Neg;
        }

        Self::None
    }
}

/// Time window for active zoom events
///
/// This is the time window during which a zoom input event still has an effect
/// on target zoom speed.
///
/// Tuning notes:
/// - If this value is too low, the user can't accumulate many active zooming
///   events, meaning zoom speed can't get very high.
/// - If this value is too high, a single zoom event will have too long of an
///   effect, leading to spongy control behavior.
///
/// This value should be as low as possible, giving the user precise control,
/// while still accommodating high enough zoom speeds.
const INPUT_WINDOW: Duration = Duration::from_millis(500);
