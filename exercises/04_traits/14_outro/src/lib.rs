use std::ops::{Add};
use std::cmp::Ord;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        SaturatingU16 {
            value: value.min(u16::MAX),
        }
    }

    fn saturating_add(self, other: u16) -> Self {
        SaturatingU16::new(self.value.saturating_add(other))
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16::new(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16::new(value as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16::new(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16::new(*value as u16)
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.saturating_add(other.value)
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        self.saturating_add(other)
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self::Output {
        self.saturating_add(*other)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self::Output {
        self.saturating_add(other.value)
    }
}

impl PartialOrd for SaturatingU16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for SaturatingU16 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
