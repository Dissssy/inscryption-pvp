use crate::util::Direction;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Sigil {
    MightyLeap,
    BifurcatedStrike,
    TrifurcatedStrike,
    FrozenAway,
    SteelTrap,
    RabbitHole,
    Sprinter(Direction),
    TouchOfDeath,
    Fledgeling(u8),
    Burrower,
    Fecundity,
    BoneKing,
    Unkillable,
    SharpQuills,
    Hefty(Direction),
    Guardian,
    Airborne,
    ManyLives,
    Repulsive,
    WorthySacrifice,
    BoneDigger,
    Brittle,
    SkeletonCrew(Direction),
    Mox(super::mox::Mox),
    GemAnimator,
    RubyHeart,
    MentalGemnastics,
    GemDependent,
    Handy,
    SquirrelShedder(Direction),
    AttackConduit,
    SpawnConduit,
    NullConduit,
    BatteryBearer,
    Detonator,
    Sentry,
    EnergyConduit,
    BombSpewer,
    DoubleDeath,
    PowerDice,
    Enlarge,
    Disentomb,
    EnergyGun,
    Looter,
    TrueScholar,
    Stimulate,
    BoneHorn,
    Waterborne,
    KrakenWaterborne,
}

impl Sigil {
    pub fn power(&self) -> i8 {
        match self {
            Sigil::MightyLeap => 1,
            Sigil::BifurcatedStrike => 4,
            Sigil::TrifurcatedStrike => 5,
            Sigil::FrozenAway => 3,
            Sigil::SteelTrap => 5,
            Sigil::RabbitHole => 3,
            Sigil::Sprinter(_) => 1,
            Sigil::TouchOfDeath => 4,
            Sigil::Fledgeling(_) => 2,
            Sigil::Burrower => 1,
            Sigil::Fecundity => 3,
            Sigil::BoneKing => 2,
            Sigil::Unkillable => 2,
            Sigil::SharpQuills => 2,
            Sigil::Hefty(_) => 1,
            Sigil::Guardian => 1,
            Sigil::Airborne => 0,
            Sigil::ManyLives => 4,
            Sigil::Repulsive => 4,
            Sigil::WorthySacrifice => 2,
            Sigil::BoneDigger => 2,
            Sigil::Brittle => -2,
            Sigil::SkeletonCrew(_) => 3,
            Sigil::Mox(m) => m.green as i8 + m.orange as i8 + m.blue as i8 + 1,
            Sigil::GemAnimator => 3,
            Sigil::RubyHeart => 3,
            Sigil::MentalGemnastics => 2,
            Sigil::GemDependent => -3,
            Sigil::Handy => 4,
            Sigil::SquirrelShedder(_) => 3,
            Sigil::AttackConduit => 3,
            Sigil::SpawnConduit => 3,
            Sigil::NullConduit => 1,
            Sigil::BatteryBearer => 2,
            Sigil::Detonator => 0,
            Sigil::Sentry => 3,
            Sigil::EnergyConduit => 2,
            Sigil::BombSpewer => 4,
            Sigil::DoubleDeath => 3,
            Sigil::PowerDice => 5,
            Sigil::Enlarge => 4,
            Sigil::Disentomb => 3,
            Sigil::EnergyGun => 4,
            Sigil::Looter => 4,
            Sigil::TrueScholar => 3,
            Sigil::Stimulate => 4,
            Sigil::BoneHorn => 4,
            Sigil::Waterborne => 1,
            Sigil::KrakenWaterborne => 2,
        }
    }
}
