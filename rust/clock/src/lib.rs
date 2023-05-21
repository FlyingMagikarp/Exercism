use std::fmt;

pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut tmp_mins: i32 = minutes;
        let mut tmp_hours: i32 = hours;

        if minutes >= 60 {
            tmp_mins = minutes % 60;
            tmp_hours += minutes / 60;
        }

        if minutes < 0 {
            tmp_mins = (minutes % 60)+60; // why does this work for minutes = -40 as input, but minutes % 60 == -40?????
            tmp_hours += (minutes / 60)-1;
        }

        while tmp_hours < 0 {
            tmp_hours = 24 + tmp_hours;
        }

        while tmp_hours >= 24 {
            tmp_hours -= 24;
        }

        Clock { minutes: tmp_mins, hours: tmp_hours }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {minutes, hours: self.hours}
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}