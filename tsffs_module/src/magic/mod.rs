//! Magic instructions raised by guest software

extern crate num_traits;
use anyhow::{Context, Error, Result};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive as _;
use serde::{Deserialize, Serialize};

// Magic number definitions for harnessing via magic instruction. The lowest `n` value is `12` on
// ARM, so all magic numbers must stay below this number.

// NOTE: 0x00 is reserved
pub const MAGIC_START: u16 = 0x0001;
pub const MAGIC_STOP: u16 = 0x0002;
// NOTE: 0x0c is reserved

#[derive(Debug, ToPrimitive, FromPrimitive, Serialize, Deserialize, Clone)]
#[repr(u16)]
/// A Magic value that can be generated by a target as part of a "Magic Instruction" (cpuid with
/// a specific leaf on x86) and passed to SIMICS through the `Core_Magic_Instruction` HAP
pub enum MagicCode {
    /// Magic value that the target has reached its start location
    Start = MAGIC_START,
    /// Magic value that the target has reached its stop location
    Stop = MAGIC_STOP,
}

impl TryFrom<i64> for MagicCode {
    type Error = Error;

    /// Try to convert a u16 value to a known `Magic` value
    fn try_from(value: i64) -> Result<Self> {
        num::FromPrimitive::from_u16(value as u16)
            .context(format!("Could not convert value {} to MagicCode", value))
    }
}

impl TryInto<i64> for MagicCode {
    type Error = Error;
    fn try_into(self) -> Result<i64> {
        Ok(self.to_u16().context("Could not convert self to i64")? as i64)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Magic {
    Start((MagicCode, Option<u64>, Option<u64>)),
    Stop((MagicCode, Option<u64>)),
}

impl TryFrom<i64> for Magic {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
        let code = MagicCode::try_from(value)?;
        Ok(match code {
            MagicCode::Start => Self::Start((code, None, None)),
            MagicCode::Stop => Self::Stop((code, None)),
        })
    }
}
