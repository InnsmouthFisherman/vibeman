use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Clone, druid::Data)]
pub struct Player {
    #[data(ignore)]
    pub current_track: PathBuf,  // Храним как PathBuf
    pub is_playing: bool,
}

impl Player {
    pub fn new(track: PathBuf) -> Self {  // Принимаем PathBuf
        Self {
            current_track: track,
            is_playing: false,
        }
    }

    pub fn play(filename: &Path){
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = File::open(filename).unwrap();
        println!("{:?} is playing...", filename);
            
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
     
        sink.append(source);
        sink.sleep_until_end();
    }

}