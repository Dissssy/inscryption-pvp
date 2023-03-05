use anyhow::Result;

pub struct Board {
    field: Box<[Option<crate::card::Card>]>,
    deck: Vec<crate::card::Card>,
    hand: Vec<crate::card::Card>,
    balance: crate::currency::Balance,
}

impl Board {
    pub fn new(rules: &crate::rules::Rules, deck: Vec<crate::card::Card>) -> Result<Self> {
        // validate the deck
        rules.validate_deck(&deck)?;
        Ok(Self {
            field: rules.get_field(),
            deck,
            hand: Vec::new(),
            balance: crate::currency::Balance::new(rules),
        })
    }
}
