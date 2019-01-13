use serde::de::{self, Deserialize};
use std::fmt;

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(Deserializer::new(deserializer))
}

pub struct Deserializer<D> {
    de: D,
}

impl<D> Deserializer<D> {
    pub fn new(deserializer: D) -> Self {
        Deserializer { de: deserializer }
    }
}

impl<'de, D> de::Deserializer<'de> for Deserializer<D>
where
    D: de::Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_any(Visitor::new(visitor))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_bool(Visitor::new(visitor))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_u8(Visitor::new(visitor))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_u16(Visitor::new(visitor))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_u32(Visitor::new(visitor))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_u64(Visitor::new(visitor))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_i8(Visitor::new(visitor))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_i16(Visitor::new(visitor))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_i32(Visitor::new(visitor))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_i64(Visitor::new(visitor))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_f32(Visitor::new(visitor))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_f64(Visitor::new(visitor))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_char(Visitor::new(visitor))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_str(Visitor::new(visitor))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_string(Visitor::new(visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_bytes(Visitor::new(visitor))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_byte_buf(Visitor::new(visitor))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_option(Visitor::new(visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_unit(Visitor::new(visitor))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_unit_struct(name, Visitor::new(visitor))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de
            .deserialize_newtype_struct(name, Visitor::new(visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_seq(Visitor::new(visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_tuple(len, Visitor::new(visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de
            .deserialize_tuple_struct(name, len, Visitor::new(visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_map(Visitor::new(visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de
            .deserialize_struct(name, fields, Visitor::new(visitor))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de
            .deserialize_enum(name, variants, Visitor::new(visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_ignored_any(Visitor::new(visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_identifier(Visitor::new(visitor))
    }
}

struct Visitor<V> {
    delegate: V,
}

impl<V> Visitor<V> {
    fn new(delegate: V) -> Self {
        Visitor { delegate }
    }
}

impl<'de, V> de::Visitor<'de> for Visitor<V>
where
    V: de::Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.delegate.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i64(v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i128(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u64(v)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u128(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_string(v)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_unit()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate.visit_some(Deserializer { de: deserializer })
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate
            .visit_newtype_struct(Deserializer { de: deserializer })
    }

    fn visit_seq<A>(self, visitor: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        self.delegate.visit_seq(SeqAccess::new(visitor))
    }

    fn visit_map<A>(self, visitor: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        self.delegate.visit_map(MapAccess::new(visitor))
    }

    fn visit_enum<A>(self, visitor: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        self.delegate.visit_enum(EnumAccess::new(visitor))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_byte_buf(v)
    }
}

struct EnumAccess<D> {
    delegate: D,
}

impl<D> EnumAccess<D> {
    fn new(delegate: D) -> Self {
        EnumAccess { delegate }
    }
}

impl<'de, D> de::EnumAccess<'de> for EnumAccess<D>
where
    D: de::EnumAccess<'de>,
{
    type Error = D::Error;
    type Variant = VariantAccess<D::Variant>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), D::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        self.delegate
            .variant_seed(DeserializeSeed::new(seed))
            .map(|(v, vis)| (v, VariantAccess::new(vis)))
    }
}

struct VariantAccess<D> {
    delegate: D,
}

impl<D> VariantAccess<D> {
    fn new(delegate: D) -> Self {
        VariantAccess { delegate }
    }
}

impl<'de, D> de::VariantAccess<'de> for VariantAccess<D>
where
    D: de::VariantAccess<'de>,
{
    type Error = D::Error;

    fn unit_variant(self) -> Result<(), D::Error> {
        self.delegate.unit_variant()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, D::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.delegate
            .newtype_variant_seed(DeserializeSeed::new(seed))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.delegate.tuple_variant(len, Visitor::new(visitor))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: de::Visitor<'de>,
    {
        self.delegate.struct_variant(fields, Visitor::new(visitor))
    }
}

struct DeserializeSeed<S> {
    delegate: S,
}

impl<S> DeserializeSeed<S> {
    fn new(delegate: S) -> Self {
        DeserializeSeed { delegate }
    }
}

impl<'de, S> de::DeserializeSeed<'de> for DeserializeSeed<S>
where
    S: de::DeserializeSeed<'de>,
{
    type Value = S::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate.deserialize(Deserializer { de: deserializer })
    }
}

struct SeqAccess<D> {
    delegate: D,
}

impl<D> SeqAccess<D> {
    fn new(delegate: D) -> Self {
        SeqAccess { delegate }
    }
}

impl<'de, D> de::SeqAccess<'de> for SeqAccess<D>
where
    D: de::SeqAccess<'de>,
{
    type Error = D::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, D::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.delegate.next_element_seed(DeserializeSeed::new(seed))
    }

    fn size_hint(&self) -> Option<usize> {
        self.delegate.size_hint()
    }
}

struct MapAccess<D> {
    delegate: D,
}

impl<D> MapAccess<D> {
    fn new(delegate: D) -> Self {
        MapAccess { delegate }
    }
}

impl<'de, D> de::MapAccess<'de> for MapAccess<D>
where
    D: de::MapAccess<'de>,
{
    type Error = D::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, D::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.delegate.next_key_seed(DeserializeSeed::new(seed))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, D::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        self.delegate.next_value_seed(DeserializeSeed::new(seed))
    }

    fn size_hint(&self) -> Option<usize> {
        self.delegate.size_hint()
    }
}