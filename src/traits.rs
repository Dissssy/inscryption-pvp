pub trait Next {
    fn next_turn(&mut self, rules: &crate::rules::Rules);
    fn next_move(&mut self, rules: &crate::rules::Rules);
}
