use crate::card::sigil::Sigil;
use crate::traits::Next;

pub struct Blood {
    pub amount: u8,
}

impl Blood {
    pub fn sacrifice(&mut self, card: &crate::card::Card) -> bool {
        let (dies, amount) = Self::get_sacrifice_cost(card);
        self.amount += amount;
        dies
    }
    fn get_sacrifice_cost(card: &crate::card::Card) -> (bool, u8) {
        let mut dies = true;
        let mut amount = 1;
        for sigil in card.sigils.iter() {
            match sigil {
                Sigil::ManyLives => {
                    dies = false;
                }
                Sigil::WorthySacrifice => {
                    amount = 3;
                }
                _ => {}
            }
        }
        (dies, amount)
    }
}

impl Next for Blood {
    fn next_turn(&mut self, rules: &crate::rules::Rules) {
        //
    }
    fn next_move(&mut self, rules: &crate::rules::Rules) {
        self.amount = rules.starter_blood().amount;
    }
}
