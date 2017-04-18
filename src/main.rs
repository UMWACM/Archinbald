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
                    server.send_privmsg(target, decorate("I am Archinbald, your faithful ACM assistant. Currently running v0.1.6. Written in :rust: with :heart: by :logoilab:.").as_str()).unwrap();
                }
                if msg.contains("@archinbald time") {
                    server.send_privmsg(target, decorate(Local::now().to_string().as_str()).as_str()).unwrap();
                }
                if msg.contains("@archinbald stop") {
                    if msg.len() == 16 {
                        server.send_privmsg(target, decorate("What is the password?").as_str()).unwrap();
                    } else {
                        if msg.contains(server.config().get_option("stop_pass")) {
                            server.send_privmsg(target, decorate("Okay...goodby cruel world.").as_str()).unwrap();
                            panic!("User forced stop.");
                        } else {
                            server.send_privmsg(target, decorate("What is that rubbish password!?").as_str()).unwrap();
                        }
                    }
                }
            },
            _ => (),
        }
    }
}
