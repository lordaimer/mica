mod capture_audio;
mod encode_pcm;
use tokio::sync::mpsc;

pub async fn run() {
    use encode_pcm::Encoder;
    use opus::Channels;

    // Channel for PCM chunks
    let (tx, mut rx) = mpsc::channel::<Vec<f32>>(32);

    // Start capture in background
    tokio::spawn(async move {
        capture_audio::capture(tx).await;
    });

    let mut encoder = Encoder::new(48_000, Channels::Mono, 960);

    while let Some(pcm) = rx.recv().await {
        let opus_frame = encoder.encode(&pcm);
        //  println!("Encoded Opus frame size: {}", opus_frame.len());
    }
}
