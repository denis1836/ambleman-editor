use crate::log::log;
use crate::metadata::{Metadata, clear_metadata, read_metadata, write_metadata};
use gtk::prelude::*;
use relm4::gtk::subclass::dialog;
use relm4::{
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, Sender, SimpleComponent, gtk,
};

pub struct App {
    metadata: Metadata,
    file_path: String,
}

#[derive(Debug)]
pub enum AppMsg {
    Save,
    SaveConfirmed,
    Clear,
    ClearConfirmed,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
    gtk::Window {
            set_title: Some("Ambleman Editor"),
            set_default_size: (760, 480),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                //file picker bar
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_all: 12,
                    gtk::Button{
                        set_label: "...",
                        //connect_clicked => {
                        //}
                    },
                    gtk::Entry{
                        set_placeholder_text: Some("Choose a file..."),
                        //connect_changed => {
                    }
                }
            },
            //editor fields
            gtk::Box {
                //left horizontal box
                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 8,
                    //picture box
                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        gtk::Image{
                            set_size_request: (200, 200),
                            set_icon_name: Some("image-missing"),
                        },
                        //picture buttons
                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,
                            set_margin_all: 12,
                            gtk::Button{
                                set_label: "Delete Picture",
                                //connect_clicked => {
                            },
                            gtk::Button{
                                set_label: "Choose Picture",
                                //connect_clicked => {
                            }
                        }
                    },
                    //track numbers and years
                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            gtk::Entry{
                                set_placeholder_text: Some("Track Number"),
                                //connect_changed => {
                            },
                            gtk::Entry{
                                set_placeholder_text: Some("Track Total"),
                                //connect_changed => {
                            }
                        },
                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            gtk::Entry{
                                set_placeholder_text: Some("Disc Number"),
                                //connect_changed => {
                            },
                            gtk::Entry{
                                set_placeholder_text: Some("Disk Total"),
                                //connect_changed => {
                            }
                        },
                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            gtk::Entry{
                                set_placeholder_text: Some("Year"),
                                //connect_changed => {
                            },
                            gtk::Entry{
                                set_placeholder_text: Some("Release Year"),
                                //connect_changed => {
                            }
                        }
                    },
                    //lyrics box
                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        set_margin_all: 12,
                        gtk::Button{
                            set_label: "...",
                            //connect_clicked => {
                        },
                        gtk::Entry{
                            set_placeholder_text: Some("Lyrics file..."),
                            //connect_changed => {
                        }
                    },
                    //save and clear buttons
                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        set_margin_all: 12,
                        gtk::Button{
                            set_label: "Save",
                            //connect_clicked => {
                            //    write_metadata(&self.file_path, &self.metadata).unwrap();
                            //}
                        },
                        gtk::Button{
                            set_label: "Clear",
                            connect_clicked => {AppMsg::Clear},
                        }
                    }
                },
                //right vertical box
                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 8,
                    set_margin_all: 12,
                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        gtk::Label{
                            set_label: "Title",
                        },
                        gtk::Entry{
                            set_placeholder_text: Some("ex. Fade to Black"),
                            //connect_changed => {
                        },
                    },
                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        gtk::Label{
                            set_label: "Artist",
                        },
                        gtk::Entry{
                            set_placeholder_text: Some("ex. Metalica"),
                            //connect_changed => {
                        },
                    },
                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        gtk::Label{
                            set_label: "Album",
                        },
                        gtk::Entry{
                            set_placeholder_text: Some("ex. Ride the Lightning"),
                            //connect_changed => {
                        },
                    },
                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        gtk::Label{
                            set_label: "Genre",
                        },
                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,
                            gtk::Entry{
                                set_placeholder_text: Some("ex. Metal"),
                                //connect_changed => {
                            },
                            gtk::Image{
                                set_icon_name: Some("dialog-information-symbolic"),
                                set_tooltip_text: Some("If you have few genres, separate them with a semicolon."),
                            }
                        }
                    },
                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        gtk::Label{
                            set_label: "Comment",
                            set_align: gtk::Align::Start,
                        },
                        gtk::TextView{
                            set_size_request: (200, 130),
                            //connect_changed => {
                        }
                    }
                }
            }
        }
    }

    fn init(_: (), root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = App {
            file_path: String::new(),
            metadata: Metadata {
                title: None,
                artist: None,
                album: None,
                year: None,
                genre: None,
                comment: None,
                track: None,
                total_tracks: None,
                disc: None,
                total_discs: None,
                release_date: None,
                lyrics: None,
                cover: None,
            },
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Save => {
                //TODO: confirmation window
            }
            AppMsg::Clear => {
                //TODO: confirmation window
            }
            AppMsg::SaveConfirmed => {
                write_metadata(&self.file_path, &self.metadata).expect("Failed to write metadata");
            }
            AppMsg::ClearConfirmed => {
                clear_metadata(&self.file_path).expect("Failed to clear metadata");
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
