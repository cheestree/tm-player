use crate::track::track::Track;
use rodio::{OutputStream, Sink};
use std::io;
use std::sync::{Arc, Mutex};

pub struct AudioState {
    pub tracks: Vec<Track>,
    current_track: usize,
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioState {
    pub fn new(tracks: Vec<Track>, stream: OutputStream, sink: Arc<Mutex<Sink>>) -> Self {
        Self {
            tracks,
            current_track: 0,
            _stream: stream,
            sink,
        }
    }

    pub fn play_track(&mut self, track: &Track) {
        let sink = self.sink.lock().unwrap();
        let file = std::fs::File::open(&track.path).expect("open audio file");
        let source = rodio::Decoder::new(io::BufReader::new(file)).expect("decode audio file");
        sink.stop();
        sink.append(source);
        sink.play();
    }

    pub fn toggle_pause(&mut self) {
        let sink = self.sink.lock().unwrap();
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }

    pub fn current_track(&self) -> usize {
        self.current_track
    }

    pub fn set_current_track(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.current_track = index;
        }
    }
}
