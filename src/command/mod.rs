use telegram_bot::User;

use maybe_from::*;
use meetup::*;

// use self::meet_at::*;
// use self::meet_now::*;

use std::fmt::Debug;

// mod meet_at;
// mod meet_now;

pub trait Command: Debug {
    fn execute(&self, user: User, chat_room: &mut Meetup) -> String;
}

// pub fn parse_command(text: &str) -> Option<Box<Command>> {
//     MeetNow::maybe_from(text).map(to_boxed_command)
//         .or_else(||MeetAt::maybe_from(text).map(to_boxed_command))
// }

fn to_boxed_command<T: Command + 'static>(command: T) -> Box<Command> {
    Box::new(command) as Box<Command>
}
