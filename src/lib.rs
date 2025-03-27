//! Parse (validate) from multiple data types or generate new random [Saudi Arabian national IDs](https://en.wikipedia.org/wiki/Saudi_Arabian_identity_card).
//!
//! Used to validate IDs and find their type (Citizen or Resident), or used to test software by generating random valid IDs.

// TODO: Support no_std.

extern crate luhnr;

#[derive(PartialEq, Eq, Debug)]
pub enum IdType {
    Citizen = 1,
    Resident = 2,
}

impl IdType {
    #[must_use]
    pub const fn prefix(self) -> u8 {
        self as u8
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Id {
    pub digits: Vec<u8>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidId,
}

const ID_SIZE: usize = 10;
const CITIZEN_PREFIX: u8 = IdType::Citizen.prefix();
const RESIDENT_PREFIX: u8 = IdType::Resident.prefix();

impl Id {
    /// Create a new random ID
    #[expect(clippy::missing_panics_doc, reason = "Never panics")]
    #[must_use]
    pub fn new(id_type: &IdType) -> Self {
        match *id_type {
            IdType::Citizen => {
                #[expect(clippy::unwrap_used, reason = "Arguments always valid")]
                let digits = luhnr::generate_with_prefix(ID_SIZE, &[CITIZEN_PREFIX]).unwrap();

                Self { digits }
            }
            IdType::Resident => {
                #[expect(clippy::unwrap_used, reason = "Arguments always valid")]
                let digits = luhnr::generate_with_prefix(ID_SIZE, &[RESIDENT_PREFIX]).unwrap();

                Self { digits }
            }
        }
    }

    fn validate(digits: &[u8]) -> bool {
        // NOTE: The second statement is less likely to fail, but it depends on your usage.
        luhnr::validate(digits) && digits.len() == ID_SIZE
    }

    #[expect(clippy::missing_panics_doc, reason = "Never panics")]
    #[must_use]
    pub fn get_type(&self) -> IdType {
        #[expect(clippy::unwrap_used, reason = "Vec always has first digit")]
        match *self.digits.first().unwrap() {
            CITIZEN_PREFIX => IdType::Citizen,
            RESIDENT_PREFIX => IdType::Resident,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u32> for Id {
    type Error = ParseError;

    /// # Errors
    ///
    /// 1. No valid usage of the Luhn Algorithm.
    /// 2. More or less than 10 digits.
    /// 3. First digit is not 1 or 2.
    fn try_from(mut id: u32) -> Result<Self, Self::Error> {
        let mut digits: Vec<u8> = Vec::with_capacity(ID_SIZE);

        while id > 0 {
            digits.insert(0, (id % 10) as u8);
            id /= 10;
        }

        // Validate ID
        if Self::validate(&digits) {
            Ok(Self { digits })
        } else {
            Err(ParseError::InvalidId)
        }
    }
}

impl TryFrom<Vec<u8>> for Id {
    type Error = ParseError;

    /// # Errors
    ///
    /// 1. No valid usage of the Luhn Algorithm.
    /// 2. More or less than 10 digits.
    /// 3. First digit is not 1 or 2.
    fn try_from(digits: Vec<u8>) -> Result<Self, Self::Error> {
        // Validate ID
        if Self::validate(&digits) {
            Ok(Self { digits })
        } else {
            Err(ParseError::InvalidId)
        }
    }
}

impl core::str::FromStr for Id {
    type Err = ParseError;

    /// # Errors
    ///
    /// 1. String can't be parsed as an integer.
    /// 2. No valid usage of the Luhn Algorithm.
    /// 3. More or less than 10 digits.
    /// 4. First digit is not 1 or 2.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>().map_or(Err(ParseError::InvalidId), |num| {
            Self::try_from(num).map_or(Err(ParseError::InvalidId), Ok)
        })
    }
}

impl core::fmt::Display for Id {
    #[expect(clippy::unwrap_used, reason = "Should never fail")]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .clone()
                .into_iter()
                .enumerate()
                .fold(0, |acc: u32, (i, digit)| {
                    const ITER_INDEXES: usize = ID_SIZE - 1;

                    acc + (u32::from(digit)
                        * (10_u32.pow(u32::try_from(ITER_INDEXES - i).unwrap())))
                })
        )
    }
}

impl Clone for Id {
    fn clone(&self) -> Self {
        Self {
            digits: self.digits.clone(),
        }
    }
}

#[expect(clippy::allow_attributes_without_reason)]
#[expect(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[test]
    fn static_tests() {
        let id = Id::try_from(1_581_872_353).unwrap();
        assert_eq!(id.get_type(), IdType::Citizen);
    }

    #[test]
    fn random_generated_tests() {
        for _ in 0..10_000_usize {
            let cit_id = Id::new(&IdType::Citizen);
            assert_eq!(cit_id, Id::try_from(cit_id.digits.clone()).unwrap());
            assert_eq!(cit_id, Id::from_str(&cit_id.to_string()).unwrap());
            assert_eq!(
                cit_id.to_string(),
                cit_id
                    .digits
                    .clone()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            );

            let res_id = Id::new(&IdType::Resident);
            assert_eq!(res_id, Id::try_from(res_id.digits.clone()).unwrap());
            assert_eq!(res_id, Id::from_str(&res_id.to_string()).unwrap());
            assert_eq!(
                res_id.to_string(),
                res_id
                    .digits
                    .clone()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            );
        }
    }
}
