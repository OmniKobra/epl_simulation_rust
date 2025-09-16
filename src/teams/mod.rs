use std::cell::RefCell;

#[derive(Default, Clone)]
pub struct Team {
    name: String,
    pts: u8,
    matches_played: u8,
    goal_difference: RefCell<i16>,
    matches: Vec<u16>,
}
impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn pts(&self) -> u8 {
        self.pts
    }
    pub fn played(&self) -> u8 {
        self.matches_played
    }
    pub fn resolve_match(&mut self, pts: u8, match_num: u16) {
        self.pts += pts;
        self.matches_played += 1;
        self.matches.push(match_num);
    }
    pub fn gd(&self) -> i16 {
        *self.goal_difference.borrow()
    }
    pub fn set_gd(&self, num: i16){
        *self.goal_difference.borrow_mut() += num;
    }
}
