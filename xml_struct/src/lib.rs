use std::{convert::Infallible, num::ParseIntError};

#[derive(thiserror::Error, Debug)]
pub enum XmlStructError {
    #[error("Don't use this in production")]
    Temp,
    #[error("error reading XML data")]
    SomethingBad {
        #[from]
        source: quick_xml::Error,
    },
    #[error("error converting string to number")]
    ParseError {
        #[from]
        source: ParseIntError,
    },
    #[error("error converting string to number")]
    ImpossibleError {
        #[from]
        source: Infallible,
    },
}

pub type Result<T> = std::result::Result<T, XmlStructError>;

pub trait XmlStruct
where
    Self: Sized + Default,
{
    fn from_xml(xml_string: String) -> Result<Self>;
}
