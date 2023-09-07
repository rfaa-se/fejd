use crate::states::State;

#[derive(Debug, Copy, Clone)]
pub enum Sender {
    None,
    Engine,
    Log,
    State,
    Menu,
    Game,
}

#[derive(Debug)]
pub enum Message {
    Engine(EngineMessage),
    State(StateMessage),
    Request(RequestMessage),
}

#[derive(Debug)]
pub enum RequestMessage {
    Engine(EngineRequestMessage),
    State(StateRequestMessage),
}

#[derive(Debug)]
pub enum EngineMessage {
    TicksPerSecondSet(u8),
    DebugSet(bool),
}

#[derive(Debug)]
pub enum EngineRequestMessage {
    SetTicksPerSecond(u8),
    SetDebug(bool),
}

#[derive(Debug)]
pub enum StateMessage {
    StateSet(State),
}

#[derive(Debug)]
pub enum StateRequestMessage {
    SetState(State),
}
