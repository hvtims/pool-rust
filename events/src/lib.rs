use chrono::Duration;
use colored::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_content = self.content.truecolor(self.color.0, self.color.1, self.color.2);
        write!(f, "({:?}, {}, {})", self.position, self.size, colored_content)
    }
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Self::make_notification(50, (50, 50, 50), Position::Bottom, text),
            Event::Registration(duration) => {
                let content = Self::format_duration(*duration);
                Self::make_notification(30, (255, 2, 22), Position::Top, &content)
            }
            Event::Appointment(text) => Self::make_notification(100, (200, 200, 3), Position::Center, text),
            Event::Holiday => Self::make_notification(25, (0, 255, 0), Position::Top, "Enjoy your holiday"),
        }
    }

    fn make_notification(size: u32, color: (u8, u8, u8), position: Position, content: &str) -> Notification {
        Notification {
            size,
            color,
            position,
            content: content.to_string(),
        }
    }

    fn format_duration(duration: Duration) -> String {
        let sec = duration.num_seconds();
        let hours = sec / 3600;
        let minutes = (sec % 3600) / 60;
        let seconds = sec % 60;

        format!("You have {}H:{}M:{}S left before the registration ends", hours, minutes, seconds)
    }
}
