use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct BloodRules {
    starter_blood: u8,
}

impl BloodRules {
    pub fn starter_blood(&self) -> crate::currency::blood::Blood {
        crate::currency::blood::Blood { amount: self.starter_blood }
    }
}
