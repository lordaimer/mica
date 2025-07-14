use opus::{Application, Channels, Encoder as OpusEncoder};

pub struct Encoder {
    encoder: OpusEncoder,
    frame_size: usize,
}

impl Encoder {
    /// Create a new Opus encoder.
    /// `sample_rate` must be 8000, 12000, 16000, 24000, or 48000.
    /// `channels` is usually 1 (mono) for mic.
    pub fn new(sample_rate: u32, channels: Channels, frame_size: usize) -> Self {
        let encoder = OpusEncoder::new(sample_rate, channels, Application::Audio)
            .expect("Failed to create Opus encoder");
        Self {
            encoder,
            frame_size,
        }
    }

    /// Encode PCM samples to Opus frame.
    /// Input: slice of f32 PCM samples.
    /// Output: Vec<u8> Opus encoded data.
    pub fn encode(&mut self, pcm: &[f32]) -> Vec<u8> {
        assert_eq!(pcm.len(), self.frame_size, "PCM input must match frame_size");
        let mut output = vec![0u8; 4000];
        let encoded_len = self.encoder
            .encode_float(pcm, &mut output)
            .expect("Failed to encode Opus frame");
        output.truncate(encoded_len);
        output
    }
}
