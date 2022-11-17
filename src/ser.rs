//! Serialize a Rust data structure into URL parameters string.

use crate::error::{Error, Result};
use std::fmt;
use std::io;

/// A structure for serializing Rust values into URL parameters string.
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
            writer,
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
        use std::iter::FromIterator;
        let encoded = String::from_iter(url::form_urlencoded::byte_serialize(value.as_bytes()));
        self.write_key_value(&encoded)
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

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        use serde::Serialize;
        variant.serialize(self)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
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

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        if self.current_key.is_some() {
            Err(Self::Error::unsupported("nested struct"))
        } else {
            Ok(self)
        }
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        if self.current_key.is_some() {
            Err(Self::Error::unsupported("nexted struct variant"))
        } else {
            Ok(self)
        }
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

impl<'a, W> ::serde::ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(&mut **self)
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

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(&mut **self)
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

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        self.current_key = {
            let mut string_serializer = StringOnlySerializer::default();
            key.serialize(&mut string_serializer)?;
            Some(string_serializer.into())
        };
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(&mut **self)
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

/// This serializer only serializes Strings and Chars. It fails for any other
/// type from Serde's data model.
#[derive(Debug, Default)]
struct StringOnlySerializer {
    value: String,
}

impl From<StringOnlySerializer> for String {
    fn from(s: StringOnlySerializer) -> Self {
        s.value
    }
}

impl<'a> ::serde::ser::Serializer for &'a mut StringOnlySerializer {
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
    fn serialize_bool(self, _value: bool) -> Result<()> {
        Err(Self::Error::unsupported("bool"))
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> {
        Err(Self::Error::unsupported("i8"))
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> {
        Err(Self::Error::unsupported("i16"))
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<()> {
        Err(Self::Error::unsupported("i32"))
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> {
        Err(Self::Error::unsupported("i64"))
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> {
        Err(Self::Error::unsupported("u8"))
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(Self::Error::unsupported("u16"))
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(Self::Error::unsupported("u32"))
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> {
        Err(Self::Error::unsupported("u64"))
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(Self::Error::unsupported("f32"))
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(Self::Error::unsupported("f64"))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        self.value = format!("{}", value);
        Ok(())
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.value = String::from(value);
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(Self::Error::unsupported("bytes"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Self::Error::unsupported("none"))
    }

    #[inline]
    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("some"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Self::Error::unsupported("unit"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Self::Error::unsupported("unit struct"))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Self::Error::unsupported("unit variant"))
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("newtype struct"))
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("newtype variant"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Self::Error::unsupported("seq"))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Self::Error::unsupported("tuple"))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Self::Error::unsupported("tuple struct"))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Self::Error::unsupported("tuple variant"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Self::Error::unsupported("map"))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Self::Error::unsupported("struct"))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Self::Error::unsupported("struct variant"))
    }
}

impl<'a> ::serde::ser::SerializeSeq for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("seq element"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("seq end"))
    }
}

impl<'a> ::serde::ser::SerializeTuple for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("tuple element"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("tuple variant"))
    }
}

impl<'a> ::serde::ser::SerializeTupleStruct for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("tuple struct field"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("tuple struct field"))
    }
}

impl<'a> ::serde::ser::SerializeTupleVariant for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("tuple variant field"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("tuple variant end"))
    }
}

impl<'a> ::serde::ser::SerializeMap for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("map key"))
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("map value"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("map end"))
    }
}

impl<'a> ::serde::ser::SerializeStruct for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("struct field"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("struct end"))
    }
}

impl<'a> ::serde::ser::SerializeStructVariant for &'a mut StringOnlySerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        Err(Self::Error::unsupported("struct variant field"))
    }

    fn end(self) -> Result<()> {
        Err(Self::Error::unsupported("struct variant end"))
    }
}

/// Serialize the given data structure as URL parameters into the IO stream.
///
/// # Errors
///
/// Serialization fails if:
///
/// * `T`'s implementation of `Serialize` decides to fail,
/// * `T` is a type without keys, i.e. not a struct.
/// * `T` contains a nested struct,
/// * `T` contains a map.
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

/// Serialize the given data structure as a byte vector containing URL
/// parameters.
///
/// # Errors
///
/// Serialization fails if:
///
/// * `T`'s implementation of `Serialize` decides to fail,
/// * `T` is a type without keys, i.e. not a struct.
/// * `T` contains a nested struct,
/// * `T` contains a map.
#[inline]
pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: ::serde::ser::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given data structure as a String of URL parameters.
///
/// # Errors
///
/// Serialization fails if:
///
/// * `T`'s implementation of `Serialize` decides to fail,
/// * `T` is a type without keys, i.e. not a struct.
/// * `T` contains a nested struct,
/// * `T` contains a map.
#[inline]
pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: ::serde::ser::Serialize,
{
    let vec = to_vec(value)?;
    let string = String::from_utf8(vec)?;
    Ok(string)
}
