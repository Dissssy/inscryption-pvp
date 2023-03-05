pub struct Energy {
    pub energy: u8,
    pub max_energy: u8,
}

impl Energy {
    pub fn next_turn(&mut self, rules: &crate::rules::Rules) {
        // max energy increases by rules.max_energy_increase every turn up to rules.max_energy
        let e = rules.energy_rules();
        e.increase(&mut self.max_energy);
        e.recharge(&mut self.energy);
    }
}
