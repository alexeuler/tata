#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use async_std::{io, prelude::*};
use command::Command;
use diesel::sqlite::SqliteConnection;
use std::error::Error;
use tata_core::Core;
// use libp2p::{
//     floodsub::{self, Floodsub, FloodsubEvent},
//     identity,
//     mdns::{Mdns, MdnsEvent},
//     swarm::NetworkBehaviourEventProcess,
//     Multiaddr, NetworkBehaviour, PeerId, Swarm,
// };
// use std::{
//     error::Error,
//     task::{Context, Poll},
// };

mod command;
mod db;
mod models;
mod repos;
mod schema;

fn handle_command(cmd_res: Result<String, io::Error>, conn: &SqliteConnection) {
    let cmd_str = cmd_res.expect("Std io error");
    if cmd_str != "" {
        let cmd: Command = cmd_str.parse().expect("Infallible; qed");
        match cmd {
            Command::ListUsers => {
                let users_res = repos::UsersRepo::list(conn);
                match users_res {
                    Ok(users) => println!("{:?}", users),
                    Err(e) => println!("{}", e),
                }
            }
            Command::CreateUser => {}
            _ => println!("{}", Command::help()),
        }
    }
    prompt();
}

fn prompt() {
    print!("> ");
    let _ = <std::io::Stdout as std::io::Write>::flush(&mut std::io::stdout());
}

fn handle_event(event: tata_core::Event) {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let conn = db::establish_connection();
    db::run_migrations(&conn);

    let core = Core::new();
    let stdin = io::BufReader::new(io::stdin())
        .lines()
        .map(|cmd| handle_command(cmd, &conn));
    let events = core.events.map(handle_event);
    let mut stream = futures::stream::select(stdin, events);
    prompt();
    loop {
        let _ = stream.next().await;
    }
    // let mut line = String::new();
    // let line_ref = &mut line;
    // loop {
    //     let line_future = Box::pin(stdin.read_line(line_ref));
    //     match select(line_future, core).await {
    //         Either::Left(_) => {
    //             let args = vec![""].into_iter().chain(line_ref.split(" "));
    //             match command::Command::from_iter_safe(args) {
    //                 Ok(cmd) => println!("{:?}", cmd),
    //                 Err(e) => {
    //                     if line != "" {
    //                         println!("{}", e);
    //                     }
    //                 }
    //             }
    //             stdout.write(b">").await?;
    //             stdout.flush().await?;
    //         }
    //         Either::Right((_, _event)) => (),
    //     }
    // }

    // let opt = command::Command::from_args();
    // Ok(())
    // task::block_on(future::poll_fn(move |cx: &mut Context| {
    //     loop {
    //         match stdin.try_poll_next_unpin(cx)? {
    //             Poll::Ready(Some(ref line)) => {
    //                 let args = vec![""].into_iter().chain(line.split(" "));
    //                 match command::Command::from_iter_safe(args) {
    //                     Ok(cmd) => println!("{:?}", cmd),
    //                     Err(e) => {
    //                         if line != "" {
    //                             println!("{}", e);
    //                         }
    //                     }
    //                 }
    //             }
    //             Poll::Ready(None) => panic!("Stdin closed"),
    //             Poll::Pending => break,
    //         }
    //     }
    //     Poll::Pending
    // }))

    // // Create a random PeerId
    // let local_key = identity::Keypair::generate_ed25519();
    // let local_peer_id = PeerId::from(local_key.public());
    // println!("Local peer id: {:?}", local_peer_id);

    // // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    // let transport = libp2p::build_development_transport(local_key)?;

    // // Create a Floodsub topic
    // let floodsub_topic = floodsub::Topic::new("chat");

    // // We create a custom network behaviour that combines floodsub and mDNS.
    // // In the future, we want to improve libp2p to make this easier to do.
    // // Use the derive to generate delegating NetworkBehaviour impl and require the
    // // NetworkBehaviourEventProcess implementations below.
    // #[derive(NetworkBehaviour)]
    // struct MyBehaviour {
    //     floodsub: Floodsub,
    //     mdns: Mdns,

    //     // Struct fields which do not implement NetworkBehaviour need to be ignored
    //     #[behaviour(ignore)]
    //     #[allow(dead_code)]
    //     ignored_member: bool,
    // }

    // impl NetworkBehaviourEventProcess<FloodsubEvent> for MyBehaviour {
    //     // Called when `floodsub` produces an event.
    //     fn inject_event(&mut self, message: FloodsubEvent) {
    //         if let FloodsubEvent::Message(message) = message {
    //             println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
    //         }
    //     }
    // }

    // impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    //     // Called when `mdns` produces an event.
    //     fn inject_event(&mut self, event: MdnsEvent) {
    //         match event {
    //             MdnsEvent::Discovered(list) =>
    //                 for (peer, _) in list {
    //                     self.floodsub.add_node_to_partial_view(peer);
    //                 }
    //             MdnsEvent::Expired(list) =>
    //                 for (peer, _) in list {
    //                     if !self.mdns.has_node(&peer) {
    //                         self.floodsub.remove_node_from_partial_view(&peer);
    //                     }
    //                 }
    //         }
    //     }
    // }

    // // Create a Swarm to manage peers and events
    // let mut swarm = {
    //     let mdns = Mdns::new()?;
    //     let mut behaviour = MyBehaviour {
    //         floodsub: Floodsub::new(local_peer_id.clone()),
    //         mdns,
    //         ignored_member: false,
    //     };

    //     behaviour.floodsub.subscribe(floodsub_topic.clone());
    //     Swarm::new(transport, behaviour, local_peer_id)
    // };

    // // Reach out to another node if specified
    // if let Some(to_dial) = std::env::args().nth(1) {
    //     let addr: Multiaddr = to_dial.parse()?;
    //     Swarm::dial_addr(&mut swarm, addr)?;
    //     println!("Dialed {:?}", to_dial)
    // }

    // // Read full lines from stdin
    // let mut stdin = io::BufReader::new(io::stdin()).lines();

    // // Listen on all interfaces and whatever port the OS assigns
    // Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;

    // // Kick it off
    // let mut listening = false;
    // task::block_on(future::poll_fn(move |cx: &mut Context| {
    //     loop {
    //         match stdin.try_poll_next_unpin(cx)? {
    //             Poll::Ready(Some(line)) => swarm.floodsub.publish(floodsub_topic.clone(), line.as_bytes()),
    //             Poll::Ready(None) => panic!("Stdin closed"),
    //             Poll::Pending => break
    //         }
    //     }
    //     loop {
    //         match swarm.poll_next_unpin(cx) {
    //             Poll::Ready(Some(event)) => println!("{:?}", event),
    //             Poll::Ready(None) => return Poll::Ready(Ok(())),
    //             Poll::Pending => {
    //                 if !listening {
    //                     for addr in Swarm::listeners(&swarm) {
    //                         println!("Listening on {:?}", addr);
    //                         listening = true;
    //                     }
    //                 }
    //                 break
    //             }
    //         }
    //     }
    //     Poll::Pending
    // }))
}
