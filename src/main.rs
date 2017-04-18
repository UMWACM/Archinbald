extern crate chrono;
extern crate irc;
extern crate rand;

use chrono::Local;

use irc::client::prelude::*;

use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

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
                if msg.contains("@archinbald help") {
                    server.send_privmsg(target, decorate(server.config().get_option("help")).as_str()).unwrap();
                }
                if msg.contains("@archinbald ping") {
                    server.send_privmsg(target, decorate("I'm right here...oh, sorry...PONG!").as_str()).unwrap();
                }
                if msg.contains("@archinbald echo") {
                    server.send_privmsg(target, decorate(msg.split_at(17).1).as_str()).unwrap();
                }
                if msg.contains("@archinbald info") {
                    server.send_privmsg(target, decorate("I am Archinbald, your faithful ACM assistant. Currently running v0.1.8. Written in :rust: with :heart: by :logoilab:.").as_str()).unwrap();
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
                if msg.contains("@archinbald quoteme") {
                    let file = match File::open(Path::new("quotes.txt")) {
                        Err(e) => panic!("couldn't open quote file\n{}", e),
                        Ok(file) => file,
                    };

                    let mut reader = BufReader::new(file);
                    let mut lines = String::new();
                    reader.read_to_string(&mut lines).unwrap();
                    let lines_vec: Vec<&str> = lines.split("\n").collect();
                    let rand_num: usize = thread_rng().gen_range(0, lines_vec.len());
                    server.send_privmsg(target, lines_vec.get(rand_num).unwrap()).unwrap();
                }
            },
            _ => (),
        }
    }
}
