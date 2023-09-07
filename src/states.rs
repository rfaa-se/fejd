mod game;
mod manager;
mod menu;

pub use self::game::GameState;
pub use self::manager::StateManager;
pub use self::menu::MenuState;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum State {
    None,
    Menu,
    Game,
}
