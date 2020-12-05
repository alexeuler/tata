use structopt::StructOpt;

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
}

fn main() {
    loop {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let tokens = vec!["."].into_iter().chain(command.split_whitespace());
        match Opt::from_iter_safe(tokens) {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{}", e),
        }
    }
}
