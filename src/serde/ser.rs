use super::SerdeError;
use crate::Key;
use crate::platform::Platform;
use alloc::string::ToString;
use serde::Serialize;

pub struct Serializer<'a, P: Platform> {
    nvs: &'a mut crate::Nvs<P>,
    namespace: &'a Key,
}

pub struct ValueSerializer<'input, 'output, P: Platform> {
    serializer: &'output mut Serializer<'input, P>,
    key: Key,
}

impl<P: Platform> ValueSerializer<'_, '_, P> {
    fn write_value<T>(&mut self, v: T) -> Result<(), crate::error::Error>
    where
        crate::Nvs<P>: crate::Set<T>,
        T: core::fmt::Debug,
    {
        self.serializer
            .nvs
            .set(self.serializer.namespace, &self.key, v)
    }
}
pub struct TopLevelSerializer<'input, 'output, P: Platform> {
    serializer: &'output mut Serializer<'input, P>,
}

impl<'input, 'output, P: Platform> serde::ser::SerializeStruct
    for &'output mut Serializer<'input, P>
where
    'output: 'input,
{
    type Ok = &'output mut Serializer<'input, P>;
    type Error = SerdeError;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = Key::from_str(key);

        let s = ValueSerializer {
            serializer: self,
            key,
        };

        value.serialize(s)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        todo!("{key}")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl<'input, 'output, P: Platform> serde::Serializer for TopLevelSerializer<'input, 'output, P>
where
    'output: 'input,
{
    type Ok = &'output mut Serializer<'input, P>;
    type Error = SerdeError;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = &'output mut Serializer<'input, P>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    // ---- all scalars error at top level ----
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }

    // Allow Option<T> at top-level if T is a struct (Serde will call into serialize_struct)
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::top_level_ser())
    }

    // Allow newtype struct wrappers around structs.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(SerdeError::top_level_ser())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerdeError::top_level_ser())
    }

    // The one thing you *do* support at top-level:
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self.serializer)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerdeError::top_level_ser())
    }
}

impl<'input, 'output, P: Platform> serde::Serializer for ValueSerializer<'input, 'output, P> {
    type Ok = ();
    type Error = SerdeError;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::not_yet_impl("i128"))
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }
    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::not_yet_impl("i128"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::not_yet_impl("f32"))
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerdeError::not_yet_impl("f64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(v.to_string().as_str())?;
        Ok(())
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write_value(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        // write the inner value to the same key
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        mut self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        // store enum unit variant as its name
        self.write_value(variant_index)?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(SerdeError::wont_impl("new_type_varient"))
    }

    // composites not supported by a single-key writer
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerdeError::wont_impl("nested seq"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerdeError::wont_impl("nested tuple"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerdeError::wont_impl("nested tuple struct"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerdeError::wont_impl("nested tuple varient"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerdeError::wont_impl("nested map"))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerdeError::wont_impl("nested structs"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerdeError::wont_impl("nested struct varient"))
    }
}

pub fn write<T: serde::ser::Serialize, P: Platform>(
    nvs: &mut crate::Nvs<P>,
    namespace: &Key,
    input: T,
) -> Result<(), SerdeError> {
    let mut serializer = Serializer { nvs, namespace };
    input.serialize(TopLevelSerializer {
        serializer: &mut serializer,
    })?;

    Ok(())
}
