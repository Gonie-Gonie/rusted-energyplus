//! Key equivalence of EnergyPlus's patched nlohmann object container.
//!
//! json.hpp20344 uses doj::alphanum_less, whose std::string overload passes
//! c_str() to alphanum_impl (alphanum.hpp97-185,222). Native's Windows unsigned
//! long is 32 bits. Numeric runs compare by wrapped value, then digit count;
//! the first NUL ends the key comparison. Assignment retains the first raw key.
//! A canonical index preserves that equivalence without installing Native's
//! non-total wrapped subtraction as a Rust Ord. No external schedule consumer
//! depends on object iteration order; array columns retain their original order.
use super::NativeExternalDataValue as Value;
use std::{collections::BTreeMap, collections::btree_map::Entry};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum KeyPart {
    Byte(u8),
    Number { value: u32, digits: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Identity(Vec<KeyPart>);

impl Identity {
    fn new(key: &[u8]) -> Self {
        let mut parts = Vec::new();
        let mut at = 0;
        while let Some(&byte) = key.get(at).filter(|byte| **byte != 0) {
            if byte.is_ascii_digit() {
                let (mut value, mut digits) = (0u32, 0u32);
                while let Some(&digit) = key.get(at).filter(|byte| byte.is_ascii_digit()) {
                    value = value.wrapping_mul(10).wrapping_add(u32::from(digit - b'0'));
                    digits = digits.wrapping_add(1);
                    at += 1;
                }
                parts.push(KeyPart::Number { value, digits });
            } else {
                parts.push(KeyPart::Byte(byte));
                at += 1;
            }
        }
        Self(parts)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeExternalDataObject {
    entries: BTreeMap<Identity, (Vec<u8>, Value)>,
}

impl NativeExternalDataObject {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn insert(&mut self, key: Vec<u8>, value: Value) {
        match self.entries.entry(Identity::new(&key)) {
            Entry::Occupied(mut entry) => entry.get_mut().1 = value,
            Entry::Vacant(entry) => {
                entry.insert((key, value));
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn get(&self, key: &[u8]) -> Option<&Value> {
        self.entries
            .get(&Identity::new(key))
            .map(|(_, value)| value)
    }

    pub(crate) fn get_or_insert_null(&mut self, key: Vec<u8>) -> &mut Value {
        &mut self
            .entries
            .entry(Identity::new(&key))
            .or_insert((key, Value::Null))
            .1
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Vec<u8>, &Value)> {
        self.entries.values().map(|(key, value)| (key, value))
    }
}

impl FromIterator<(Vec<u8>, Value)> for NativeExternalDataObject {
    fn from_iter<T: IntoIterator<Item = (Vec<u8>, Value)>>(iter: T) -> Self {
        let mut object = Self::new();
        for (key, value) in iter {
            object.insert(key, value);
        }
        object
    }
}
