use client_common::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// hardware wallet types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum HardwareKind {
    /// ledger wallet
    Ledger,
    /// trezor wallet
    Trezor,
    /// mock wallet
    #[cfg(feature = "mock-hardware-wallet")]
    Mock,
}

impl FromStr for HardwareKind {
    type Err = Error;

    #[cfg(feature = "mock-hardware-wallet")]
    fn from_str(s: &str) -> Result<Self> {
        if s == "ledger" {
            Ok(HardwareKind::Ledger)
        } else if s == "trezor" {
            Ok(HardwareKind::Trezor)
        } else if s == "mock" {
            Ok(HardwareKind::Mock)
        } else {
            Err(ErrorKind::DeserializationError.into())
        }
    }

    #[cfg(not(feature = "mock-hardware-wallet"))]
    fn from_str(s: &str) -> Result<Self> {
        if s == "ledger" {
            Ok(HardwareKind::Ledger)
        } else if s == "trezor" {
            Ok(HardwareKind::Trezor)
        } else {
            Err(ErrorKind::DeserializationError.into())
        }
    }
}
