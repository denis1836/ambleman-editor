mod metadata;
mod ui;

use crate::metadata::{Metadata, clear_metadata, read_metadata, write_metadata};

fn main() -> Result<(), id3::Error> {
    fn debug_metadata(song: &str) {
        let metadata = read_metadata(song).unwrap();
        println!("Tytuł: {}", metadata.title);
        println!("Artysta: {}", metadata.artist);
        println!("Album: {}", metadata.album);
        println!("Rok: {}", metadata.year);
        println!("Gatunek: {}", metadata.genre);
        println!("Komentarz: {}", metadata.comment);

        println!("EOF\n");
    }

    let song = "testfiles/test.mp3";

    let song_data = Metadata {
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        album: "Test Album".to_string(),
        year: "2023".to_string(),
        genre: "Test Genre".to_string(),
        comment: "Best song ever!".to_string(),
    };
    write_metadata(song, &song_data)?;

    debug_metadata(song);
    clear_metadata(song)?;
    debug_metadata(song);
    write_metadata(song, &song_data)?;
    println!("You can now check the metadata by using any media player.");
    Ok(())
}
