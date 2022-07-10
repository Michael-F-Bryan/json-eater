use std::fmt;

use serde::de::{DeserializeSeed, MapAccess, SeqAccess};

use crate::{Path, Segment, Visitor};

#[derive(Debug)]
pub(crate) struct Seed<'de, V> {
    visitor: V,
    path: Path<'de>,
}

impl<'de, V: Visitor<'de>> Seed<'de, V> {
    pub(crate) fn new(visitor: V) -> Self {
        Seed {
            visitor,
            path: Path::default(),
        }
    }
}

impl<'de, V: Visitor<'de>> DeserializeSeed<'de> for &mut Seed<'de, V> {
    type Value = ();

    fn deserialize<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        de.deserialize_any(self)
    }
}

impl<'de, V: Visitor<'de>> serde::de::Visitor<'de> for &mut Seed<'de, V> {
    type Value = ();

    fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        unreachable!("Visiting should never fail")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_null(&self.path);
        Ok(())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_null(&self.path);
        Ok(())
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_bool(&self.path, v);
        Ok(())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_str(&self.path, v);
        Ok(())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_owned_string(&&self.path, v);
        Ok(())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_u64(&self.path, v);
        Ok(())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_i64(&self.path, v);
        Ok(())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_f64(&self.path, v);
        Ok(())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key()? {
            self.path.push(Segment::String(key));

            map.next_value_seed(&mut *self)?;
            self.path.pop();
        }

        Ok(())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut index = 0;

        loop {
            self.path.push(index.into());
            let done = seq.next_element_seed(&mut *self)?.is_none();
            self.path.pop();
            index += 1;

            if done {
                return Ok(());
            }
        }
    }
}
