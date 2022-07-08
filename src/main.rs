use std::io::Error;
use std::net::TcpStream;
use rand::Rng;
use crate::challenge::{Challenge as ChallengeMD5, MD5HashCash};
use crate::message::Challenge;
mod tcp;
mod challenge;
mod message;



fn main() {
    let mut stream = TcpStream::connect("localhost:7878").unwrap();
    let mut stream2 = TcpStream::connect("localhost:7878").unwrap();

    // Envoie de Hello pour start la conversation
    tcp::write(&mut stream, &message::Message::Hello).unwrap();
    tcp::write(&mut stream2, &message::Message::Hello).unwrap();

    loop {
        let message = tcp::read(&mut stream).unwrap();
        let finish : Result<bool, Error> = handle_message(message, &mut stream);
        match finish {
            Err(e) => println!("Error while communicating with the server {}", e),
            Ok(true) => {  },
            Ok(false) => break,
        }

        let message2 = tcp::read(&mut stream2).unwrap();
        let finish2 : Result<bool, Error> = handle_message(message2, &mut stream2);
        match finish2 {
            Err(e) => println!("Error while communicating with the server {}", e),
            Ok(true) => {  },
            Ok(false) => break,
        }
    }
}

pub fn handle_message(message: message::Message, stream: &mut TcpStream) -> Result<bool, Error> {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    // Username aléatoire
    let username = format!("{}{}", "playername", n1);
    // On stock les joueurs afin de passer la patate
    let mut players: Vec<message::PublicPlayer> = vec![];

    match message {
        message::Message::Welcome(..) => {
            tcp::write(
                stream,
                &message::Message::Subscribe(
                    message::Subscribe {
                        name: username
                    }
                )
            )?;
            Ok(true)
        },
        message::Message::SubscribeResult(subcribe_result) => {
            match subcribe_result {
                message::SubscribeResult::Ok => Ok(true),
                message::SubscribeResult::Err(..) => Ok(false)
            }
        },
        message::Message::PublicLeaderBoard(public_leader_board) => {
            players.append(&mut public_leader_board.0
                .clone()
                .into_iter()
                .filter(|p| p.name != username)
                .collect::<Vec<message::PublicPlayer>>()
            );

            Ok(!players.is_empty())
        },
        message::Message::Challenge(challenge) => {
            match challenge {
                Challenge::MD5HashCash(hash_code) => {
                    let challenge = MD5HashCash::new(hash_code.clone());
                    let answer = challenge.solve();

                    let target = choose_target(&players);

                    let message = message::ChallengeResult {
                        answer: message::ChallengeAnswer::MD5HashCash(answer),
                        next_target: target
                    };

                    tcp::write(stream, &message::Message::ChallengeResult(message)) ?;
                    Ok(true)
                }
            }
        },
        message::Message::RoundSummary(..) => Ok(true),
        message::Message::EndOfGame(..) => {
            match tcp::close(stream) {
                Ok(..) => println!("Client shutdown."),
                Err(e) => println!("Error closing client. {}", e),
            };
            Ok(false)
        },
        _ => {
            println!("Message does not exist in the enum{:?}", message);
            Ok(false)
        }
    }
}

fn choose_target(players: &Vec<message::PublicPlayer>) -> String {
    let target = rand::thread_rng().gen_range(0..players.len());
    let player_name: String = players[target].name.clone();
    return player_name;
}