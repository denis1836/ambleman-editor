use id3::{Tag, TagLike};

pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: String,
    pub genre: String,
    pub comment: String,
}

/// Clears all the metadata from the specified MP3 file.
/// If the file does not have an ID3 tag, a new empty one will be created and then cleared.
pub fn clear_metadata(file: &str) -> Result<(), id3::Error> {
    let mut tag = Tag::read_from_path(file).unwrap_or(Tag::new());

    tag.remove_title();
    tag.remove_artist();
    tag.remove_album();
    tag.remove_year();
    tag.remove_genre();
    tag.remove_comment(Option::None, Option::None);

    tag.write_to_path(file, id3::Version::Id3v24)
}

/// Writes the provided metadata to the specified MP3 file.
/// If the file does not have an ID3 tag, a new one will be created and then populated with the provided metadata.
pub fn write_metadata(file: &str, metadata: &Metadata) -> Result<(), id3::Error> {
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

/// Reads the metadata from the specified MP3 file and returns it as a `Metadata` struct.
/// If the file does not have an ID3 tag, a new empty one will be created and the returned metadata will contain "null" for all fields.
pub fn read_metadata(file: &str) -> Result<Metadata, id3::Error> {
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
