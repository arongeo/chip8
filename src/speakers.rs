// Copyright 2023 - https://github.com/arongeo
//
// Chip-8 Interpreter written in Rust
// https://github.com/arongeo/chip8
// 
// speakers.rs file
//

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub struct Speaker {
    pub sink: Sink, // remove pub after done testing
    stream: OutputStream,
}

impl Speaker {
    pub fn new() -> Self {
        let (stream_creator, stream_handle) = match OutputStream::try_default() {
            Ok((str, str_hand)) => (str, str_hand),
            Err(error) => panic!("Failed to create sound stream handle! ERROR: {}", error),
        };
        let sink_creator = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(error) => panic!("Failed to create sound sink! ERROR: {}", error),
        };
        let source = SineWave::new(700.0);
        sink_creator.append(source);
        sink_creator.set_volume(0.5);
        sink_creator.pause();
        Self {
            sink: sink_creator,
            stream: stream_creator,
        }
    }

    pub fn sound(&mut self) {
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.pause();
    }

    pub fn volume(&mut self, vol: f32) {
        self.sink.set_volume(vol);
    }

    pub fn check_st(&mut self, st: u8) {
        if st > 0 {
            if self.sink.is_paused() == true {
                self.sink.play();
            }
        } else {
            if self.sink.is_paused() == false {
                self.sink.pause();
            }
        }
    }
}
