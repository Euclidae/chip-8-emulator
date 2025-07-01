// Not sure if this is the best way to do it but it should work.
// Rodio docs: https://docs.rs/rodio/0.13.0/rodio/

use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct Audio {
    _stream: OutputStream,
    sink: Sink,
}

impl Audio {
    pub fn new() -> Result<Audio, String> {
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
        let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

        let source = rodio::source::SineWave::new(440.0)
            .take_duration(Duration::from_secs_f32(0.05))
            .repeat_infinite();
        sink.append(source);
        sink.pause();

        Ok(Audio { _stream, sink })
    }

    pub fn new_silent() -> Audio {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.pause();
        Audio { _stream, sink }
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }
}
