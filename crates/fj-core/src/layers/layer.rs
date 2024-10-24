use std::ops::Deref;

/// A generic layer, which controls access to layer state
///
/// `Layer` is a generic wrapper around some state and controls access to it. It
/// [`Deref`]s to the state it wraps, for easy read access, but prevents any
/// direct write access.
///
/// Instead, each write access to state is reified as a command, which are
/// processed by [`Layer::process_command`]. Processing a command can result in
/// any number of events, which can then be used as commands for other layers.
///
/// This design takes inspiration from, and uses the nomenclature of, this
/// article:
/// <https://thinkbeforecoding.com/post/2021/12/17/functional-event-sourcing-decider>
pub struct Layer<S> {
    state: S,
}

impl<S> Layer<S> {
    /// Create an instance of `Layer`
    pub fn new(state: S) -> Self {
        Self { state }
    }

    /// # Process a command without capturing any events
    ///
    /// The command is processed synchronously. When this method returns, the
    /// state has been updated.
    pub fn process_command<C>(&mut self, command: C) -> C::Result
    where
        C: Command<S>,
    {
        let mut events = Vec::new();
        let result = command.decide(&self.state, &mut events);

        for event in events {
            event.evolve(&mut self.state);
        }

        result
    }

    /// # Process a command and capture the events that produces
    ///
    /// The command is processed synchronously. When this method returns, the
    /// state has been updated.
    pub fn process_command_and_capture_events<C>(
        &mut self,
        command: C,
        events: &mut Vec<C::Event>,
    ) -> C::Result
    where
        C: Command<S>,
        C::Event: Clone,
    {
        let result = command.decide(&self.state, events);

        for event in events.iter().cloned() {
            event.evolve(&mut self.state);
        }

        result
    }

    /// Drop this instance, returning the wrapped state
    pub fn into_state(self) -> S {
        self.state
    }
}

impl<S> Deref for Layer<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S> Default for Layer<S>
where
    S: Default,
{
    fn default() -> Self {
        Self::new(S::default())
    }
}

/// A command that encodes a request to update a layer's state
pub trait Command<S> {
    /// The direct result of processing a command that is returned to the caller
    ///
    /// Changes to the state that result from a command are encoded as events
    /// (see [`Command::Event`]). In addition to that, a command may return
    /// information to the caller, and `Result` defines the type of that
    /// information.
    type Result;

    /// An event that encodes a change to the state
    ///
    /// Events are produced by [`Command::decide`] and processed by
    /// [`Event::evolve`].
    type Event: Event<S>;

    /// Decide which events to produce, given the command and provided state
    ///
    /// If the command must result in changes to the state, any number of events
    /// that describe these state changes can be produced.
    fn decide(self, state: &S, events: &mut Vec<Self::Event>) -> Self::Result;
}

/// An event that encodes a change to a layer's state
pub trait Event<S> {
    /// Evolve the provided state
    ///
    /// This is the only method that [`Layer`] gives mutable access to the
    /// state, making sure that all changes to the state are captured as events.
    ///
    /// Implementations of this method are supposed to be relatively dumb. Any
    /// decisions that go into updating the state should be made in
    /// [`Command::decide`], and encoded into the event.
    fn evolve(self, state: &mut S);
}
