use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: VecDeque<u16>,
    current_frame: usize,
    current_roll: usize,
    last_frame_strike: bool,
    last_frame_spare: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: VecDeque::new(),
            current_frame: 0,
            current_roll: 0,
            last_frame_strike: false,
            last_frame_spare: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 && self.current_roll > 2 {
            return Err(Error::GameComplete);
        }

        if self.frames.len() >= 10 && pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.frames.back_mut() {
            Some(frame) => {
                *frame += pins;
                self.current_roll += 1;
                if self.current_roll == 2 {
                    self.current_roll = 0;
                    self.current_frame += 1;
                }
            },
            None => {
                self.frames.push_back(pins);
                self.current_roll += 1;
                if self.current_roll == 2 {
                    self.current_roll = 0;
                    self.current_frame += 1;
                }
            }
        }

        if self.current_frame == 9 && pins == 10 {
            self.last_frame_strike = true;
        } else if self.current_frame == 9 && pins + self.frames[self.frames.len() - 2] == 10 {
            self.last_frame_spare = true;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut frame_index = 0;

        while frame_index < self.frames.len() {
            let frame_pins = self.frames[frame_index];
            if frame_index == 9 && self.last_frame_strike {
                score += 10 + self.frames[frame_index + 1] + self.frames[frame_index + 2];
                frame_index += 3;
            } else if frame_index == 9 && self.last_frame_spare {
                score += 10 + self.frames[frame_index + 1];
                frame_index += 2;
            } else {
                score += frame_pins;
                frame_index += 1;
            }
        }

        Some(score)
    }
}


fn main() -> Result<(), Error> {
    let mut game = BowlingGame::new();
    game.roll(0)?; // frame 1
    game.roll(10)?; // frame 1: spare
    game.roll(10)?; // frame 2: strike
    game.roll(5)?; // frame 3
    game.roll(5)?; // frame 3: spare
    game.roll(10)?; // frame 4: strike
    game.roll(10)?; // frame 5: strike
    game.roll(10)?; // frame 6: strike
    game.roll(10)?; // frame 7: strike
    game.roll(10)?; // frame 8: strike
    game.roll(10)?; // frame 9: strike
    game.roll(10)?; // frame 10: strike
    game.roll(2)?; // fill ball 1
    game.roll(8)?; // fill ball 2
    println!("{:?}", game.score());

    let mut perfect_game = BowlingGame::new();
    for _ in 0..12 {
        perfect_game.roll(10)?;
    }
    println!("{:?}", perfect_game.score());
    Ok(())
}
