//! FileSystem::readJSON binary branches, with Native's strict end-of-input.
//! The complete parsed value reaches the resolved-path cache before selection.
use super::{NativeExternalDataObject as Object, NativeExternalDataValue as Value};

mod bson;
mod cbor;
mod msgpack;
mod ubjson;

#[derive(Clone, Copy, Debug)]
pub(crate) enum NativeBinaryFormat {
    Cbor,
    MsgPack,
    Ubjson,
    Bson,
}

/// A cause retained for the host's explicit Native parse-exception boundary.
/// This is not a fabricated nlohmann diagnostic string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativeBinaryDecodeError {
    pub(crate) byte_offset: usize,
    pub(crate) cause: &'static str,
}

impl NativeBinaryDecodeError {
    pub(crate) fn new(byte_offset: usize, cause: &'static str) -> Self {
        Self { byte_offset, cause }
    }
}

pub(crate) fn decode_native_binary_json(
    bytes: &[u8],
    format: NativeBinaryFormat,
) -> Result<Value, NativeBinaryDecodeError> {
    match format {
        NativeBinaryFormat::Cbor => cbor::decode_native_cbor(bytes),
        NativeBinaryFormat::MsgPack => msgpack::decode_native_msgpack(bytes),
        NativeBinaryFormat::Ubjson => ubjson::decode_native_ubjson(bytes),
        NativeBinaryFormat::Bson => bson::decode_native_bson(bytes),
    }
}

pub(crate) struct BinaryCursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> BinaryCursor<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    pub(crate) fn position(&self) -> usize {
        self.at
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.len() - self.at
    }

    pub(crate) fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    pub(crate) fn read(&mut self, cause: &'static str) -> Result<u8, NativeBinaryDecodeError> {
        let byte = self
            .peek()
            .ok_or_else(|| NativeBinaryDecodeError::new(self.at, cause))?;
        self.at += 1;
        Ok(byte)
    }

    pub(crate) fn take(
        &mut self,
        len: usize,
        cause: &'static str,
    ) -> Result<&'a [u8], NativeBinaryDecodeError> {
        if len > self.remaining() {
            self.at = self.bytes.len();
            return Err(NativeBinaryDecodeError::new(self.at, cause));
        }
        let start = self.at;
        self.at += len;
        Ok(&self.bytes[start..self.at])
    }

    pub(crate) fn fixed<const N: usize>(
        &mut self,
        cause: &'static str,
    ) -> Result<[u8; N], NativeBinaryDecodeError> {
        let mut value = [0; N];
        value.copy_from_slice(self.take(N, cause)?);
        Ok(value)
    }

    pub(crate) fn finish(&mut self, allow_noops: bool) -> Result<(), NativeBinaryDecodeError> {
        if allow_noops {
            while self.peek() == Some(b'N') {
                self.at += 1;
            }
        }
        if self.remaining() == 0 {
            Ok(())
        } else {
            Err(NativeBinaryDecodeError::new(
                self.at,
                "unexpected data after binary JSON value",
            ))
        }
    }
}
