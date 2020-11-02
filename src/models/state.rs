#[derive(Debug, PartialEq)]
pub enum GameState {
    Menu,
    Play,
    Exit,
    None,
}

#[derive(PartialEq)]
pub enum GameCmd {
    Default,
    Ok,
    Reset,
}