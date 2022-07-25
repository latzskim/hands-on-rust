#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AawaitingInput,
    PlayerTurn,
    MonsterTurn,
}
