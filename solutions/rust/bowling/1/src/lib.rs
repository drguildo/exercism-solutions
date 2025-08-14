#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
    StrikeBonus(u16, u16),
    SpareBonus(u16),
    Unfinished(u16),
}

pub struct BowlingGame {
    frame_in_progress: bool,
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame_in_progress: false,
            rolls: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frame_in_progress {
            if (self.rolls.last().unwrap() + pins) > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        self.rolls.push(pins);

        if pins == 10 {
            self.frame_in_progress = false;
        } else {
            if self.frame_in_progress {
                self.frame_in_progress = false;
            } else {
                self.frame_in_progress = true;
            }
        }

        Ok(())
    }

    fn get_frame(&self, frame: u8) -> Option<Frame> {
        if frame > 11 {
            return None;
        }

        let mut frame_index = 0;
        for current_frame in 1..=frame {
            if frame_index >= self.rolls.len() {
                return None;
            }

            if current_frame == 11 {
                if (frame_index + 1) < self.rolls.len() {
                    return Some(Frame::StrikeBonus(
                        self.rolls[frame_index],
                        self.rolls[frame_index + 1],
                    ));
                } else {
                    return Some(Frame::SpareBonus(self.rolls[frame_index]));
                }
            }

            if self.is_strike(frame_index) {
                if current_frame == frame {
                    return Some(Frame::Strike);
                } else {
                    frame_index += 1;
                }
                continue;
            }

            if (frame_index + 1) < self.rolls.len() {
                if current_frame == frame {
                    if self.is_spare(frame_index) {
                        return Some(Frame::Spare(
                            self.rolls[frame_index],
                            self.rolls[frame_index + 1],
                        ));
                    } else {
                        return Some(Frame::Open(
                            self.rolls[frame_index],
                            self.rolls[frame_index + 1],
                        ));
                    }
                } else {
                    frame_index += 2;
                }
                continue;
            }

            if frame_index < self.rolls.len() {
                return Some(Frame::Unfinished(self.rolls[frame_index]));
            }
        }

        None
    }

    fn is_complete(&self) -> bool {
        let last_frame = self.get_frame(10);

        if let Some(Frame::Open(_, _)) = last_frame {
            return true;
        }

        if let Some(Frame::Spare(_, _)) = last_frame {
            return self.get_frame(11).is_some();
        }

        if let Some(Frame::Strike) = last_frame {
            if let Some(bonus_rolls) = self.get_frame(11) {
                return matches!(bonus_rolls, Frame::StrikeBonus(_, _));
            }
        }

        false
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        (self.rolls[frame_index] + self.rolls[frame_index + 1]) == 10
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut score = 0;
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.is_strike(frame_index) {
                score += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            } else if self.is_spare(frame_index) {
                score += 10 + self.rolls[frame_index + 2];
                frame_index += 2;
            } else {
                score += self.rolls[frame_index] + self.rolls[frame_index + 1];
                frame_index += 2;
            }
        }

        Some(score)
    }
}
