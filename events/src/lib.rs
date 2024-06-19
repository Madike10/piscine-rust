extern crate colored;
extern crate chrono;
use chrono::Duration;
use std::fmt::Formatter;
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

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            format!(
                "\x1b[38;2;{};{};{}m{}\x1b[0m",
                self.color.0, self.color.1, self.color.2, self.content
            )
        )
    }
}

use Event::*;
// Create a method named notify which returns a Notification with the following characteristics for each of:

//     Remainder(text):
//         size: 50
//         color: (50, 50, 50)
//         position: Bottom
//         content: the text associated to the enum variant
//     Registration(chrono::Duration):
//         size: 30
//         color: (255, 2, 22)
//         position: Top
//         content: "You have {duration} left before the registration ends"
//     Appointment(text):
//         size: 100
//         color: (200, 200, 3)
//         position: Center
//         content: text associated to the value
//     Holiday:
//         size: 25
//         color: (0, 255, 0)
//         position: Top
//         content: "Enjoy your holiday"

// duration must be displayed in the form of {hours}H:{minutes}M:{seconds}S. The time will represent the remaining time before the event starts. For example, if there are 13 hours, 38 minutes and 14 seconds left, then the content will be "You have 13H:38M:14S left before the registration ends"

// Implement the std::fmt::Display trait so the text of the notifications are printed in the right color in the command line.


impl Event<'_> {
	pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(duration) => {
                let total_secs = duration.num_seconds();
                let (hours, remainder) = (total_secs / 3600, total_secs % 3600);
                let (minutes, seconds) = (remainder / 60, remainder % 60);
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    ),
                }
            }
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
	}
}


// $ cargo run
// (Bottom, 50, [38;2;50;50;50mGo to the doctor[0m)
// (Top, 30, [38;2;255;2;22mYou have 13H:38M:14S left before the registration ends[0m)
// (Center, 100, [38;2;200;200;3mGo to the doctor[0m)
// (Top, 25, [38;2;0;255;0mEnjoy your holiday[0m)
// $
