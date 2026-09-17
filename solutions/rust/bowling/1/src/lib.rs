use crate::Error::{GameComplete, NotEnoughPinsLeft};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: u16,
    throw_position: u16,
    remaining_pin: u16,
    throws: Vec<u16>,
    prev_throw: u16,
    strike_throws: Vec<u16>,
    spare_throws: Vec<u16>,
    check_fillballs_spare: bool,
    check_fillballs_strike: bool,
    total_score: u16
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            current_frame: 1,
            throw_position: 1,
            remaining_pin: 10,
            throws: Vec::new(),
            prev_throw: 0,
            strike_throws: Vec::new(),
            spare_throws: Vec::new(),
            check_fillballs_spare: true,
            check_fillballs_strike: true,
            total_score: 0
        }
    }


    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.remaining_pin{
            return Err(NotEnoughPinsLeft);
        }

        if self.current_frame <= 10 {
            if self.strike_throws.contains(&(self.prev_throw)) || self.spare_throws.contains(&(self.prev_throw)){
                self.total_score += pins;
            }
            if self.strike_throws.contains(&(self.prev_throw.checked_sub(1).unwrap_or(0))) {
                self.total_score += pins;
            }
            
            //add broken pins as score at each throw and count unbroken pins
            self.total_score += pins;
            self.remaining_pin -= pins;

            //process throw
            let _ = self.process_throw(pins);
        }
        else {
            if self.current_frame == 11 {
                let mut fill_balls = false;
                if self.throw_position == 1 {
                    if self.strike_throws.contains(&(self.prev_throw)) || self.spare_throws.contains(&(self.prev_throw)) {
                        self.total_score += pins;
                        fill_balls = true;
                    }

                    if self.strike_throws.contains(&(self.prev_throw.checked_sub(1).unwrap_or(0))) {
                        self.total_score += pins;
                        fill_balls = true;
                    }

                    if !fill_balls {
                        self.check_fillballs_spare = true;
                        return Err(GameComplete);
                    }

                    self.remaining_pin -= pins;
                    if self.remaining_pin == 0 {
                        self.remaining_pin = 10;
                    }
                    self.throw_position = 2;
                    self.prev_throw += 1;
                    self.check_fillballs_spare = true;
                }
                else {
                    if self.strike_throws.contains(&(self.prev_throw.checked_sub(1).unwrap_or(0))) {
                        if pins > self.remaining_pin{
                            self.check_fillballs_strike = true;
                            return Err(NotEnoughPinsLeft);
                        }
                        self.total_score += pins;
                        // self.current_frame += 1;
                    }
                    else {
                        self.check_fillballs_strike = true;
                        return Err(GameComplete);
                    }
                    self.check_fillballs_strike = true;
                    Self::go_to_next_frame(self);
                }

            }
            else {
                return  Err(GameComplete);
            }            
        }

        Ok(())
    }

    fn process_throw(&mut self, pins: u16) -> Result<(), Error>{
        //Record all throws
        self.throws.push(pins);
        self.prev_throw += 1;

        //check for strike
        if self.throw_position == 1 && pins == 10 {
            self.strike_throws.push(self.prev_throw);
            if self.current_frame == 10 {
                self.check_fillballs_strike = false;
            }
            Self::go_to_next_frame(self);
            return Ok(());
        }

        //check for spare
        if self.throw_position == 2 && self.remaining_pin == 0 {
            self.spare_throws.push(self.prev_throw);
            if self.current_frame == 10 {
                self.check_fillballs_spare = false;
            }
            Self::go_to_next_frame(self);
            return Ok(());
        }

        //Change frame after each pair of throw
        if self.throw_position == 2 {
            Self::go_to_next_frame(self);
        }
        else {
            self.throw_position = 2;
        }
        Ok(())
    }

    fn go_to_next_frame(&mut self) {
        self.current_frame += 1;
        self.throw_position = 1;
        self.remaining_pin = 10;
    }

    pub fn score(&self) -> Option<u16> {
        dbg!(self.current_frame);
        if self.current_frame > 10 && self.check_fillballs_spare && self.check_fillballs_strike{
            return Some(self.total_score);
        }
        None
    }
}
