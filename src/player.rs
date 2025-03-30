use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone, druid::Data)]
pub struct Player {
    #[data(ignore)]
    pub current_track: PathBuf,
    pub is_playing: bool,
    sink: Arc<Mutex<Option<Sink>>>,
    _output_stream: Arc<Mutex<Option<OutputStream>>>,
    output_stream_handle: Arc<Mutex<Option<OutputStreamHandle>>>,
}

impl Player {
    pub fn new(track: PathBuf) -> Self {
        Self {
            current_track: track,
            is_playing: false,
            sink: Arc::new(Mutex::new(None)),
            _output_stream: Arc::new(Mutex::new(None)),
            output_stream_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn play(&mut self) {
        let mut output_guard = self._output_stream.lock().unwrap();
        let mut handle_guard = self.output_stream_handle.lock().unwrap();
        
        if output_guard.is_none() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            *output_guard = Some(stream);
            *handle_guard = Some(stream_handle);
        }
        let handle = handle_guard.as_ref().unwrap();

        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
        }

        let sink = Sink::try_new(handle).unwrap();
        let file = BufReader::new(File::open(&self.current_track).unwrap());
        let source = Decoder::new(file).unwrap();
        
        sink.append(source);
        *self.sink.lock().unwrap() = Some(sink);
        self.is_playing = true;
    }

    pub fn toggle_playback(&mut self) {
        if self.is_playing {
            self.pause();
        } else {
            self.play();
        }
    }

    fn pause(&mut self) {
        if let Some(sink) = &mut *self.sink.lock().unwrap() {
            if sink.is_paused() {
                sink.play();
                self.is_playing = true;
            } else {
                sink.pause();
                self.is_playing = false;
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &mut *self.sink.lock().unwrap() {
            sink.stop();
            self.is_playing = false;
        }
    }
}