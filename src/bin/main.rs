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
        /// Host (with optional :port). Example: 192.168.1.6:7373 or 192.168.1.6 (uses 7373 as default port)
        host: String,

        /// Port override
        #[arg(short = 'p', long = "port", default_value = "7373")]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Serve { port } => {
            mica_lib::server::start_server(port);
        },
        Command::Connect { host, port } => {
            let (address, host_port) = if let Some((h, p)) = host.split_once(':') {
                let p = match p.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("[mica] Invalid port '{}', using default 7373", p);
                        7373
                    }
                };
                (h.to_string(), p)
            } else {
                (host, 7373)
            };

            let final_port = port.unwrap_or(host_port);
            let final_address = format!("{}:{}", address, final_port);
            mica_lib::client::connect(&final_address).await;
        }
    }
}