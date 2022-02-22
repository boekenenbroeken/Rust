use crate::{COLS, ROWS};

pub type Frame = [[char; ROWS]; COLS];

pub fn new_frame() -> Frame {
    [[' '; ROWS]; COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}