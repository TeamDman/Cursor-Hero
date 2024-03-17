use std::{rc::Rc, string::FromUtf16Error};

use widestring::error::ContainsNul;
use windows::Win32::Foundation::BOOL;

#[derive(Debug, Clone)]
pub enum Error {
    Windows(windows::core::Error),
    WideString(ContainsNul<u16>),
    FromUtf16Error,
    Described(Rc<Error>, String),
    ImageContainerNotBigEnough,
}
impl Error {
    pub fn from_win32() -> Self {
        Error::Windows(windows::core::Error::from_win32())
    }
    pub fn with_description(self, description: String) -> Self {
        Error::Described(Rc::new(self), description)
    }
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
    fn from(_e: FromUtf16Error) -> Self {
        Error::FromUtf16Error
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait WithDescription<T> {
    fn with_description(self, description: String) -> Result<T>;
}
impl<T> WithDescription<T> for windows::core::Result<T> {
    fn with_description(self, description: String) -> Result<T> {
        self.map_err(|e| Error::Windows(e).with_description(description))
    }
}

pub trait OkWithDescription<T> {
    fn ok_with_description(self, description: String) -> Result<T>;
}
impl OkWithDescription<()> for BOOL {
    fn ok_with_description(self, description: String) -> Result<()> {
        self.ok().map_err(|e| Error::Windows(e).with_description(description))
    }
}