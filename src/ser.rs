// #![deny(missing_docs)]

use std::io;
use std::fmt;
use error::{Error, Result};

pub struct Serializer<W> {
    writer: W,
    current_key: Option<String>,
    first_param: bool,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    fn new(writer: W) -> Self {
        Serializer {
            writer: writer,
            current_key: None,
            first_param: true,
        }
    }

    #[inline]
    fn write_key_value<T>(&mut self, value: T) -> Result<()>
    where
        T: fmt::Display,
    {
        use serde::ser::Error;
        match self.current_key.as_ref() {
            Some(key) => {
                write!(
                    self.writer,
                    "{}{}={}",
                    if self.first_param { "" } else { "&" },
                    key,
                    value
                )?;
                self.first_param = false;
                Ok(())
            }
            None => Err(Error::custom("cannot serialize top level value")),
        }
    }
}

impl<'a, W> ::serde::ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        // TODO: escape url
        self.write_key_value(value)
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(value.len()))?;
        for byte in value {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        use serde::Serialize;
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        use serde::ser::Error;
        Err(Error::custom(
            "serialization of newtype struct is not supported",
        ))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        use serde::ser::Error;
        Err(Error::custom(
            "serialization of tuple struct is not supported",
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        use serde::ser::Error;
        Err(Self::Error::custom(
            "serialization of tuple variant is not supported",
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        use serde::ser::Error;
        Err(Self::Error::custom("serialization of map is not supported"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        use serde::ser::Error;
        if self.current_key.is_some() {
            Err(Self::Error::custom(
                "serialization of nested struct is not supported",
            ))
        } else {
            Ok(self)
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        use serde::ser::Error;
        Err(Self::Error::custom(
            "serialization of struct variant is not supported",
        ))
    }
}

impl<'a, W> ::serde::ser::SerializeSeq for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeTuple for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeMap for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        // TODO: For now, we are not supporting maps, since we need to make sure that T is String
        // convertable. For that, we need another Serializer that can serialize only String and
        // fails for every other type.
        //
        // For that we could use trait inheritance:
        //   EmptySerializer < StringSerializer < this serializer
        Ok(())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        self.current_key = Some(String::from(key));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.current_key = None;
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[inline]
pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ::serde::ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}

#[inline]
pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: ::serde::ser::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

#[inline]
pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: ::serde::ser::Serialize,
{
    let vec = to_vec(value)?;
    let string = String::from_utf8(vec)?;
    Ok(string)
}
