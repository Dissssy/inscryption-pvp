use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct BoneRules {
    starter_bones: u8,
    gain_multiplier: f32,
}

impl BoneRules {
    pub fn starter_bones(&self) -> crate::currency::bones::Bones {
        crate::currency::bones::Bones { bones: self.starter_bones }
    }
}
