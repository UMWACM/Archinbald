extern crate chrono;
extern crate irc;


use chrono::Local;

use irc::client::prelude::*;

fn decorate(msg: &str) -> String {
    String::from("Archinbald Bot: ") + msg
}

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap();
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if msg.contains("@archinbald ping") {
                    server.send_privmsg(target, decorate("I'm right here...oh, sorry...PONG!").as_str()).unwrap();
                }
                if msg.contains("@archinbald echo") {
                    server.send_privmsg(target, decorate(msg.split_at(15).1).as_str()).unwrap();
                }
                if msg.contains("@archinbald info") {
                    server.send_privmsg(target, decorate("I am Archinbald, your faithful ACM assistant. Currently running v0.1.5. Written in :rust: with :heart: by :logoilab:.").as_str()).unwrap();
                }
                if msg.contains("@archinbald time") {
                    server.send_privmsg(target, decorate(Local::now().to_string().as_str()).as_str()).unwrap();
                }
            },
            _ => (),
        }
    }
}
