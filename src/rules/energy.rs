use typed_builder::TypedBuilder;

use crate::util::Change;

#[derive(TypedBuilder, Clone, Copy)]
pub struct EnergyRules {
    max_max: u8,
    increase_amount: Change,
    recharge: Change,
    starter_amount: u8,
    starter_energy: Change,
}

impl EnergyRules {
    pub fn recharge(&self, energy: &mut u8) {
        let change_to = match self.recharge {
            Change::PercentageMax(p) => (self.max_max as f32 * p).ceil() as u8 + *energy,
            Change::PercentageAmount(p) => (*energy as f32 * p).ceil() as u8 + *energy,
            Change::Fixed(f) => f + *energy,
        };
        *energy = std::cmp::min(change_to, self.max_max);
    }

    pub fn increase(&self, max: &mut u8) {
        let change_to = match self.increase_amount {
            Change::PercentageMax(p) => (self.max_max as f32 * p).ceil() as u8 + *max,
            Change::PercentageAmount(p) => (*max as f32 * p).ceil() as u8 + *max,
            Change::Fixed(f) => f + *max,
        };
        *max = std::cmp::min(change_to, self.max_max);
    }

    pub(crate) fn starter_energy(&self) -> crate::currency::energy::Energy {
        crate::currency::energy::Energy {
            max_energy: self.starter_amount,
            energy: {
                let f = match self.starter_energy {
                    Change::PercentageMax(p) => (self.max_max as f32 * p).ceil() as u8,
                    Change::PercentageAmount(p) => (self.starter_amount as f32 * p).ceil() as u8,
                    Change::Fixed(f) => f,
                };
                std::cmp::min(f, self.starter_amount)
            },
        }
    }
}
