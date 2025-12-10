use alloc::{format, string::String, vec::Vec};
use serde::{Deserializer, de::IntoDeserializer, de::Visitor};

use crate::{Key, Nvs, platform::Platform, serde::SerdeError};

type Result<T, E = SerdeError> = core::result::Result<T, E>;

pub struct NvsDeserializer<'a, P: Platform> {
    nvs: &'a mut crate::Nvs<P>,
    namespace: &'a Key,
}

struct KvEnumAccess<'a, P: Platform> {
    idx: u32,
    variants: &'static [&'static str],
    nvs: &'a mut crate::Nvs<P>,
    namespace: &'a Key,
}

impl<'a, 'de, P: Platform> serde::de::EnumAccess<'de> for KvEnumAccess<'a, P> {
    type Error = SerdeError;
    type Variant = KvVariantAccess<'a, P>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let i = self.idx as usize;

        let variant_name = self.variants.get(i).ok_or_else(|| {
            <SerdeError as serde::de::Error>::custom(format!(
                "enum index {} out of range ({} variants)",
                self.idx,
                self.variants.len()
            ))
        })?;

        // Deserialize the chosen variant name into Serde's seed.
        let v = seed.deserialize::<serde::de::value::StrDeserializer<'_, SerdeError>>(
            (*variant_name).into_deserializer(),
        )?;
        Ok((
            v,
            KvVariantAccess {
                nvs: self.nvs,
                namespace: self.namespace,
            },
        ))
    }
}

#[allow(unused)]
struct KvVariantAccess<'a, P: Platform> {
    nvs: &'a mut crate::Nvs<P>,
    namespace: &'a Key,
}

impl<'a, 'de, P: Platform> serde::de::VariantAccess<'de> for KvVariantAccess<'a, P> {
    type Error = SerdeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        // Unit variant: only the index was stored.
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        Err(SerdeError::wont_impl("newtype enum"))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("tuple variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("struct variant"))
    }
}

pub struct ValueDeserializer<'a, P: Platform> {
    nvs: &'a mut crate::Nvs<P>,
    namespace: &'a Key,
    key: Key,
}

impl<P: Platform> ValueDeserializer<'_, P> {
    fn read_value<T>(&mut self) -> Result<T, crate::error::Error>
    where
        crate::Nvs<P>: crate::Get<T>,
        T: core::fmt::Debug,
    {
        self.nvs.get(self.namespace, &self.key)
    }
}

impl<'de, 'a, P: Platform> Deserializer<'de> for &'a mut ValueDeserializer<'a, P> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("inner any"))
    }

    // --- numbers / bool ---
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: bool = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_bool(v)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: i8 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_i8(v)
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: i16 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_i16(v)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: i32 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_i32(v)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: i64 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_i64(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: u8 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_u8(v)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: u16 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_u16(v)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: u32 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_u32(v)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: u64 = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_u64(v)
    }

    // --- strings ---
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: String = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_string(v)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: String = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_str(v.as_str())
    }

    // --- option ---
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.nvs.check_key_exists(self.namespace, &self.key)? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    // --- bytes ---
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v: Vec<u8> = self.read_value().map_err(SerdeError::from)?;
        visitor.visit_byte_buf(v)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Read the stored enum index.
        let idx: u32 = self.read_value().map_err(SerdeError::from)?;

        let access = KvEnumAccess {
            idx,
            variants,
            nvs: self.nvs,
            namespace: self.namespace,
        };

        visitor.visit_enum(access)
    }

    fn deserialize_unit<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_char<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_u128<V>(self, _visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::not_yet_impl("u128"))
    }
    fn deserialize_i128<V>(self, _visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::not_yet_impl("i128"))
    }
    fn deserialize_f32<V>(self, _visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::not_yet_impl("f32"))
    }
    fn deserialize_f64<V>(self, _visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::not_yet_impl("f64"))
    }

    serde::forward_to_deserialize_any! {
        seq tuple tuple_struct map struct identifier ignored_any
    }
}

struct StructAccess<'a, P: Platform> {
    nvs: &'a mut crate::Nvs<P>,
    namespace: &'a Key,
    fields: &'static [&'static str],
    idx: usize,
    pending_key: Option<Key>,
}

impl<'de, P: Platform> serde::de::MapAccess<'de> for StructAccess<'de, P> {
    type Error = SerdeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.idx < self.fields.len() {
            let field = self.fields[self.idx];
            self.idx += 1;

            let k = Key::from_str(field);
            self.pending_key = Some(k);

            return seed.deserialize(field.into_deserializer()).map(Some);
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let key = self.pending_key.take().ok_or_else(SerdeError::no_key)?;

        let mut value_de = ValueDeserializer {
            nvs: self.nvs,
            namespace: self.namespace,
            key,
        };

        seed.deserialize(&mut value_de)
    }
}

impl<'de, P: Platform> Deserializer<'de> for NvsDeserializer<'de, P> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("any"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let access = StructAccess {
            nvs: self.nvs,
            namespace: self.namespace,
            fields,
            idx: 0,
            pending_key: None,
        };

        visitor.visit_map(access)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("ident"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("ignored_any"))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(SerdeError::wont_impl("top level enum"))
    }

    serde::forward_to_deserialize_any! {
        bool
        u8
        u16
        u32
        u64
        u128
        i8
        i16
        i32
        i64
        i128
        f32
        f64
        char
        str
        string
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        tuple
        unit
        seq
        map
    }
}

pub fn read<'de, T: serde::Deserialize<'de>, P: Platform>(
    nvs: &'de mut Nvs<P>,
    namespace: &'de Key,
) -> Result<T, SerdeError> {
    let deserializer = NvsDeserializer { nvs, namespace };
    let t = T::deserialize(deserializer)?;
    Ok(t)
}
