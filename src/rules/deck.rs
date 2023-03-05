use crate::util::{Range, RangeComparison};
use anyhow::{anyhow, Result};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct DeckRules {
    quantity: Range<u8>,
    power: Range<i16>,
    duplicates: u8,
}

impl DeckRules {
    pub fn validate_deck(&self, deck: &Vec<crate::card::Card>) -> Result<()> {
        match self.quantity.compare(deck.len() as u8) {
            RangeComparison::GreaterThanMax => return Err(anyhow!("Deck has too many cards: {}", deck.len())),
            RangeComparison::LessThanMin => return Err(anyhow!("Deck has too few cards: {}", deck.len())),
            _ => (),
        }
        let p = deck.iter().map(|c| c.power()).sum::<i8>() as i16;
        match self.power.compare(p) {
            RangeComparison::GreaterThanMax => return Err(anyhow!("Deck has too much power: {}", p)),
            RangeComparison::LessThanMin => return Err(anyhow!("Deck has too little power: {}", p)),
            _ => (),
        }
        // ensure the deck contains no more than self.duplicates of any one card
        let mut counts = std::collections::HashMap::new();
        for c in deck {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
            if *count > self.duplicates {
                return Err(anyhow!("Deck contains too many copies of card: {}", c.name));
            }
        }
        Ok(())
    }
}
