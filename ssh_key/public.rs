/// `sshsig` provides a general-purpose signature format based on SSH keys and
/// wire formats.
///
/// These signatures can be produced using `ssh-keygen -Y sign`. They're
/// encoded as PEM and begin with the following:
///
/// ```text
/// -----BEGIN SSH SIGNATURE-----
/// ```
///
/// See [PROTOCOL.sshsig] for more information.
///
/// # Usage
///
/// See [`PrivateKey::sign`] and [`PublicKey::verify`] for usage information.
/// 
#[cfg(feature = "alloc")]
mod rsa;

#[cfg(feature = "alloc")]
pub use self::{
    dsa::DsaPublicKey,
    opaque::{OpaquePublicKey, OpaquePublicKeyBytes},
    rsa::RsaPublicKey,
};

/// SSH public key.
///
/// # OpenSSH encoding
///
/// The OpenSSH encoding of an SSH public key looks like following:
///
/// ```text
/// ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILM+rvN+ot98qgEN796jTiQfZfG1KaT0PtFDJ/XFSqti user@example.com
/// ```
///
/// It consists of the following three parts:
///
/// 1. Algorithm identifier (in this example `ssh-ed25519`)
/// 2. Key data encoded as Base64
/// 3. [`Comment`] (optional): arbitrary label describing a key. Usually an email address
///
/// The [`PublicKey::from_openssh`] and [`PublicKey::to_openssh`] methods can be
/// used to decode/encode public keys, or alternatively, the [`FromStr`] and
/// [`ToString`] impls.
///
/// # `serde` support
///
/// When the `serde` feature of this crate is enabled, this type receives impls
/// of [`Deserialize`][`serde::Deserialize`] and [`Serialize`][`serde::Serialize`].
///
/// The serialization uses a binary encoding with binary formats like bincode
/// and CBOR, and the OpenSSH string serialization when used with
/// human-readable formats like JSON and TOML.
///
/// Note that since the `comment` is an artifact on the string serialization of
/// a public key, it will be implicitly dropped when encoding as a binary
/// format. To ensure it's always preserved even when using binary formats, you
/// will first need to convert the [`PublicKey`] to a string using e.g.
/// [`PublicKey::to_openssh`].
/// 
/// 
pub struct PublicKey {
    /// Key data.
    // pub(crate) key_data: KeyData,

    /// Comment on the key (e.g. email address)
    ///
    /// Note that when a [`PublicKey`] is serialized in a private key, the
    /// comment is encoded as an RFC4251 `string` which may contain arbitrary
    /// binary data, so `Vec<u8>` is used to store the comment to ensure keys
    /// containing such comments successfully round-trip.
    #[cfg(feature = "alloc")]
    pub(crate) comment: Comment,
}
