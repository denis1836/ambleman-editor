use id3::{Tag, TagLike, Timestamp};

pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub comment: Option<String>,
    pub track: Option<u32>,
    pub total_tracks: Option<u32>,
    pub disc: Option<u32>,
    pub total_discs: Option<u32>,
    pub release_date: Option<Timestamp>,
    pub lyrics: Option<String>,
    pub cover: Option<Vec<u8>>,
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
    tag.remove_track();
    tag.remove_total_tracks();
    tag.remove_disc();
    tag.remove_total_discs();
    tag.remove_date_released();
    tag.remove_all_lyrics();
    tag.remove_all_pictures();

    tag.write_to_path(file, id3::Version::Id3v24)
}

/// Writes the provided metadata to the specified MP3 file.
/// If the file does not have an ID3 tag, a new one will be created and then populated with the provided metadata.
pub fn write_metadata(file: &str, metadata: &Metadata) -> Result<(), id3::Error> {
    let mut tag = Tag::read_from_path(file).unwrap_or(Tag::new());

    tag.set_title(
        metadata
            .title
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("null"),
    );
    //TODO: adding multiple artists
    tag.set_artist(
        metadata
            .artist
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("null"),
    );
    tag.set_album(
        metadata
            .album
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("null"),
    );
    if let Some(year) = metadata.year {
        tag.set_year(year as i32);
    }
    tag.set_genre(
        metadata
            .genre
            .as_ref()
            .map(|s| s.as_str()) //i fucking hate rust formating and rust-fmt
            .unwrap_or("null"),
    );
    tag.add_frame(id3::frame::Comment {
        lang: "eng".to_string(),
        description: "Comment".to_string(),
        text: metadata
            .comment
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("null")
            .to_string(),
    });
    tag.set_track(metadata.track.unwrap_or(0));
    tag.set_total_tracks(metadata.total_tracks.unwrap_or(0));
    tag.set_disc(metadata.disc.unwrap_or(0));
    tag.set_total_discs(metadata.total_discs.unwrap_or(0));
    tag.set_date_released(metadata.release_date.unwrap_or(Timestamp::default()));
    if let Some(lyrics) = &metadata.lyrics {
        tag.add_frame(id3::frame::Lyrics {
            lang: "eng".to_string(),
            description: "Lyrics".to_string(),
            text: lyrics.clone(),
        });
    }

    tag.write_to_path(file, id3::Version::Id3v24)
}

/// Reads the metadata from the specified MP3 file and returns it as a `Metadata` struct.
/// If the file does not have an ID3 tag, a new empty one will be created and the returned metadata will contain "null" for all fields.
pub fn read_metadata(file: &str) -> Result<Metadata, id3::Error> {
    let tag = Tag::read_from_path(file)?;

    let metadata = Metadata {
        title: tag.title().as_ref().map(|s| s.to_string()),
        artist: tag.artist().as_ref().map(|s| s.to_string()),
        album: tag.album().as_ref().map(|s| s.to_string()),
        year: tag.year().map(|y: i32| y as u32).or_else(|| Some(0)),
        genre: tag.genre().as_ref().map(|s| s.to_string()),
        comment: tag.comments().next().map(|c| c.text.clone()),
        track: tag.track().or_else(|| Some(0)),
        total_tracks: tag.total_tracks().or_else(|| Some(0)),
        disc: tag.disc().or_else(|| Some(0)),
        total_discs: tag.total_discs().or_else(|| Some(0)),
        release_date: tag.date_released().or_else(|| Some(Timestamp::default())),
        lyrics: None,
        cover: None,
    };

    Ok(metadata)
}
