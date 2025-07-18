mod rtsp;

/// Starts the server: uses a GStreamer launch string for mic → Opus → RTP.
pub fn start_server(port: u16) {
    let launch = "autoaudiosrc ! audioconvert ! audioresample ! opusenc ! rtpopuspay name=pay0 pt=96";
    rtsp::serve_rtsp(launch, port)
        .expect("RTSP server failed");
}
