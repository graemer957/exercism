#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    throw: u8,
    pins_standing: u16,
    last_throws: Vec<u16>,
}

#[derive(Debug)]
enum Frame {
    Open(u16, u16),
    Spare(u16),
    Strike,
    Last(Vec<u16>),
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            throw: 1,
            pins_standing: 10,
            ..Self::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || self.pins_standing < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }

        if self.frames.len() == 9 {
            self.last_throws.push(pins);
            if pins == 10 {
                self.pins_standing = 10;
            } else {
                self.pins_standing = 10 - pins;
            }
            match (self.throw, self.last_throws.iter().sum::<u16>() >= 10) {
                (2, true) if self.last_throws[0] != 10 => {
                    self.pins_standing = 10;
                }
                (2, false) => {
                    self.last_throws.push(0);
                    self.frames.push(Frame::Last(self.last_throws.to_vec()));
                }
                (3, _) => {
                    self.frames.push(Frame::Last(self.last_throws.to_vec()));
                }
                _ => {}
            }

            self.throw += 1;
        } else if pins == 10 && self.throw == 1 {
            self.frames.push(Frame::Strike);
        } else if self.throw == 1 {
            self.pins_standing = 10 - pins;
            self.throw = 2;
        } else {
            let first_throw = 10 - self.pins_standing;
            if first_throw + pins == 10 {
                self.frames.push(Frame::Spare(first_throw));
            } else {
                self.frames.push(Frame::Open(first_throw, pins));
            }
            self.throw = 1;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.is_empty() || self.frames.len() < 10 {
            return None;
        }

        let mut score = 0;
        for (index, frame) in self.frames.iter().enumerate() {
            score += match frame {
                Frame::Open(first_roll, second_roll) => first_roll + second_roll,
                Frame::Spare(_) => {
                    if let Some(frame_ahead) = self.frames.get(index + 1) {
                        10 + match frame_ahead {
                            Frame::Open(first_roll, _) => *first_roll,
                            Frame::Spare(first_roll) => *first_roll,
                            Frame::Strike => 10,
                            Frame::Last(rolls) => rolls[0],
                        }
                    } else {
                        0
                    }
                }
                Frame::Strike => {
                    if let Some(frame_ahead) = self.frames.get(index + 1) {
                        10 + match frame_ahead {
                            Frame::Open(first_roll, second_roll) => *first_roll + *second_roll,
                            Frame::Spare(_) => 10,
                            Frame::Strike => {
                                if let Some(frame_2_ahead) = self.frames.get(index + 2) {
                                    10 + match frame_2_ahead {
                                        Frame::Open(first_roll, _) => *first_roll,
                                        Frame::Spare(first_roll) => *first_roll,
                                        Frame::Strike => 10,
                                        Frame::Last(rolls) => rolls[0],
                                    }
                                } else {
                                    10
                                }
                            }
                            Frame::Last(rolls) => rolls[0] + rolls[1],
                        }
                    } else {
                        10
                    }
                }
                Frame::Last(rolls) => rolls.iter().sum::<u16>(),
            }
        }

        Some(score)
    }
}
