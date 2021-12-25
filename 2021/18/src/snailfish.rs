use std::ops::Add;
use std::{fmt::Write, str::FromStr};

pub struct Number {
    left: Box<Option<Number>>,
    right: Box<Option<Number>>,
    num: Option<u8>,
}

impl Number {
    fn pair(left: Option<Number>, right: Option<Number>) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            num: None,
        }
    }

    fn regular(num: u8) -> Self {
        Self {
            left: Box::new(None),
            right: Box::new(None),
            num: Some(num),
        }
    }

    pub fn magnitude(&self) -> usize {
        if let Some(num) = self.num {
            num as usize
        } else {
            (3 * self.left.as_ref().as_ref().unwrap().magnitude())
                + (2 * self.right.as_ref().as_ref().unwrap().magnitude())
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> <Self as std::ops::Add<Self>>::Output {
        Number::pair(Some(self), Some(rhs))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNumError;

impl FromStr for Number {
    type Err = ParseNumError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        if s.starts_with('[') {
            // find the pair split point
            let mut split_idx_tracker = 0;
            let mut split_idx = 0;
            for (i, c) in s.chars().enumerate() {
                match c {
                    '[' => split_idx_tracker += 1,
                    ']' => split_idx_tracker -= 1,
                    ',' if split_idx_tracker <= 1 => {
                        split_idx = i;
                        break;
                    }
                    _ => (),
                }
            }

            let l = &s[1..split_idx];
            let r = &s[split_idx + 1..s.len() - 1];

            Ok(Number::pair(l.parse().ok(), r.parse().ok()))
        } else if let Ok(n) = s.parse() {
            Ok(Number::regular(n))
        } else {
            Err(ParseNumError)
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let Some(n) = self.num {
            f.write_char(char::from_digit(n as u32, 10).unwrap())
        } else {
            f.debug_list()
                .entry(&self.left.as_ref().as_ref().unwrap())
                .entry(&self.right.as_ref().as_ref().unwrap())
                .finish()
        }
    }
}
