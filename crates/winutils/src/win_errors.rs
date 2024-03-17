use std::string::FromUtf16Error;

use widestring::error::ContainsNul;

#[derive(Debug)]
pub enum Error {
    Windows(windows::core::Error),
    WideString(ContainsNul<u16>),
    FromUtf16Error(FromUtf16Error),
}
impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        Error::Windows(e)
    }
}
impl From<ContainsNul<u16>> for Error {
    fn from(e: ContainsNul<u16>) -> Self {
        Error::WideString(e)
    }
}
impl From<FromUtf16Error> for Error {
    fn from(e: FromUtf16Error) -> Self {
        Error::FromUtf16Error(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;