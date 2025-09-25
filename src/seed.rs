use anyhow::{Error, Result};
use bc_components::{Seed as ComponentsSeed, tags};
use bc_envelope::{Envelope, known_values};
use dcbor::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seed {
    data: Vec<u8>,
    name: String, // Empty string: no name
    note: String, // Empty string: no note
    creation_date: Option<Date>,
}

impl Seed {
    pub fn new<T>(data: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        Self::new_opt(data, String::new(), String::new(), None)
    }

    pub fn new_opt<T, S, U>(
        data: T,
        name: S,
        note: U,
        creation_date: Option<Date>,
    ) -> Self
    where
        T: AsRef<[u8]>,
        S: AsRef<str>,
        U: AsRef<str>,
    {
        Self {
            data: data.as_ref().to_vec(),
            name: name.as_ref().to_string(),
            note: note.as_ref().to_string(),
            creation_date,
        }
    }

    pub fn data(&self) -> &[u8] { &self.data }

    pub fn name(&self) -> &str { &self.name }

    pub fn set_name(&mut self, name: impl AsRef<str>) {
        self.name = name.as_ref().to_string();
    }

    pub fn note(&self) -> &str { &self.note }

    pub fn set_note(&mut self, note: impl AsRef<str>) {
        self.note = note.as_ref().to_string();
    }

    pub fn creation_date(&self) -> Option<&Date> { self.creation_date.as_ref() }

    pub fn set_creation_date(
        &mut self,
        creation_date: Option<impl AsRef<Date>>,
    ) {
        self.creation_date = creation_date.map(|s| s.as_ref().clone());
    }
}

impl CBORTagged for Seed {
    fn cbor_tags() -> Vec<Tag> { tags_for_values(&[tags::TAG_SEED]) }
}

impl From<Seed> for CBOR {
    fn from(value: Seed) -> Self { value.tagged_cbor() }
}

impl CBORTaggedEncodable for Seed {
    fn untagged_cbor(&self) -> CBOR {
        let mut map = Map::new();
        map.insert(1, CBOR::to_byte_string(self.data()));
        if let Some(creation_date) = self.creation_date() {
            map.insert(2, creation_date.clone());
        }
        if !self.name().is_empty() {
            map.insert(3, self.name());
        }
        if !self.note().is_empty() {
            map.insert(4, self.note());
        }
        map.into()
    }
}

impl TryFrom<CBOR> for Seed {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        Self::from_tagged_cbor(cbor)
    }
}

impl CBORTaggedDecodable for Seed {
    fn from_untagged_cbor(cbor: CBOR) -> dcbor::Result<Self> {
        let map = cbor.try_into_map()?;
        let data = map
            .extract::<i32, CBOR>(1)?
            .try_into_byte_string()?
            .to_vec();
        if data.is_empty() {
            return Err("invalid seed data".into());
        }
        let creation_date = map.get::<i32, Date>(2);
        let name = map.get::<i32, String>(3).unwrap_or_default();
        let note = map.get::<i32, String>(4).unwrap_or_default();
        Ok(Self::new_opt(data, name, note, creation_date))
    }
}

impl From<Seed> for Envelope {
    fn from(seed: Seed) -> Self {
        let mut e = Envelope::new(CBOR::to_byte_string(seed.data()))
            .add_type(known_values::SEED_TYPE)
            .add_optional_assertion(
                known_values::DATE,
                seed.creation_date().cloned(),
            );

        if !seed.name().is_empty() {
            e = e.add_assertion(known_values::NAME, seed.name());
        }

        if !seed.note().is_empty() {
            e = e.add_assertion(known_values::NOTE, seed.note());
        }

        e
    }
}

impl TryFrom<Envelope> for Seed {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        envelope.check_type(&known_values::SEED_TYPE)?;
        let data = envelope
            .subject()
            .try_leaf()?
            .try_into_byte_string()?
            .to_vec();
        let name = envelope
            .extract_optional_object_for_predicate::<String>(
                known_values::NAME,
            )?
            .unwrap_or_default()
            .to_string();
        let note = envelope
            .extract_optional_object_for_predicate::<String>(
                known_values::NOTE,
            )?
            .unwrap_or_default()
            .to_string();
        let creation_date = envelope
            .extract_optional_object_for_predicate::<Date>(known_values::DATE)?
            .map(|s| s.as_ref().clone());
        Ok(Self::new_opt(data, name, note, creation_date))
    }
}

fn optional_string(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

impl From<ComponentsSeed> for Seed {
    fn from(seed: ComponentsSeed) -> Self {
        let creation_date = seed.creation_date().clone();
        Self::new_opt(seed.as_bytes(), seed.name(), seed.note(), creation_date)
    }
}

impl TryFrom<&Seed> for ComponentsSeed {
    type Error = bc_components::Error;

    fn try_from(seed: &Seed) -> Result<Self, Self::Error> {
        ComponentsSeed::new_opt(
            seed.data(),
            optional_string(seed.name()),
            optional_string(seed.note()),
            seed.creation_date().cloned(),
        )
    }
}

impl TryFrom<Seed> for ComponentsSeed {
    type Error = bc_components::Error;

    fn try_from(seed: Seed) -> Result<Self, Self::Error> {
        Self::try_from(&seed)
    }
}
