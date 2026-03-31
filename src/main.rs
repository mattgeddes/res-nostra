use clap::Parser;
use nostr_relay_builder::prelude::*;
use rustls::crypto::CryptoProvider;
use rustls::crypto::ring::default_provider;
use std::net::IpAddr;
use std::time::Duration;
use tracing::info;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[arg(long, short = 'a', default_value_t = [0,0,0,0].into())]
    listen_addr: IpAddr,
    #[arg(long, short = 'p', default_value_t = 8080)]
    listen_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    CryptoProvider::install_default(default_provider()).unwrap();

    let cli = Cli::parse();

    let tor = RelayBuilderHiddenService::new("res-nostra-hs");
    /*let nip42 = RelayBuilderNip42 {
        mode: RelayBuilderNip42Mode::Both,
    };*/
    let builder = RelayBuilder::default()
        .tor(tor)
        //.nip42(nip42)
        .port(cli.listen_port)
        .addr(cli.listen_addr);

    let relay = LocalRelay::new(builder);

    relay.run().await?;

    info!("Url: {}", relay.url().await);
    info!("Hidden service: {:?}", relay.hidden_service().await?);

    // Keep up the program
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
