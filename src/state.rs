#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    Load,
    Menu(MenuState),
    Game(GameState),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MenuState {
    StartMenu,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum GameState {
    Running,
    Pause,
}
