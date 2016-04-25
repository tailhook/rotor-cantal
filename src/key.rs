///
/// This contains CBOR-encoded key-value pairs
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(Option<Box<[u8]>>);

impl Key {
    pub fn as_bytes<'x>(&'x self) -> &'x [u8] {
        self.0.as_ref().map(|x| &x[..]).unwrap_or(b"")
    }
}

mod serde {
    use std::io::Cursor;
    use probor::{Decodable, Decoder, DecodeError, Input, Config};
    use probor::{Encodable, Encoder, EncodeError, Output};
    use Key;

    fn validate_key(val: &[u8]) -> Result<(), &'static str> {
        let mut d = Decoder::new(Config::default(), Cursor::new(val));
        let num = try!(d.object().map_err(|_| "Invalid key"));
        for _ in 0..num {
            // TODO(tailhook) other types may work in future
            try!(d.text_borrow().map_err(|_| "Invalid key"));
            try!(d.text_borrow().map_err(|_| "Invalid key"));
        }
        if d.into_reader().position() as usize != val.len() {
            return Err("Invalid key: extra data");
        }
        return Ok(());
    }

    impl Decodable for Key {
        fn decode_opt<R:Input>(d: &mut Decoder<R>)
            -> Result<Option<Self>, DecodeError>
        {
            let value = try!(d.bytes().map_err(|e|
                DecodeError::WrongType("bytes expected", e)));
            try!(validate_key(&value[..]).map_err(|e|
                DecodeError::WrongValue(e)));
            Ok(Some(Key(Some(value.into_boxed_slice()))))
        }
    }

    impl Encodable for Key {
        fn encode<W:Output>(&self, e: &mut Encoder<W>)
            -> Result<(), EncodeError>
        {
            e.bytes(self.as_bytes())
        }
    }
}

mod std_trait {
    use std::fmt::{Debug, Formatter, Error};
    use std::io::Cursor;
    use cbor::{Decoder, Config};
    use Key;

    impl Debug for Key {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
        {
            let b = if let Some(ref b) = self.0 { b } else {
                try!(write!(f, "Key {{}}"));
                return Ok(());
            };
            let mut d = Decoder::new(Config::default(), Cursor::new(&b[..]));
            try!(write!(f, "Key {{"));
            let num = try!(d.object()
                .map_err(|_| Error));
            for _ in 0..num {
                // TODO(tailhook) other types may work in future
                try!(write!(f, "{}: ",
                    try!(d.text_borrow().map_err(|_| Error))));
                try!(write!(f, "{}",
                    try!(d.text_borrow().map_err(|_| Error))));
            }
            try!(write!(f, "}}"));
            Ok(())
        }
    }

    impl Clone for Key {
        fn clone(&self) -> Key {
            // Unfortunately Box<[u8]> doesn't support Clone
            Key(self.0.as_ref().map(|x| x.to_vec().into_boxed_slice()))
        }
    }
}