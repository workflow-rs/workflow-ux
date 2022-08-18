use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Base58 decode error: {0}")]
    Base58Decode(#[from] bs58::decode::Error),
    #[error("Invalid buffer size")]
    InvalidBufferSize,
}

#[repr(transparent)]
#[derive(Clone,Copy,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct Id(pub(crate) [u8; 8]);

// Dom Element Id
impl Id {
    pub fn new() -> Id {
        Id::new_from_slice(&rand::random::<[u8; 8]>())
    }

    pub fn new_from_slice(vec: &[u8]) -> Self {
        Self(
            <[u8; 8]>::try_from(<&[u8]>::clone(&vec)).expect("Error: invalid slice size for id"),
        )
    }

    pub fn to_bytes(self) -> [u8; 8] {
        self.0
    }

}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Id {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > std::mem::size_of::<Id>()*2 {
            return Err(Error::InvalidBufferSize);
        }
        let vec = bs58::decode(s)
            .into_vec()?;
        if vec.len() != std::mem::size_of::<Id>() {
            Err(Error::InvalidBufferSize)
        } else {
            Ok(Id::new_from_slice(&vec))
        }
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Id::from_str(s)
    }
}

