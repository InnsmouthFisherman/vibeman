use std::fs;
use std::env;
use std::path::PathBuf;

pub struct ComandlineProcessor{
    pub path: PathBuf,
}

impl ComandlineProcessor{

    pub fn new() -> Self{
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn process(&mut self){
        self.get_dir();
    }

    fn get_dir(&mut self){
        match env::current_dir() {
            Ok(mut current_dir) => {
                current_dir.push("src\\files");
                self.path = current_dir;
            },
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
            }
        }
    }

    pub fn locate_directories(&mut self) -> PathBuf {
        let mut tracks: PathBuf = PathBuf::new();
        let path_to_tracks = fs::read_dir(&self.path).unwrap();

        for track in path_to_tracks {
            tracks = track.unwrap().path(); 
            break;
        }
        tracks
    }
}