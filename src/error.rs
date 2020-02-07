use crate::common::*;

#[derive(Debug)]
pub enum Error {
  Encoding(bendy::encoding::Error),
  Message(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl serde::ser::Error for Error {
  fn custom<T: Display>(message: T) -> Self {
    Error::Message(message.to_string())
  }
}

impl Display for Error {
  fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
    formatter.write_str(std::error::Error::description(self))
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match self {
      Self::Message(message) => message,
      Self::Encoding(source) => "encoding", // TODO: what goes here?
    }
  }
}

impl From<bendy::encoding::Error> for Error {
  fn from(source: bendy::encoding::Error) -> Self {
    Self::Encoding(source)
  }
}
