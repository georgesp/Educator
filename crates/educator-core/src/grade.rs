use std::fmt;
use std::str::FromStr;

/// School grade (French primary school, CP to CM2). Stored and exchanged by name (`"CE1"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Grade {
    CP,
    CE1,
    CE2,
    CM1,
    CM2,
}

impl Grade {
    pub const ALL: [Grade; 5] = [Grade::CP, Grade::CE1, Grade::CE2, Grade::CM1, Grade::CM2];

    pub fn as_str(self) -> &'static str {
        match self {
            Grade::CP => "CP",
            Grade::CE1 => "CE1",
            Grade::CE2 => "CE2",
            Grade::CM1 => "CM1",
            Grade::CM2 => "CM2",
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Unknown grade name (case-sensitive, like the content files and the stored values).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownGrade(pub String);

impl fmt::Display for UnknownGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown grade '{}'.", self.0)
    }
}

impl std::error::Error for UnknownGrade {}

impl FromStr for Grade {
    type Err = UnknownGrade;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Grade::ALL
            .into_iter()
            .find(|grade| grade.as_str() == value)
            .ok_or_else(|| UnknownGrade(value.to_owned()))
    }
}
