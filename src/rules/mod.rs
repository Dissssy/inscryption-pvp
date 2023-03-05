use anyhow::Result;
use typed_builder::TypedBuilder;
pub mod blood;
pub mod bones;
pub mod deck;
pub mod energy;
pub mod mox;
use crate::util::{Change, FixedOrVariable, Range};

#[derive(TypedBuilder)]
pub struct Rules {
    energy: energy::EnergyRules,
    bones: bones::BoneRules,
    mox: mox::MoxRules,
    blood: blood::BloodRules,
    deck_rules: deck::DeckRules,
    field_size: usize,
}

impl Rules {
    pub fn energy_rules(&self) -> energy::EnergyRules {
        self.energy
    }
    pub fn new(preset: Preset) -> Self {
        match preset {
            Preset::Vanilla => Self::builder()
                .energy(
                    energy::EnergyRules::builder()
                        .max_max(6)
                        .increase_amount(Change::Fixed(1))
                        .recharge(Change::PercentageMax(1.))
                        .starter_amount(1)
                        .starter_energy(Change::Fixed(1))
                        .build(),
                )
                .bones(bones::BoneRules::builder().starter_bones(0).gain_multiplier(1.).build())
                .mox(
                    mox::MoxRules::builder()
                        .green(FixedOrVariable::Variable(false))
                        .orange(FixedOrVariable::Variable(false))
                        .blue(FixedOrVariable::Variable(false))
                        .build(),
                )
                .deck_rules(
                    deck::DeckRules::builder()
                        .quantity(Range::new(20, 60).expect("You should get that checked out"))
                        .power(Range::new(0, 100).expect("You should get that checked out"))
                        .duplicates(3)
                        .build(),
                )
                .blood(blood::BloodRules::builder().starter_blood(0).build())
                .field_size(4),
        }
        .build()
    }
    pub fn starter_bones(&self) -> crate::currency::bones::Bones {
        self.bones.starter_bones()
    }
    pub fn starter_energy(&self) -> crate::currency::energy::Energy {
        self.energy.starter_energy()
    }
    pub fn starter_mox(&self) -> crate::currency::mox::Mox {
        self.mox.starter_mox()
    }
    pub fn starter_blood(&self) -> crate::currency::blood::Blood {
        self.blood.starter_blood()
    }
    pub fn validate_deck(&self, deck: &Vec<crate::card::Card>) -> Result<()> {
        self.deck_rules.validate_deck(deck)
    }
    pub fn get_field(&self) -> Box<[Option<crate::card::Card>]> {
        vec![None; self.field_size].into_boxed_slice()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Preset {
    Vanilla,
}
