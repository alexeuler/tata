use async_std::io;
use structopt::StructOpt;

use crate::network::send;

#[derive(StructOpt, Debug)]
enum Opt {
    /// Prints info about your id
    Me,
    /// Get all peers list
    Peers,
    /// List logs
    Logs,
    /// Get messages for a peer
    Messages {
        #[structopt(short, long)]
        peer: String,
    },
    SendMessage {
        #[structopt(short, long)]
        peer: String,
        #[structopt(short, long)]
        message: String,
    },
}

pub async fn start_command_line() {
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .await
            .expect("Failed to read line");
        let tokens = vec!["."].into_iter().chain(command.split_whitespace());
        let command = match Opt::from_iter_safe(tokens) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        match command {
            Opt::SendMessage { peer, message } => {
                if !send(peer, message) {
                    log::error!("Error sending message to peer");
                }
            }
            _ => (),
        }
    }
}
