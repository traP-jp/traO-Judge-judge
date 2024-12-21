use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Clone)]
pub enum Phase {
    BeforeTest,
    Test,
    AfterTest,
}

impl Display for Phase {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Phase::BeforeTest => write!(f, "BeforeTest"),
            Phase::Test => write!(f, "Test"),
            Phase::AfterTest => write!(f, "AfterTest"),
        }
    }
}
