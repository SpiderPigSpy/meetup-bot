#![feature(try_from)]

extern crate telegram_bot;
extern crate regex;
extern crate time;
#[macro_use] extern crate log;
extern crate env_logger;

mod meetup;
mod command;
mod manager;
mod bot;
mod message;
mod error;
mod send;
mod maybe_from;

use telegram_bot::*;

fn main() {
    env_logger::init().unwrap();
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    info!("getMe: {:?}", api.get_me());
    let res = listen(api);
    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}

fn listen(api: Api) -> Result<()> {
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    // Fetch new updates via long poll method
    let mut bot = bot::Bot::new(&api);
    let res = listener.listen(|u| {
        if let Some(message) = u.message {
            debug!("Message {:?}", &message);
            bot.process_message(message);
        }
        Ok(ListeningAction::Continue)
    });
    return res;
}
