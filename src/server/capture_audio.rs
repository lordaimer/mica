use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc::Sender;

pub async fn capture(pcm_sender: Sender<Vec<f32>>) {
    // println!("Starting microphone capture...");

    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device");
    let config = device.default_input_config().unwrap();

    // println!("Input device: {}", device.name().unwrap());
    // println!("Default input config: {:?}", config);

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let samples: Vec<f32> = data.to_vec();
            // Send to channel
            if let Err(e) = pcm_sender.blocking_send(samples) {
                eprintln!("Failed to send PCM data: {}", e);
            }
        },
        move |err| {
            eprintln!("Stream error: {}", err);
        },
        None,
    ).unwrap();

    stream.play().unwrap();

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
