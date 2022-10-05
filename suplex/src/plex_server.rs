// #![feature(trace_macros)]
use quick_xml::{events::Event, Reader};

use xml_struct_derive::XmlStruct;

use xml_struct::XmlStruct;
use xml_struct::XmlStructError;

use crate::SuplexError;

// trace_macros!(true);

#[derive(Default, Debug, Clone, XmlStruct)]
pub struct PlexServer {
    first_field: String,
    another_field: u64,
}

// trace_macros!(false);
