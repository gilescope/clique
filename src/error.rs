use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

type Source = Box<dyn error::Error + Send + Sync>;

pub struct Error {
    kind: ErrorKind,
    source: Option<Source>,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Start,
    Join,
    BrokenPipe,
    UnexpectedRequestType,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, source: Option<Source>) -> Self {
        unimplemented!()
    }

    pub(crate) fn new_broken_pipe(source: Option<Source>) -> Self {unimplemented!()}

    pub(crate) fn new_join(source: Option<Source>) -> Self {unimplemented!()}

    pub(crate) fn new_unexpected_request(source: Option<Source>) -> Self {unimplemented!()}
}


impl From<ErrorKind> for Error {
    fn from(t: ErrorKind) -> Self {unimplemented!()}
}

impl From<(ErrorKind, Source)> for Error {
    fn from(t: (ErrorKind, Source)) -> Self {unimplemented!()}
}



impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_tuple("Error");
        f.field(&self.kind);
        if let Some(source) = &self.source {unimplemented!()}
        f.finish()
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for Error {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}