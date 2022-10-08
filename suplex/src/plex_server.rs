use quick_xml::{events::Event, Reader};

use xml_struct_derive::XmlStruct;

use xml_struct::XmlStruct;
use xml_struct::XmlStructError;

#[derive(Default, Debug, Clone, XmlStruct)]
#[xmlElement = b"server"]
pub struct PlexServer {
    first_field: String,
    another_field: u64,
}
