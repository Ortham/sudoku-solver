use std::{num::NonZeroU8, str::FromStr};

fn clamp_value(value: NonZeroU8) -> Option<CellValue> {
    if value.get() > 9 {
        None
    } else {
        Some(CellValue(value))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellValue(NonZeroU8);

impl CellValue {
    pub fn new(value: u8) -> Option<Self> {
        NonZeroU8::new(value).and_then(clamp_value)
    }

    pub fn from_str(value: &str) -> Option<Self> {
        NonZeroU8::from_str(value).ok().and_then(clamp_value)
    }
}

impl From<CellValue> for usize {
    fn from(value: CellValue) -> Self {
        usize::from(value.0.get())
    }
}

impl From<&CellValue> for usize {
    fn from(value: &CellValue) -> Self {
        usize::from(value.0.get())
    }
}

impl ToString for CellValue {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
