use std::ops::Deref;

/// A service that controls access to some state
///
/// `Service` is a generic wrapper around some state, as well as code that knows
/// how to operate on that state. It processes commands, changes the state based
/// on those command, and produces events that capture these changes. These
/// events are stored, providing a log of all changes to the state, and can be
/// replayed later to re-create the state at any point in time.
///
/// The wrapped state must implement [`State`], which defines the type of
/// command that this service processes, and the type of event that captures
/// state changes. It also defines methods that operate on the state, commands,
/// and events.
///
/// Implementations of [`State`] might also define an extension trait for a
/// specific `Service<MyState>`, to provide a convenient API to callers.
///
/// This design takes inspiration from, and uses the nomenclature of, this
/// article:
/// <https://thinkbeforecoding.com/post/2021/12/17/functional-event-sourcing-decider>
pub struct Layer<S: State> {
    state: S,
}

impl<S: State> Layer<S> {
    /// Create an instance of `Service`
    pub fn new(state: S) -> Self {
        Self { state }
    }

    /// Execute a command
    ///
    /// The command is executed synchronously. When this method returns, the
    /// state has been updated and any events have been logged.
    pub fn process(&mut self, command: S::Command, events: &mut Vec<S::Event>) {
        self.state.decide(command, events);

        for event in events {
            self.state.evolve(event);
        }
    }

    /// Drop this instance, returning the wrapped state
    pub fn into_state(self) -> S {
        self.state
    }
}

impl<S: State> Deref for Layer<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S: State> Default for Layer<S>
where
    S: Default,
{
    fn default() -> Self {
        Self::new(S::default())
    }
}

/// Implemented for state that can be wrapped by a [`Layer`]
///
/// See [`Layer`] for a detailed explanation.
pub trait State {
    /// A command that relates to the state
    ///
    /// Commands are processed by [`State::decide`].
    type Command;

    /// An event that captures modifications to this state
    ///
    /// Events are produced by [`State::decide`] and processed by
    /// [`State::evolve`].
    type Event;

    /// Decide how to react to the provided command
    ///
    /// If the command must result in changes to the state, any number of events
    /// that describe these state changes can be produced.
    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>);

    /// Evolve the state according to the provided event
    ///
    /// This is the only method gets mutable access to the state, making sure
    /// that all changes to the state are captured as events.
    ///
    /// Implementations of this method are supposed to be relatively dumb. Any
    /// decisions that go into updating the state should be made in
    /// [`State::decide`], and encoded into the event.
    fn evolve(&mut self, event: &Self::Event);
}
