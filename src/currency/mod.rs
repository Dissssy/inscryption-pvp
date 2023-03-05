pub mod blood;
pub mod bones;
pub mod energy;
pub mod mox;

pub struct Balance {
    pub bones: bones::Bones,
    pub energy: energy::Energy,
    pub mox: mox::Mox,
    pub blood: blood::Blood,
}

impl Balance {
    pub fn new(rules: &crate::rules::Rules) -> Self {
        Self {
            bones: rules.starter_bones(),
            energy: rules.starter_energy(),
            mox: rules.starter_mox(),
            blood: rules.starter_blood(),
        }
    }
}
