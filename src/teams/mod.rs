use super::Match;

#[derive(Default)]
pub struct Team {
    name: String,
    pts: u8,
    matches_played: u8,
    matches: Vec<u16>,
}
impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}
