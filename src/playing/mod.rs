use crate::board::Board;

pub struct GameInProgress {
    pub board: Box<[Board]>,
    pub rules: crate::rules::Rules,
    pub turn: usize,
    pub moves_this_turn: Vec<Move>,
    pub all_moves: Vec<Move>,
}

pub enum Move {
    PlayCard { card: crate::card::Card, position: usize },
    DestroyCard { position: usize },
    ActivateCard { position: usize },
}
