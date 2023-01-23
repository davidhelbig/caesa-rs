use std::fmt::Display;

use ascii::{AsAsciiStrError, AsciiChar, AsciiString, IntoAsciiString};

pub mod cli;

#[derive(Debug)]
pub enum CaesarError {
    AsciiError(usize),
    KeyRange(i8),
}

impl Display for CaesarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CaesarError::*;
        match self {
            AsciiError(pos) => write!(f, "Non-ASCII char at pos {pos}"),
            KeyRange(value) => write!(f, "Key {value} is not in range 0..26"),
        }
    }
}

impl From<AsAsciiStrError> for CaesarError {
    fn from(value: AsAsciiStrError) -> Self {
        Self::AsciiError(value.valid_up_to())
    }
}

pub struct Key {
    value: i8,
}

impl Key {
    pub fn new(value: i8) -> Result<Self, CaesarError> {
        match value {
            0..=26 => Ok(Key { value }),
            _ => Err(CaesarError::KeyRange(value)),
        }
    }

    pub fn get_value(&self) -> i8 {
        self.value
    }
}

/// Something that implements Shift can be shifted, i.e. it can return
/// an instance of itself that was modified `by` another value.
pub trait Shift<U> {
    fn shift(&self, by: U) -> Self;
}

impl Shift<i8> for AsciiChar {
    fn shift(&self, by: i8) -> Self {
        let b = self.as_byte();
        match b {
            // ASCII uppercase alphabetic characters
            65..=90 => {
                let u = (b - 65) as i8;
                let u = ((u + by).rem_euclid(26)) + 65;
                AsciiChar::from_ascii(u).unwrap()
            }
            // ASCII lowercase alphabetic characters
            97..=122 => {
                let l = (b - 97) as i8;
                let l = ((l + by).rem_euclid(26)) + 97;
                AsciiChar::from_ascii(l).unwrap()
            }
            _ => self.clone(),
        }
    }
}

impl Shift<i8> for AsciiString {
    fn shift(&self, by: i8) -> Self {
        let shifted: AsciiString = self.into_iter().map(|char| char.shift(by)).collect();
        shifted
    }
}

/// Encrypt an ASCII string by shifting the alphabetic characters by
/// the indicated key/offset.
/// Whitespace is ignored, as are non-alphabetic characters.
///
/// ```
/// use caesar::{encrypt, Key};
///
/// let text = "abc def.";
/// let key = Key::new(3).unwrap();
/// let shifted = encrypt(text, &key).unwrap();
/// assert_eq!("def ghi.", shifted);
/// ```
pub fn encrypt(text: &str, key: &Key) -> Result<String, AsAsciiStrError> {
    let ascii_string = text.into_ascii_string().map_err(|e| e.ascii_error())?;

    let encrypted: AsciiString = ascii_string.shift(key.get_value());

    Ok(String::from(encrypted))
}

/// Decrypt a message with a given key.
/// Whitespace is ignored, as are non-alphabetic characters.
///
/// ```
/// use caesar::{decrypt, Key};
///
/// let encrypted = "def ghi.";
/// let key = Key::new(3).unwrap();
/// let shifted = decrypt(encrypted, &key).unwrap();
/// assert_eq!("abc def.", shifted);
/// ```
pub fn decrypt(text: &str, key: &Key) -> Result<String, AsAsciiStrError> {
    let ascii_string = text.into_ascii_string().map_err(|e| e.ascii_error())?;

    let encrypted: AsciiString = ascii_string.shift(-key.get_value());

    Ok(String::from(encrypted))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_shift_by_3() {
        let shifted = "Vdpsoh Whaw + qxpehuv : 1234567890 ;-)";
        let key = Key::new(3).unwrap();

        assert_eq!(
            encrypt("Sample Text + numbers : 1234567890 ;-)", &key).unwrap(),
            shifted
        );
    }
}
