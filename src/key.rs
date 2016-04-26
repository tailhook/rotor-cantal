use cbor::{Decoder, Config};
use std::io::Cursor;

///
/// This contains CBOR-encoded key-value pairs
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(Option<Box<[u8]>>);


/// A type passed to by `Key::visit`
pub enum KeyVisitor<'x> {
    Key(&'x str),
    Value(&'x str),
}

impl Key {
    pub fn as_bytes<'x>(&'x self) -> &'x [u8] {
        self.0.as_ref().map(|x| &x[..]).unwrap_or(b"")
    }
    /// This method has separate key and value visitor because of borrowing
    /// rules
    ///
    /// Keys are visited in sorted order, and every key and every value
    /// is visited in sequence (key1, value1, key2, value2...)
    pub fn visit<'x, K>(&self, mut visitor: K)
        where K: FnMut(KeyVisitor)
    {
        if let Some(ref x) = self.0 {
            let mut d = Decoder::new(Config::default(), Cursor::new(&x[..]));
            let num = d.object().unwrap();
            for _ in 0..num {
                // TODO(tailhook) other types may work in future
                visitor(KeyVisitor::Key(d.text_borrow().unwrap()));
                visitor(KeyVisitor::Value(d.text_borrow().unwrap()));
            }
        }
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
            if value.len() == 0 {
                Ok(Some(Key(None)))
            } else {
                try!(validate_key(&value[..]).map_err(|e|
                    DecodeError::WrongValue(e)));
                Ok(Some(Key(Some(value.into_boxed_slice()))))
            }
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

#[cfg(test)]
mod test {
    use super::{Key, KeyVisitor};

    #[test]
    fn test_visitor() {
        let mut x = String::new();
        let key = Key(Some(vec![
            0xa2, 0x61, b'a', 0x61, b'b', 0x61, b'c', 0x61, b'd',
            ].into_boxed_slice()));
        key.visit(|item| {
            match item {
                KeyVisitor::Key(k) => {
                    x.push_str(k);
                    x.push(':');
                }
                KeyVisitor::Value(v) => {
                    x.push_str(v);
                    x.push(',');
                }
            }
        });
        assert_eq!(&x[..], "a:b,c:d,");
    }
}
