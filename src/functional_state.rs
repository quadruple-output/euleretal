pub trait FunctionalState {
    type State: PartialEq;

    fn functional_state(&self) -> Self::State;
}
