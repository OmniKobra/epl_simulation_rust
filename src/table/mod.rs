use rand::rngs::ThreadRng;

use super::Match;
use super::Team;
use std::collections::HashMap;
pub struct Table {
    standings: HashMap<u8, Team>,
    matches: Vec<Match>,
    match_count: u16,
    rng: ThreadRng,
}
impl Table {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        let teams: Vec<Team> = vec![
            "AFC Bournemouth",
            "Arsenal",
            "Aston Villa",
            "Brentford",
            "Brighton & Hove Albion",
            "Burnley",
            "Chelsea",
            "Crystal Palace",
            "Everton",
            "Fulham",
            "Leeds United",
            "Liverpool",
            "Manchester City",
            "Manchester United",
            "Newcastle United",
            "Nottingham Forest",
            "Sunderland",
            "Tottenham Hotspur",
            "West Ham United",
            "Wolverhampton Wanderers",
        ]
        .into_iter()
        .map(|t| Team::new(t))
        .collect();
        for (i, t) in teams.into_iter().enumerate() {
            map.insert((i + 1) as u8, t);
        }
        Self {
            standings: map,
            matches: vec![],
            match_count: 0,
            rng: rand::rng(),
        }
    }
}
