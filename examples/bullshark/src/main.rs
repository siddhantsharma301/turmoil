use std::net::{IpAddr, Ipv4Addr};
use tokio::sync::mpsc::{channel, Receiver};
use tracing::{info_span, Instrument};
use turmoil::{net, Builder};

// Narwhal imports
use config::Export as _;
use config::Import as _;
use config::{Committee, KeyPair, Parameters, WorkerId};
use consensus::Consensus;
use primary::Primary;
use store::Store;
use worker::Worker;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let mut sim = Builder::new().build();

    // sim.host("primary0", async move || {

    // });
}
