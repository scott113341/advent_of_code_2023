use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct History {
    pub values: Vec<isize>,
}

impl FromStr for History {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(History {
            values: s.split_whitespace().map(|v| v.parse().unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_thing_from_str() {
        assert_eq!(
            get_input::<History>(Input::Test1)[0],
            History {
                values: vec![0, 3, 6, 9, 12, 15]
            }
        );
    }
}
