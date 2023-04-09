//! This module contains utilities for implementing and working with state machines.
//!

use std::collections::HashMap;
use std::hash::Hash;

pub trait SMState: PartialEq + Eq + Clone + Hash {
    fn is_final(&self) -> bool;
    fn is_initial(&self) -> bool;
    fn to_string(&self) -> String;
}

pub trait SMEvent: PartialEq + Eq + Clone + Hash {
    fn ident(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait SMTransition: PartialEq + Eq {
    type State: SMState;
    type Event: SMEvent;

    fn current_state(&self) -> Self::State;
    fn to_state(&self) -> Self::State;
    fn on(&self) -> Self::Event;
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait SMTransitionTable {
    type State: SMState;
    type Event: SMEvent;
    type Transition: SMTransition<State = Self::State, Event = Self::Event>;

    fn get_transition(&self, event: Self::Event) -> Option<Self::Transition>;
    fn add_transition(&mut self, tr_for: Self::Transition, transition: Self::Transition);
    fn remove_transition(&mut self, tr_for: Self::Transition);
}

pub trait StateMachine {
    type State: SMState;
    type Event: SMEvent;

    fn transition(&mut self, event: Self::Event) -> &Self::State;
    fn current_state(&self) -> &Self::State;
    fn is_initial(&self) -> bool {
        self.current_state().is_initial()
    }
    fn is_final(&self) -> bool {
        self.current_state().is_final()
    }
    fn is_in_state(&self, state: &Self::State) -> bool {
        self.current_state() == state
    }
    fn is_in_states(&self, states: &[Self::State]) -> bool {
        states.contains(&self.current_state())
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SMStEvPair<S: SMState, E: SMEvent> {
    state: S,
    event: E,
}

impl<S: SMState, E: SMEvent, T: SMTransition> From<T> for SMStEvPair<S, E>
where
    S: From<T::State>,
    E: From<T::Event>,
{
    fn from(tr: T) -> Self {
        Self {
            state: tr.current_state().into(),
            event: tr.on().into(),
        }
    }
}

impl<S: SMState, E: SMEvent> SMStEvPair<S, E> {
    pub fn new(state: S, event: E) -> Self {
        Self { state, event }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SMTransitionImpl<S: SMState, E: SMEvent> {
    from_state: S,
    to_state: S,
    on: E,
    tr_fn: Option<Box<dyn Fn() -> Result<(), Box<dyn std::error::Error>>>>,
}

impl<S: SMState, E: SMEvent> SMTransitionImpl<S, E> {
    pub fn new(from_state: S, to_state: S, on: E) -> Self {
        Self {
            from_state,
            to_state,
            on,
        }
    }
}

impl<S: SMState, E: SMEvent> SMTransition for SMTransitionImpl<S, E> {
    type State = S;
    type Event = E;

    fn current_state(&self) -> Self::State {
        self.from_state.clone()
    }

    fn to_state(&self) -> Self::State {
        self.to_state.clone()
    }

    fn on(&self) -> Self::Event {
        self.on.clone()
    }

    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(tr_fn) = &self.tr_fn {
            tr_fn()
        } else {
            Ok(())
        }
    }
}

pub struct GenericTransitionTable<S: SMState, E: SMEvent> {
    transitions: HashMap<E, SMTransitionImpl<S, E>>,
}

impl<S: SMState, E: SMEvent> GenericTransitionTable<S, E> {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
        }
    }
}

impl<S: SMState, E: SMEvent> Default for GenericTransitionTable<S, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: SMState, E: SMEvent> SMTransitionTable for GenericTransitionTable<S, E> {
    type State = S;
    type Event = E;
    type Transition = SMTransitionImpl<S, E>;

    fn get_transition(&self, event: Self::Event) -> Option<Self::Transition> {
        self.transitions.get(&event).cloned()
    }

    fn add_transition(&mut self, tr_for: Self::Transition, transition: Self::Transition) {
        self.transitions.insert(tr_for.on, transition);
    }

    fn remove_transition(&mut self, tr_for: Self::Transition) {
        self.transitions.remove(&tr_for.on);
    }
}

pub struct GenericStateMachine<T: SMTransitionTable> {
    current_state: T::State,
    trans_tbl: T,
}

impl<T: SMTransitionTable> StateMachine for GenericStateMachine<T> {
    type State = T::State;
    type Event = T::Event;

    fn transition(&mut self, event: Self::Event) -> &Self::State {
        let transition = self.trans_tbl.get_transition(event);
        if let Some(transition) = transition {
            self.current_state = transition.to_state();
        }
        &self.current_state
    }

    fn current_state(&self) -> &Self::State {
        &self.current_state
    }
}
