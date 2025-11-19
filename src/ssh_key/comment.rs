//! SSH key comment support.

use core::{
    convert::Infallible,
    fmt,
    str::{self, FromStr},
};

/// SSH key comment (e.g. email address of owner)
///
/// Comments may be found in both the binary serialization of  [`PrivateKey`] as well as the text
/// serialization of [`PublicKey`].
///
/// The binary serialization of [`PrivateKey`] stores the comment encoded as an [RFC4251]
/// `string` type which can contain arbitrary binary data and does not necessarily represent valid
/// UTF-8. To support round trip encoding of such comments.
///
/// To support round-trip encoding of such comments, this type also supports arbitrary binary data.
///
/// [RFC4251]: https://datatracker.ietf.org/doc/html/rfc4251#section-5
/// [`PrivateKey`]: crate::PrivateKey
/// [`PublicKey`]: crate::PublicKey
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Comment(Box<[u8]>);

impl AsRef<[u8]> for Comment {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Comment {
    fn as_ref(&self) -> &str {
        self.as_str_lossy()
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str_lossy())
    }
}

impl From<&[u8]> for Comment {
    fn from(bytes: &[u8]) -> Comment {
        bytes.to_owned().into()
    }
}

impl From<Vec<u8>> for Comment {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec.into_boxed_slice())
    }
}

impl Comment {
    /// Interpret the comment as raw binary data.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Is the comment empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the length of this comment in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Interpret the comment as a UTF-8 string.
    ///
    /// This is the maximal prefix of the comment which can be interpreted as valid UTF-8.
    /// TODO(tarcieri): precompute and store the offset which represents this prefix?
    pub fn as_str_lossy(&self) -> &str {
        for i in (1..=self.len()).rev() {
            if let Ok(s) = str::from_utf8(&self.0[..i]) {
                return s;
            }
        }

        ""
    }
}

#[cfg(test)]
mod tests {
    use super::Comment;

    #[test]
    fn as_str_lossy_ignores_non_utf8_data() {
        const EXAMPLE: &[u8] = b"hello world\xc3\x28";

        let comment = Comment::from(EXAMPLE);
        assert_eq!(comment.as_str_lossy(), "hello world");
    }
}
