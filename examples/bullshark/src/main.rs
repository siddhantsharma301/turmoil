use rand::rngs::StdRng;
use rand::SeedableRng as _;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use tokio::sync::mpsc::{channel, Receiver};
use tracing::{info_span, Instrument};
use turmoil::{net, Builder};

// Narwhal imports
use config::{Authority, Committee, PrimaryAddresses, WorkerAddresses};
use consensus::Consensus;
use crypto::{generate_keypair, PublicKey, SecretKey, Signature};
use primary::Primary;
use store::Store;
use worker::Worker;

// Local imports
mod commands;
mod utils;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let committee = committee();

    // let mut sim = Builder::new().build();

    // sim.host("primary0", move || {
    //     let keypair = KeyPair::new();

    // });
}

// fn setup() {
//     let cmd = commands::CommandMaker::kill();
//     let _ = Command::new("sh")
//         .arg(cmd.clone())
//         .spawn()
//         .expect("failed to kill nodes")
//         .wait()
//         .expect("failed to wait on command");
//     println!("{:?}", cmd);

//     let cmd = format!("{} ; {}", commands::CommandMaker::clean_logs(), commands::CommandMaker::cleanup());
//     let _ = Command::new("sh")
//         .arg(cmd.clone())
//         .spawn()
//         .expect("failed to cleanup")
//         .wait()
//         .expect("failed to wait on command");
//     println!("{:?}", cmd);

//     println!("successfully here");
// }

// Fixture
fn keys() -> Vec<(PublicKey, SecretKey)> {
    let mut rng = StdRng::from_seed([0; 32]);
    (0..4).map(|_| generate_keypair(&mut rng)).collect()
}
// Fixture
fn committee() -> Committee {
    Committee {
        authorities: keys()
            .iter()
            .enumerate()
            .map(|(i, (id, _))| {
                let primary = PrimaryAddresses {
                    primary_to_primary: format!("127.0.0.1:{}", 100 + i).parse().unwrap(),
                    worker_to_primary: format!("127.0.0.1:{}", 200 + i).parse().unwrap(),
                };
                let workers = vec![(
                    0,
                    WorkerAddresses {
                        primary_to_worker: format!("127.0.0.1:{}", 300 + i).parse().unwrap(),
                        transactions: format!("127.0.0.1:{}", 400 + i).parse().unwrap(),
                        worker_to_worker: format!("127.0.0.1:{}", 500 + i).parse().unwrap(),
                    },
                )]
                .iter()
                .cloned()
                .collect();
                (
                    *id,
                    Authority {
                        stake: 1,
                        primary,
                        workers,
                    },
                )
            })
            .collect(),
    }
}
