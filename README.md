# Overview
MICA (Microphone Input Capture Application) is a crossplatform, lightweight, standalone Rust audio server.
it captures live microphone input, encodes it with the Opus codec, and streams it over RTP to connected clients.

# Instructions

```bash
Usage
  mica [COMMAND] [OPTIONS]

Commands:
  serve                Start the microphone server on port 7373 (default)
  connect <host:port>  Connect to a running server and play the audio stream

Options:
  -h, --help           Show help information
  -v, --version        Show version information

Examples:
  mica serve
  mica connect 192.168.1.5:7373
```