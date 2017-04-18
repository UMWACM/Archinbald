extern crate irc;

use irc::client::prelude::*;

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if msg.contains("archinbald ping") {
                    server.send_privmsg(target, "I'm right here...oh, sorry...PONG!").unwrap();
                }
                if msg.contains("archinbald echo") {
                    server.send_privmsg(target, msg.split_at(15).1).unwrap();
                }
                if msg.contains("archinbald info") {
                    server.send_privmsg(target, "I am Archinbald, your faithful ACM assistant. Currently running v0.1.3. Written in :rust: with :heart: by :logoilab:.").unwrap();
                }
            },
            _ => (),
        }
    }
}
