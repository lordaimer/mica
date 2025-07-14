mod server;
mod client;
use clap::{Parser, Subcommand};

/// MICA (Microphone Input Capture Application)
#[derive(Parser)]
#[command(
    name = "mica",
    version,
    about =
    r#"MICA is a simple, standalone tool that captures your microphone input
and streams it over the network in real time."#,
    author = "Arane Aimer"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the microphone server on port 7373 (default)
    Serve {
        /// Port to bind the server to
        #[arg(short = 'p', long = "port", default_value = "7373")]
        port: u16,
    },

    /// Connect to a server and play the audio stream
    Connect {
        /// Host and port to connect to, e.g. 192.168.1.10:7373
        address: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Serve { port } => {
            server::start_server(port);
        },
        Command::Connect { address } => {
            client::connect(&address).await;
        },
    }
}