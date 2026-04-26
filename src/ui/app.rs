use crate::log::log;
use crate::metadata::{Metadata, clear_metadata, read_metadata, write_metadata};
use gtk::prelude::*;
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, gtk};

pub struct App {
    metadata: Metadata,
    file_path: String,
}

#[derive(Debug)]
pub enum AppMsg {
    LoadFile,
    Save,
    Clear,
    OpenFilePicker,
    FileSelected(String),
    TitleChanged(String),
    ArtistChanged(String),
    AlbumChanged(String),
    YearChanged(String),
    GenreChanged(String),
    CommentChanged(String),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
    gtk::Window {
            set_title: Some("MP3 Metadata Editor"),
            set_default_size: (400, 300),
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 8,
                set_margin_all: 12,

                gtk::Button {
                    set_label: "Search...",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::OpenFilePicker);
                    }
                },
                gtk::Button {
                    set_label: "Load Metadata",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::LoadFile);
                    }
                },
                gtk::Entry {
                    set_placeholder_text: Some("Title"),
                    connect_changed[sender] => move |e| {
                        sender.input(AppMsg::TitleChanged(e.text().into()));
                    }
                },
                gtk::Entry {
                    set_placeholder_text: Some("Artist"),
                    connect_changed[sender] => move |e| {
                        sender.input(AppMsg::ArtistChanged(e.text().into()));
                    }
                },
                gtk::Entry {
                    set_placeholder_text: Some("Album"),
                    connect_changed[sender] => move |e| {
                        sender.input(AppMsg::AlbumChanged(e.text().into()));
                    }
                },
                gtk::Entry {
                    set_placeholder_text: Some("Year"),
                    connect_changed[sender] => move |e| {
                        sender.input(AppMsg::YearChanged(e.text().into()));
                    }
                },
                gtk::Entry {
                    set_placeholder_text: Some("Genre"),
                    connect_changed[sender] => move |e| {
                        sender.input(AppMsg::GenreChanged(e.text().into()));
                    }
                },
                gtk::Entry {
                    set_placeholder_text: Some("Comment"),
                    connect_changed[sender] => move |e| {
                        sender.input(AppMsg::CommentChanged(e.text().into()));
                    }
                },
                gtk::Button {
                    set_label: "Save",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Save);
                    }
                },
                gtk::Button {
                    set_label: "Clear",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Clear);
                    }
                },
            },
        }
    }

    fn init(_: (), root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = App {
            file_path: String::new(),
            metadata: Metadata {
                title: String::new(),
                artist: String::new(),
                album: String::new(),
                year: String::new(),
                genre: String::new(),
                comment: String::new(),
            },
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::TitleChanged(s) => self.metadata.title = s,
            AppMsg::ArtistChanged(s) => self.metadata.artist = s,
            AppMsg::AlbumChanged(s) => self.metadata.album = s,
            AppMsg::YearChanged(s) => self.metadata.year = s,
            AppMsg::GenreChanged(s) => self.metadata.genre = s,
            AppMsg::CommentChanged(s) => self.metadata.comment = s,
            AppMsg::LoadFile => {
                if let Ok(metadata) = read_metadata(&self.file_path) {
                    self.metadata = metadata;
                }
            }
            AppMsg::Save => {
                if let Err(e) = write_metadata(&self.file_path, &self.metadata) {
                    log('E', &format!("Failed to save metadata: {}", e));
                }
            }
            AppMsg::Clear => {
                if let Err(e) = clear_metadata(&self.file_path) {
                    log('E', &format!("Failed to clear metadata: {}", e));
                } else {
                    self.metadata = Metadata {
                        title: String::new(),
                        artist: String::new(),
                        album: String::new(),
                        year: String::new(),
                        genre: String::new(),
                        comment: String::new(),
                    };
                }
            }
            AppMsg::OpenFilePicker => {
                let sender = _sender.clone();
                relm4::spawn(async move {
                    if let Some(path) = rfd::AsyncFileDialog::new()
                        .add_filter("MP3", &["mp3"])
                        .pick_file()
                        .await
                    {
                        sender.input(AppMsg::FileSelected(
                            path.path().to_string_lossy().to_string(),
                        ));
                    }
                });
            }
            AppMsg::FileSelected(path) => {
                self.file_path = path;
                if let Ok(metadata) = read_metadata(&self.file_path) {
                    self.metadata = metadata;
                }
            }
        }
    }
}

///Starts the UI.
pub fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    log('I', "Starting UI...");
    let app = RelmApp::new("com.ambleman.editor");
    app.run::<App>(());
    Ok(())
}
