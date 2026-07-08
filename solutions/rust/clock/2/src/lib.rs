use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
        hours: i32,
        minutes: i32
    }

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = Self { hours: hours, minutes: 0 };
        hours.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        let mut final_minutes = total_minutes % 60;
        let mut hr_to_add = total_minutes / 60;
        
        if final_minutes < 0 {
            final_minutes += 60;
            hr_to_add -= 1;
        }

        let total_hour = self.hours + hr_to_add;
        let mut final_hour = total_hour % 24;
        if final_hour < 0 {
            final_hour += 24;
        }

        Self { hours: final_hour, minutes: final_minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hours < 10{
            write!(f, "0")?;
        }
        write!(f, "{}:", self.hours)?;
        
        if self.minutes < 10{
            write!(f, "0")?;
        }
        write!(f, "{}", self.minutes)
    }
}