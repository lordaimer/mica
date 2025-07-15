use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer_rtsp_server as rtsp;
use rtsp::prelude::*;
use glib::MainLoop;

/// Serve an RTSP stream at `/` on `port` using a simple launch string.
pub fn serve_rtsp(launch: &str, port: u16) -> Result<()> {
    // 0) Initialize GStreamer
    gst::init().context("Failed to initialize GStreamer")?;

    // 1) Create and configure the RTSP server
    let server = rtsp::RTSPServer::new();
    server.set_service(&port.to_string()); // RTSPServerExt

    // 2) Grab server mount points
    let mounts = server
        .mount_points()                     // RTSPServerExt
        .context("Failed to get mount points")?;

    // 3) Create a media factory from a launch string
    let factory = rtsp::RTSPMediaFactory::new();
    factory.set_shared(true);              // RTSPMediaFactoryExt
    factory.set_launch(&format!("( {} )", launch));

    // 4) Mount at / — pass factory by value!
    mounts.add_factory("/", factory); // RTSPMountPointsExt

    // 5) Attach and run
    server
        .attach(None)                      // RTSPServerExtManual
        .context("Failed to attach RTSP server")?;
    println!("RTSP server running at rtsp://0.0.0.0:{}/", port);

    // 6) Start the GLib main loop so RTSP + pipeline stay alive
    let main_loop = MainLoop::new(None, false);
    main_loop.run();

    Ok(())
}
