use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::Accessor;
use lofty::probe::Probe;

#[derive(Clone)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: core::time::Duration,
    pub path: String,
}

impl Track {
    pub fn new(path: &str) -> Self {
        let tagged_file = Probe::open(path)
            .expect("Bad path provided!")
            .read()
            .expect("Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(tag) => tag,
            None => panic!("No tag found in file!")
        };
        Self {
            title: tag.title().as_deref().unwrap_or("Unknown Track Title").to_string(),
            artist: tag.artist().as_deref().unwrap_or("Unknown Artist").to_string(),
            album: tag.album().as_deref().unwrap_or("Unknown Album").to_string(),
            duration: tagged_file.properties().duration(),
            path: path.to_string(),
        }
    }

    pub fn display_info(&self) {
        println!("Title: {}", self.title);
        println!("Artist: {}", self.artist);
        println!("Album: {}", self.album);
        println!("Duration: {:.2?}", self.duration);
        println!("Path: {}", self.path);
    }
}