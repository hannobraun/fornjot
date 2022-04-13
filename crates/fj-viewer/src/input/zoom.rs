use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use nalgebra::distance;

use crate::camera::{Camera, FocusPoint};

pub struct Zoom {
    events: VecDeque<(Instant, f64)>,

    target_speed: f64,
    current_speed: f64,

    last_direction: Direction,
    idle_since: Option<Instant>,
}

impl Zoom {
    pub fn new(now: Instant) -> Self {
        Self {
            events: VecDeque::new(),

            target_speed: 0.0,
            current_speed: 0.0,

            last_direction: Direction::None,
            idle_since: Some(now),
        }
    }

    /// Push an input delta from the mouse wheel or track pad
    ///
    /// Expects the delta to be normalized, so using the mouse wheel and track
    /// pad lead to the same zoom feel.
    pub fn push_input_delta(&mut self, delta: f64, now: Instant) {
        let new_event = delta * 0.01;

        // If this input is opposite to previous inputs, discard previous inputs
        // to stop ongoing zoom.
        let can_break = match self.idle_since {
            Some(idle_since) => idle_since.elapsed() < BREAK_WINDOW,
            None => true, // ongoing movement; can always break that
        };
        if self.last_direction.is_opposite(&Direction::from(new_event))
            && can_break
        {
            self.events.clear();

            // Make sure that this breaks the zoom instantly.
            self.current_speed = 0.0;

            return;
        }

        self.events.push_back((now, new_event));
    }

    /// Discard zoom events that fall out of the zoom input time window
    ///
    /// See [`INPUT_WINDOW`].
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
    pub fn update_speed(
        &mut self,
        now: Instant,
        delta_t: f64,
        focus_point: FocusPoint,
        camera: &Camera,
    ) {
        self.target_speed = self.events.iter().map(|(_, event)| event).sum();

        // Compute current speed from target speed. Gradually converge towards
        // target speed, but snap to target speed once the difference becomes
        // minuscule.
        let speed_delta = self.target_speed - self.current_speed;
        if speed_delta.abs() >= MIN_SPEED_DELTA {
            let accel = speed_delta.signum() * ACCELERATION;
            self.current_speed += accel * delta_t;

            // By just applying the acceleration, we can overshoot the target,
            // which can cause erratic movement, as we overshoot again every
            // frame.
            //
            // If you need to test this code, you can trigger the problem quite
            // easily by setting acceleration to a really high value.
            if (self.target_speed - self.current_speed).signum()
                != speed_delta.signum()
            {
                // We overshot. Snap to target speed.
                self.current_speed = self.target_speed;
            }
        } else {
            self.current_speed = self.target_speed;
        }

        // Track last zoom direction.
        self.last_direction = Direction::from(self.current_speed);

        // Limit current speed, if close to focus point and zooming in.
        if let Some(focus_point) = focus_point.0 {
            if self.last_direction == Direction::In {
                let d = distance(&focus_point, &camera.position());
                self.current_speed = -f64::min(-self.current_speed, d / 8.);
            }
        }

        // Track idle time
        if self.current_speed == 0.0 {
            if self.idle_since.is_none() {
                self.idle_since = Some(now);
            }
        } else {
            self.idle_since = None
        }
    }

    /// Access the current zoom speed
    pub fn speed(&self) -> f64 {
        self.current_speed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Out,
    In,
    None,
}

impl Direction {
    fn is_opposite(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (Self::Out, Self::In) => true,
            (Self::In, Self::Out) => true,
            _ => false,
        }
    }
}

impl From<f64> for Direction {
    fn from(speed: f64) -> Self {
        if speed > 0.0 {
            return Self::Out;
        }
        if speed < 0.0 {
            return Self::In;
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

/// Time window in which opposite movement is interpreted as breaking
///
/// Defines the time window after a movement ends, during which an input
/// opposite to the movement is interpreted as breaking, not as a new movement.
///
/// Tuning notes:
/// - If this value is too low, zoom input intended to stop the previous
///   movement will instead start a new, opposite movement, leading to jumpy
///   zooming behavior.
/// - If this value is too high, input meant to start a new zoom movement will
///   not be detected, making zoom less controllable.
///
/// This value should be as low as possible, while still preventing jumpy
/// zooming behavior.
const BREAK_WINDOW: Duration = Duration::from_millis(50);

/// The minimum delta between current and target zoom speed
///
/// If the speed delta is below this value, the current zoom speed is snapped to
/// the target zoom speed.
///
/// Tuning notes:
/// - If this value is too low, zoom speed will technically be non-zero, even
///   though no movement is perceivable. This makes detection of last zoom speed
///   and idle time inaccurate, leading to problems.
/// - If this value is too high, zoom acceleration will jump to infinite in that
///   last moment before reaching the target speed, which can seem jarring.
///
/// This value should be as high as possible, allowing for precise detection of
/// last zoom speed an idle time, while not causing jarring accelerations.
const MIN_SPEED_DELTA: f64 = 0.01;

/// Acceleration value for the zoom movement
///
/// Tuning notes:
/// - If this value is too low, target zoom speed will be reached slowly,
///   leading to less precise control.
/// - If this value is too high, zoom movement seems unnatural, which can cause
///   a jarring experience.
///
/// This value should be as high as possible, while not causing jarring
/// accelerations.
const ACCELERATION: f64 = 0.5;
