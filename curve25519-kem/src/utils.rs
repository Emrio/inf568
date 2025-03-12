use std::{io, str::FromStr, usize};

#[derive(Clone)]
pub struct Bytes<const N: usize>(pub [u8; N]);

// inspired from Thomas Sauvage's work
impl<const N: usize> FromStr for Bytes<N> {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s)
            .map_err(|_| {
                std::io::Error::new(io::ErrorKind::InvalidInput, "Hexadecimal string expected")
            })?
            .try_into()
            .and_then(|v| Ok(Self(v)))
            .map_err(|_| {
                std::io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid length. Expected {} bytes, got {}", N, s.len() / 2),
                )
            })
    }
}
