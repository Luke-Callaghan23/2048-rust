use std::error::Error;


#[derive(Debug)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub enum MoveResult {
    Continue(Board),
    Quit(Reason),
    Err(&'static str)
}

pub enum Reason {
    QPressed,
    Win( Board ),
    Loss
}

pub const SIDE_SIZE: usize = 4;

pub struct Board (
    pub [u32; SIDE_SIZE * SIDE_SIZE]
);

pub enum Control {
    Direction(Directions),
    Help,
    Quit,
}