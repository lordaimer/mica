## Overview

**Mica** (Microphone Input Capture Application) is a cross-platform, lightweight Rust audio server.  
It captures live microphone input, encodes it with the Opus codec, and streams it over RTP to connected clients.

> **Note:** Mica depends on GStreamer. The installer script will set it up automatically.

---

## Installation (Recommended)

**Always install Mica system-wide using the official PowerShell installer.**  
Run the following **in an **Administrator** PowerShell window**:

```powershell
iex "& { $(irm https://github.com/lordaimer/mica/raw/main/scripts/Install-Mica.ps1) }"
```