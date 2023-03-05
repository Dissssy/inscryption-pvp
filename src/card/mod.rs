pub mod mox;
pub mod sigil;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Card {
    pub cost: Cost,
    pub sigils: Vec<sigil::Sigil>,
    pub name: String,
    pub description: String,
    pub attack: Attack,
    pub health: u8,
}

impl Card {
    pub fn power(&self) -> i8 {
        self.sigils.iter().map(|s| s.power()).sum::<i8>() + (self.attack.power() * 2) + self.health as i8
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Cost {
    Free,
    Bones(u8),
    Energy(u8),
    Blood(u8),
    Mox(mox::Mox),
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Attack {
    Normal(u8),
    Ant,
    Mirror,
    BellRinger,
    CardCounter,
    GreenGems,
}

impl Attack {
    pub fn power(&self) -> i8 {
        match self {
            Attack::Normal(n) => *n as i8,
            Attack::Ant => 2,
            Attack::Mirror => 1,
            Attack::BellRinger => 2,
            Attack::CardCounter => 3,
            Attack::GreenGems => 2,
        }
    }
}
