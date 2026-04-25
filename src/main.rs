use id3::{Tag, TagLike};

struct Metadata {
    title: String,
    artist: String,
    album: String,
    year: String,
    genre: String,
    comment: String,
}

fn clear_metadata(file: &str) -> Result<(), id3::Error> {
    let mut tag = Tag::read_from_path(file).unwrap_or(Tag::new());

    tag.remove_title();
    tag.remove_artist();
    tag.remove_album();
    tag.remove_year();
    tag.remove_genre();
    tag.remove_comment(Option::None, Option::None);

    tag.write_to_path(file, id3::Version::Id3v24)
}

fn write_metadata(file: &str, metadata: &Metadata) -> Result<(), id3::Error> {
    let mut tag = Tag::read_from_path(file).unwrap_or(Tag::new());

    tag.set_title(&metadata.title);
    tag.set_artist(&metadata.artist);
    tag.set_album(&metadata.album);
    if let Ok(year) = metadata.year.parse::<i32>() {
        tag.set_year(year);
    }
    tag.set_genre(&metadata.genre);
    tag.add_frame(id3::frame::Comment {
        lang: "eng".to_string(),
        description: "Comment".to_string(),
        text: metadata.comment.clone(),
    });

    tag.write_to_path(file, id3::Version::Id3v24)
}

fn read_metadata(file: &str) -> Result<Metadata, id3::Error> {
    let tag = Tag::read_from_path(file)?;

    let metadata = Metadata {
        title: tag.title().unwrap_or("null").to_string(),
        artist: tag.artist().unwrap_or("null").to_string(),
        album: tag.album().unwrap_or("null").to_string(),
        year: tag
            .year()
            .map(|y| y.to_string())
            .unwrap_or("null".to_string()),
        genre: tag.genre().unwrap_or("null").to_string(),
        comment: tag
            .comments()
            .next()
            .map(|c| c.text.clone())
            .unwrap_or("null".to_string()),
    };

    Ok(metadata)
}

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
    Ok(())
}
