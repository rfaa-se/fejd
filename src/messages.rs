use crate::{entities::EntityTypeIndex, states::State};

#[derive(Debug, Copy, Clone)]
pub enum Sender {
    None,
    Engine,
    Log,
    State,
    Menu,
    Game,
    World,
    Logic,
    Audio,
}

#[derive(Debug)]
pub enum Message {
    Engine(EngineMessage),
    State(StateMessage),
    Request(RequestMessage),
    Logic(LogicMessage),
    Audio(AudioMessage),
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
    DebugGet(bool),
}

#[derive(Debug)]
pub enum EngineRequestMessage {
    SetTicksPerSecond(u8),
    SetDebug(bool),
    GetDebug,
}

#[derive(Debug)]
pub enum StateMessage {
    StateSet(State),
}

#[derive(Debug)]
pub enum StateRequestMessage {
    SetState(State),
}

#[derive(Debug)]
pub enum LogicMessage {
    Death(EntityTypeIndex),
    Collision(EntityTypeIndex, EntityTypeIndex),
}

#[derive(Debug)]
pub enum AudioMessage {
    Play(EntityTypeIndex),
}
