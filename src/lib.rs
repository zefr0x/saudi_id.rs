//! Parse (validate) from multible data types or generate new random [Saudi Arabian national IDs](https://en.wikipedia.org/wiki/Saudi_Arabian_identity_card).
//!
//! Used to validate IDs and find thier type (Citizen or Resident), or used to test software by generating random valid IDs.

extern crate luhnr;

#[derive(PartialEq, Debug)]
pub enum IdType {
    Citizen,
    Resident,
}

pub struct Id {
    pub digits: Vec<u8>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidId,
}

const ID_SIZE: usize = 10;
const CITIZEN_PREFIX: u8 = 1;
const RESIDENT_PREFIX: u8 = 2;

impl Id {
    /// Create a new random ID
    pub fn new(id_type: IdType) -> Self {
        match id_type {
            IdType::Citizen => {
                let digits = luhnr::generate_with_prefix(ID_SIZE, &[CITIZEN_PREFIX]).unwrap();

                Id { digits }
            }
            IdType::Resident => {
                let digits = luhnr::generate_with_prefix(ID_SIZE, &[RESIDENT_PREFIX]).unwrap();

                Id { digits }
            }
        }
    }

    fn validate(digits: &Vec<u8>) -> bool {
        luhnr::validate(digits) && digits.len() == ID_SIZE
    }

    pub fn get_type(&self) -> IdType {
        match self.digits[0] {
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
    /// 1. Not using the Luhn Algorithm.
    /// 2. More or less then 10 digits.
    /// 3. First digit is not 1 or 2.
    fn try_from(mut id: u32) -> Result<Self, Self::Error> {
        let mut digits: Vec<u8> = Vec::with_capacity(ID_SIZE);

        while id > 0 {
            digits.insert(0, (id % 10) as u8);
            id /= 10
        }

        // Validate ID
        if Id::validate(&digits) {
            Ok(Id { digits })
        } else {
            Err(ParseError::InvalidId)
        }
    }
}

impl TryFrom<Vec<u8>> for Id {
    type Error = ParseError;

    /// # Errors
    ///
    /// 1. Not using the Luhn Algorithm.
    /// 2. More or less then 10 digits.
    /// 3. First digit is not 1 or 2.
    fn try_from(digits: Vec<u8>) -> Result<Self, Self::Error> {
        // Validate ID
        if Id::validate(&digits) {
            Ok(Id { digits })
        } else {
            Err(ParseError::InvalidId)
        }
    }
}

impl std::str::FromStr for Id {
    type Err = ParseError;

    /// # Errors
    ///
    /// 1. String can't be parsed as an integer.
    /// 2. Not using the Luhn Algorithm.
    /// 3. More or less then 10 digits.
    /// 4. First digit is not 1 or 2.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(num) => match Id::try_from(num) {
                Ok(id) => Ok(id),
                Err(_) => Err(ParseError::InvalidId),
            },
            Err(_) => Err(ParseError::InvalidId),
        }
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.digits
            .clone()
            .into_iter()
            .map(|digit| digit.to_string())
            .collect()
    }
}

impl Clone for Id {
    fn clone(&self) -> Self {
        Id {
            digits: self.digits.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn static_tests() {
        let id = Id::try_from(1581872353).unwrap();
        assert_eq!(id.get_type(), IdType::Citizen);
    }

    #[test]
    fn random_generated_tests() {
        for _ in 0..10000 {
            let citizen_id = Id::new(IdType::Citizen);
            Id::try_from(citizen_id.digits.clone()).unwrap();
            Id::from_str(&citizen_id.to_string()).unwrap();

            let resident_id = Id::new(IdType::Resident);
            Id::try_from(resident_id.digits.clone()).unwrap();
            Id::from_str(&resident_id.to_string()).unwrap();
        }
    }
}
