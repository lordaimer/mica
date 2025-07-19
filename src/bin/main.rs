use clap::{Parser, Subcommand};
use libloading::{Library, Symbol};
use std::{ffi::CString, path::Path, process};

/// MICA (Microphone Input Capture Application)
#[derive(Parser)]
#[command(
    name = "mica",
    version,
    about = r#"MICA is a simple, standalone tool that captures your microphone input
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

fn gstreamer_installed() -> bool {
    // Try gst-launch-1.0
    if process::Command::new("gst-launch-1.0")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return true;
    }
    // Fallback to gst-launch-1.0.exe
    process::Command::new("gst-launch-1.0.exe")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn main() {
    let cli = Cli::parse();

    if !gstreamer_installed() {
        panic!("[error] gstreamer runtime is not installed on system. cannot continue")
    }

    // SAFELY load the DLL
    let lib = unsafe {
        Library::new(Path::new("libmica.dll"))
            .expect("Failed to load libmica.dll, Make sure to place libmica.dll alongside mica.exe\
            ")
    };

    match cli.command {
        Command::Serve { port } => unsafe {
            // load and call run_server
            let run_server: Symbol<unsafe extern "C" fn(u16)> =
                lib.get(b"run_server").expect("Symbol run_server not found");
            run_server(port);
        },

        Command::Connect { host, port } => {
            // parse host[:port]
            let (address, host_port) = if let Some((h, p)) = host.split_once(':') {
                let p = p.parse::<u16>().unwrap_or_else(|_| {
                    eprintln!("[mica] Invalid port '{}', using default 7373", p);
                    7373
                });
                (h.to_string(), p)
            } else {
                (host, 7373)
            };
            let final_port = port.unwrap_or(host_port);
            let final_address = format!("{}:{}", address, final_port);
            let c_addr = CString::new(final_address).expect("Failed to convert CString");

            unsafe {
                // load and call run_client
                let run_client: Symbol<unsafe extern "C" fn(*const i8)> =
                    lib.get(b"run_client").expect("Symbol run_client not found");
                run_client(c_addr.as_ptr() as *const i8);
            }
        }
    }
}
