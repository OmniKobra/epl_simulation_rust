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
const team_names: [&str; 20] = [
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
];
impl Table {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        let teams: Vec<Team> = team_names.iter().map(|t| Team::new(t)).collect();
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
    pub fn show_teams_stats(&self) -> String {
        let mut output = String::new();

        output.push_str("+------+-------------------------+-----+-----+-----+-----+------+------+------+-------+\n");
        output.push_str("| Pos  | Team                    |  P  |  W  |  D  |  L  |  GF  |  GA  |  GD  |  Pts  |\n");
        output.push_str("+------+-------------------------+-----+-----+-----+-----+------+------+------+-------+\n");
        let foo = "foo";
        let mut entries: Vec<_> = self.standings.iter().collect();
        entries.sort_by_key(|(pos, _)| *pos);
        for (pos, t) in entries {
            let row = format!(
                "|{:^6}| {:<24}|{:^5}|{:^5}|{:^5}|{:^5}|{:^6}|{:^6}|{:^6}|{:^7}|\n",
                pos,
                t.name(),
                foo,
                foo,
                foo,
                foo,
                foo,
                foo,
                foo,
                foo,
            );
            output.push_str(&row);
        }
        output.push_str("+------+-------------------------+-----+-----+-----+-----+------+------+------+-------+\n");

        output
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show_teams_stats())
    }
}
