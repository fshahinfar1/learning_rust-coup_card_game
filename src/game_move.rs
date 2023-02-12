pub enum Move {
    TakeOneCoin,    // Everyone can play this
    TakeThreeCoin,  // Only Oligarch can play this
    TakeTaxFrom,    // Only Commander can play this
    Assassinate,    // Only Assassin can play this
    ChangeCards,    // Only Ambassador can play this
    Coup,           // Everyone who has 7 coins or more
}

impl Move {
    pub fn name(&self) -> &str {
        match self {
            Move::TakeOneCoin => {
                "take one coin"
            },
            Move::TakeThreeCoin => {
                "take three coin"
            },
            Move::TakeTaxFrom => {
                "take tax from"
            },
            Move::Assassinate => {
                "assassinate"
            },
            Move::ChangeCards => {
                "change card"
            },
            Move::Coup => {
                "coup!"
            },
        }
    }
}
