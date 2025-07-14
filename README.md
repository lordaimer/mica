## Overview
MICA (Microphone Input Capture Application) is a crossplatform, lightweight, standalone Rust audio server.
it captures live microphone input, encodes it with the Opus codec, and streams it over RTP to connected clients.
___
## Instructions

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
___

## Dependencies: GStreamer & pkg-config (Windows)

To build this project **on Windows**, you must:

1. **Install GStreamer**

    * Download the **MSVC development installer** (not MinGW) from [https://gstreamer.freedesktop.org/download/](https://gstreamer.freedesktop.org/download/)
    * Install **both the runtime** and **development** packages (x86\_64)

2. **Install `pkg-config`**

    * Install via [MSYS2](https://www.msys2.org/) or use [WinPKG](https://github.com/pkgconf/pkgconf) or any other Windows pkg-config distribution.

3. **Set environment variables in PowerShell before building:**

   ```powershell
   $Env:PKG_CONFIG_PATH = "C:\Program Files\gstreamer\1.0\msvc_x86_64\lib\pkgconfig"
   $Env:LIB = "C:\Program Files\gstreamer\1.0\msvc_x86_64\lib;$Env:LIB"
   $Env:PATH = "C:\Program Files\gstreamer\1.0\msvc_x86_64\bin;$Env:PATH"
   ```
   Adjust paths if you installed to a custom location.

4. **Verify**

   ```powershell
   pkg-config --libs --cflags gstreamer-1.0
   ```

   Should output correct include & lib paths.

---

**Now** you can build:

```powershell
cargo build
```

---
