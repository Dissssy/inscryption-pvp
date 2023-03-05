use crate::util::FixedOrVariable;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MoxRules {
    pub green: FixedOrVariable<bool>,
    pub orange: FixedOrVariable<bool>,
    pub blue: FixedOrVariable<bool>,
}

impl MoxRules {
    pub fn starter_mox(&self) -> crate::currency::mox::Mox {
        crate::currency::mox::Mox {
            green: self.green,
            orange: self.orange,
            blue: self.blue,
        }
    }
}
