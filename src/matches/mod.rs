use rand::{Rng, rngs::ThreadRng};

pub struct Stats {
    team_name: String,
    score: u8,
}
impl Stats {
    fn new(name: &str) -> Self {
        Self {
            team_name: name.into(),
            score: 0,
        }
    }
    fn update_score(&mut self, new_score: u8) -> u8 {
        self.score = new_score;
        self.score
    }
    fn name(&self) -> &str {
        &self.team_name
    }
    pub fn score(&self) -> u8 {
        self.score
    }
}

pub struct Match {
    match_number: u16,
    home_team: Stats,
    away_team: Stats,
}

impl Match {
    pub fn new(match_number: u16, home_team: &str, away_teeam: &str) -> Self {
        Self {
            match_number,
            home_team: Stats::new(home_team),
            away_team: Stats::new(away_teeam),
        }
    }
    pub fn sim_match(&mut self, rng: &mut ThreadRng) -> &str {
        let upper: u8 = rng.random_range(0..=3);
        let home_score = self.home_team.update_score(rng.random_range(0..=upper));
        let upper2: u8 = rng.random_range(0..=3);
        let away_score = self.away_team.update_score(rng.random_range(0..=upper2));
        if home_score > away_score {
            self.home_team.name()
        } else if home_score < away_score {
            self.away_team.name()
        } else {
            "DRAW"
        }
    }
    pub fn get_home_team(&self) -> &str {
        self.home_team.name()
    }
    pub fn get_home_stats(&self) -> &Stats {
        &self.home_team
    }
    pub fn get_away_team(&self) -> &str {
        self.away_team.name()
    }
    pub fn get_away_stats(&self) -> &Stats {
        &self.away_team
    }
    pub fn match_num(&self) -> u16 {
        self.match_number
    }
}
