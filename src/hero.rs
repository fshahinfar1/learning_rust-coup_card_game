// Define some types
#[derive(Copy, Clone, PartialEq)]
pub enum Hero {
    Empty,
    Assassin,
    Commander,
    Oligarch,
    Princess,
    Ambassador,
}

impl Hero {
    pub fn get_name(&self) -> &str {
        // TODO: match and return a string
        match self {
            Hero::Empty => "Empty",
            Hero::Assassin => "Assassin",
            Hero::Commander => "Commander",
            Hero::Oligarch => "Oligarch",
            Hero::Princess => "Princess",
            Hero::Ambassador => "Ambassador",
        }
    }
}
