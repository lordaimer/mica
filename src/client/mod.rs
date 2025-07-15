use anyhow::{Context, Result};
use gstreamer as gst;
use gst::prelude::*;

pub async fn connect(address: &str) {
    println!("[client] connecting to {}", address);
    if let Err(e) = run(address) {
        eprintln!("[client] error: {}", e);
    }
}

fn run(address: &str) -> Result<()> {
    // 1) Initialize GStreamer
    gst::init().context("Failed to initialize GStreamer")?;

    // 2) Build pipeline with parse_launch (caps + depay + decode + convert + sink)
    let uri = format!("rtsp://{}", address);
    let launch = format!(
        "uridecodebin uri={} ! audioconvert ! autoaudiosink",
        uri
    );


    let element = gst::parse::launch(&launch)
        .context("Failed to parse launch string")?;
    let pipeline = element
        .downcast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Expected a gst::Pipeline"))?;

    // 3) Start playback
    pipeline.set_state(gst::State::Playing)?;
    println!("[client] connection successful, playing audio…");

    // 4) Wait for EOS or Error
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("[client] stream ended (EOS)");
                break;
            }
            MessageView::Error(err) => {
                let src = err
                    .src()
                    .map(|s| s.path_string())
                    .unwrap_or_else(|| "<unknown>".into());
                eprintln!(
                    "[client] Error from {}: {} ({:?})",
                    src,
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => {} // ignore everything else
        }
    }

    // 5) Clean up
    pipeline.set_state(gst::State::Null)?;
    Ok(())
}
