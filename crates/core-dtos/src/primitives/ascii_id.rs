use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum AsciiIds {
    NotVisible,
    Visible,
    Sand,
    ShallowWater,
    DeepWater,
    HeadQuarter,
    UnknownAsciiId,
}

impl Default for AsciiIds {
    fn default() -> Self {
        AsciiIds::UnknownAsciiId
    }
}

impl AsciiIds {

    fn contains_value(current: f64, start: f64, end: f64) -> bool {
        current >= start && current < end
    }

    pub fn to_float(&self) -> f64{
        match self {
            Self::HeadQuarter => 1000.0,
            Self::Sand => 1.0,
            Self::NotVisible => 0.0,
            Self::Visible => 255.0,
            Self::ShallowWater => 3.0,
            Self::DeepWater => 4.0,
            Self::UnknownAsciiId => 0.0, 
        }
    }

    pub fn value_to_id(value: f64) -> AsciiIds {
        if Self::contains_value(value, -100.0, -99.9) {
            return AsciiIds::NotVisible;
        }

        if Self::contains_value(value, -5.0, -0.5) {
            return AsciiIds::DeepWater;
        }

        if Self::contains_value(value, -0.5, 0.1) {
            return AsciiIds::ShallowWater;
        }

        if Self::contains_value(value, 0.1, 2.5) {
            return AsciiIds::Sand;
        }

        if Self::contains_value(value, 1000.0, 1001.0) {
            return AsciiIds::HeadQuarter;
        }

        return AsciiIds::UnknownAsciiId;
    }
}