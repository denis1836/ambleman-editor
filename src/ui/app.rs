use crate::metadata::{Metadata, clear_metadata, read_metadata, write_metadata};
use relm4::{ComponentParts, ComponentSender, SimpleComponent, gtk, view};

pub struct App {
    metadata: Metadata,
}
