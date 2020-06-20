use std::cmp;
use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

impl Clock {
    fn to_standard(minutes: i32) -> i32 {
        let mut minutes = minutes;

        while minutes < 0 {
            minutes += 60 * 24;
        }

        minutes % (24 * 60)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: Self::to_standard(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Self::to_standard(self.minutes + minutes),
        }
    }
}
