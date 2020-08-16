use std::fmt;

#[derive(Clone, Debug)]
pub struct Field {
    pub values: Vec<u8>,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            values: (1..9).collect(),
        }
    }
}

impl Field {
    pub fn settled(v: u8) -> Field {
        Field { values: vec![v] }
    }
    pub fn is_valid(&self) -> bool {
        self.values.len() > 0
    }
    pub fn is_settled(&self) -> bool {
        self.values.len() == 1
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_settled() {
            write!(f, "{}", self.values[0])
        } else {
            write!(f, "_")
        }
    }
}
