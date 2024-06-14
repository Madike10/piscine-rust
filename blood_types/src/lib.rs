extern crate core;

use core::fmt;
use std::cmp::{Ord, Ordering};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl Display for Antigen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen.cmp(&other.antigen) == Ordering::Equal {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_at(s.len() - 1);
        let antigen = Antigen::from_str(parts.0)?;
        let rh_factor = RhFactor::from_str(parts.1)?;
        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rh_format = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, " {}{}", self.antigen, rh_format)
    }
}

impl BloodType {
    pub fn all_blood_types(&self) -> [BloodType; 8] {
        [
            Self {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            Self {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            Self {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            Self {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            Self {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            Self {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            Self {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            Self {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
        ]
    }

    pub fn can_receive_from(&self, other: &BloodType) -> bool {
        match (&self.antigen, &other.antigen) {
            (Antigen::A, Antigen::A)
            | (Antigen::A, Antigen::O)
            | (Antigen::B, Antigen::B)
            | (Antigen::B, Antigen::O)
            | (Antigen::AB, _)
            | (Antigen::O, Antigen::O) => match (&self.rh_factor, &other.rh_factor) {
                (RhFactor::Positive, _) | (RhFactor::Negative, RhFactor::Negative) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        self.all_blood_types()
            .iter()
            .filter_map(|blood_type| {
                if self.can_receive_from(blood_type) {
                    Some(blood_type.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        self.all_blood_types()
            .iter()
            .filter_map(|blood_type| {
                if blood_type.can_receive_from(self) {
                    Some(blood_type.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
