//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        StrUtf8(::std::str::Utf8Error);
    }
    errors {
        Deprecated(feat: &'static str) {
            description("deprecated feature")
            display("feature '{}' has been deprecated", feat)
        }
        UnknownWireType(t: u8) {
            description("unknown wire type")
            display("wire type must be less than 6, found {}", t)
        }
        Varint {
            description("cannot decode varint")
        }
        ParseMessage(s: String) {
            description("error while parsing message")
            display("error while parsing message: {}", s)
        }
        Map(tag: u8) {
            description("unexpected map tag")
            display("expecting a tag number 1 or 2, got {}", tag)
        }
    }
}

unsafe impl Sync for Error {}

impl Into<::std::io::Error> for Error {
    fn into(self) -> ::std::io::Error {
        use std::io;
        match self {
            Error(ErrorKind::Io(x), _) => x,
            Error(ErrorKind::StrUtf8(x), _) => io::Error::new(io::ErrorKind::InvalidData, x),
            x => io::Error::new(io::ErrorKind::Other, x),
        }
    }
}
