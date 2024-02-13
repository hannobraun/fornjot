use std::ops::Deref;

/// A generic layer, which controls access to layer state
///
/// `Layer` is a generic wrapper around some state and controls access to it. It
/// [`Deref`]s to the state it wraps, for easy read access, but prevents any
/// direct write access.
///
/// Instead, each write access to state is reified as a command, which are
/// processed by [`Layer::process`]. Processing a command can result in any
/// number of events, which can then be used as commands for other layers.
///
/// All of this is mediated through [`State`], which the wrapped state must
/// implement.
///
/// This design takes inspiration from, and uses the nomenclature of, this
/// article:
/// <https://thinkbeforecoding.com/post/2021/12/17/functional-event-sourcing-decider>
pub struct Layer<S: State> {
    state: S,
}

impl<S: State> Layer<S> {
    /// Create an instance of `Layer`
    pub fn new(state: S) -> Self {
        Self { state }
    }

    /// Process a command
    ///
    /// The command is processed synchronously. When this method returns, the
    /// state has been updated.
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

/// The state of a specific layer
///
/// Implementations of this trait are wrapped by the generic [`Layer`], which is
/// the consumer of this trait's API.
///
/// See [`Layer`] for a more detailed explanation.
pub trait State {
    /// A command that encodes a request to update the state
    ///
    /// Commands are processed by [`State::decide`].
    type Command;

    /// An event that encodes a change to the state
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
    /// This is the only method that gets mutable access to the state, making
    /// sure that all changes to the state are captured as events.
    ///
    /// Implementations of this method are supposed to be relatively dumb. Any
    /// decisions that go into updating the state should be made in
    /// [`State::decide`], and encoded into the event.
    fn evolve(&mut self, event: &Self::Event);
}
