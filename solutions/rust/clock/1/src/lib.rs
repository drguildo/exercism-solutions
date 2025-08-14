#[derive(Debug, PartialEq)]
pub struct Clock(i32);

fn normalise_minutes(minutes: i32) -> i32 {
    const MINUTES_IN_DAY: i32 = 24 * 60;

    let minutes = minutes % MINUTES_IN_DAY;
    if minutes < 0 {
        minutes + MINUTES_IN_DAY
    } else {
        minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(normalise_minutes((hours * 60) + minutes))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock(normalise_minutes(self.0 + minutes))
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        let hours = (self.0 / 60) % 24;
        let minutes = self.0 % 60;
        format!("{:02}:{:02}", hours, minutes)
    }
}
