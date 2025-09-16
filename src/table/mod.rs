use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use super::Match;
use super::Team;
use std::collections::HashMap;
#[derive(Default)]
pub struct Table {
    standings: HashMap<u8, Team>,
    matches: Vec<Match>,
    match_count: u16,
    rng: ThreadRng,
}
const TEAM_NAMES: [&str; 20] = [
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
        let teams: Vec<Team> = TEAM_NAMES.iter().map(|t| Team::new(t)).collect();
        for (i, t) in teams.into_iter().enumerate() {
            map.insert((i + 1) as u8, t);
        }
        let mut s = Self {
            standings: map,
            matches: vec![],
            match_count: 0,
            rng: rand::rng(),
        };
        s.gen_all_matches();
        s
    }

    pub fn show_teams_stats(&self) -> String {
        let mut output = String::new();

        output.push_str("+------+-------------------------+-----+-----+-----+-----+------+------+------+-------+\n");
        output.push_str("| Pos  | Team                    |  P  |  W  |  D  |  L  |  GF  |  GA  |  GD  |  Pts  |\n");
        output.push_str("+------+-------------------------+-----+-----+-----+-----+------+------+------+-------+\n");
        let mut entries: Vec<_> = self.standings.iter().collect();
        entries.sort_by_key(|(pos, _)| *pos);
        for (pos, t) in entries {
            let mut goals_for: u16 = 0;
            let mut goals_against: u16 = 0;
            let wins: Vec<&Match> = self
                .matches
                .iter()
                .filter(|m| {
                    if m.get_away_team() == t.name() {
                        let stats = m.get_away_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_home_stats().score() as u16;
                        if stats.score() > m.get_home_stats().score() {
                            return true;
                        }
                    }
                    if m.get_home_team() == t.name() {
                        let stats = m.get_home_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_away_stats().score() as u16;
                        if m.get_away_stats().score() < stats.score() {
                            return true;
                        }
                    }
                    false
                })
                .collect();
            let losses: Vec<&Match> = self
                .matches
                .iter()
                .filter(|m| {
                    if m.get_away_team() == t.name() {
                        let stats = m.get_away_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_home_stats().score() as u16;
                        if stats.score() < m.get_home_stats().score() {
                            return true;
                        }
                    }
                    if m.get_home_team() == t.name() {
                        let stats = m.get_home_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_away_stats().score() as u16;
                        if m.get_away_stats().score() > stats.score() {
                            return true;
                        }
                    }
                    false
                })
                .collect();
            let draws: Vec<&Match> = self
                .matches
                .iter()
                .filter(|m| {
                    if m.get_away_team() == t.name() {
                        let stats = m.get_away_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_home_stats().score() as u16;
                        if stats.score() == m.get_home_stats().score() {
                            return true;
                        }
                    }
                    if m.get_home_team() == t.name() {
                        let stats = m.get_home_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_away_stats().score() as u16;
                        if m.get_away_stats().score() == stats.score() {
                            return true;
                        }
                    }
                    false
                })
                .collect();
            let gd: i16 = goals_for as i16 - goals_against as i16;
            t.set_gd(gd);

            let row = format!(
                "|{:^6}| {:<24}|{:^5}|{:^5}|{:^5}|{:^5}|{:^6}|{:^6}|{:^6}|{:^7}|\n",
                pos,
                t.name(),
                t.played(),
                wins.len(),
                draws.len(),
                losses.len(),
                goals_for,
                goals_against,
                gd,
                t.pts(),
            );
            output.push_str(&row);
        }
        output.push_str("+------+-------------------------+-----+-----+-----+-----+------+------+------+-------+\n");

        output
    }
    fn update_table(&mut self) {
        let mut entries: Vec<_> = self.standings.iter().collect();
        entries.sort_by_key(|(pos, _)| *pos);
        for (_, t) in entries {
            let mut goals_for: u16 = 0;
            let mut goals_against: u16 = 0;
            let _: Vec<&Match> = self
                .matches
                .iter()
                .filter(|m| {
                    if m.get_away_team() == t.name() {
                        let stats = m.get_away_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_home_stats().score() as u16;
                        if stats.score() > m.get_home_stats().score() {
                            return true;
                        }
                    }
                    if m.get_home_team() == t.name() {
                        let stats = m.get_home_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_away_stats().score() as u16;
                        if m.get_away_stats().score() < stats.score() {
                            return true;
                        }
                    }
                    false
                })
                .collect();
            let _: Vec<&Match> = self
                .matches
                .iter()
                .filter(|m| {
                    if m.get_away_team() == t.name() {
                        let stats = m.get_away_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_home_stats().score() as u16;
                        if stats.score() < m.get_home_stats().score() {
                            return true;
                        }
                    }
                    if m.get_home_team() == t.name() {
                        let stats = m.get_home_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_away_stats().score() as u16;
                        if m.get_away_stats().score() > stats.score() {
                            return true;
                        }
                    }
                    false
                })
                .collect();
            let _: Vec<&Match> = self
                .matches
                .iter()
                .filter(|m| {
                    if m.get_away_team() == t.name() {
                        let stats = m.get_away_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_home_stats().score() as u16;
                        if stats.score() == m.get_home_stats().score() {
                            return true;
                        }
                    }
                    if m.get_home_team() == t.name() {
                        let stats = m.get_home_stats();
                        goals_for += stats.score() as u16;
                        goals_against += m.get_away_stats().score() as u16;
                        if m.get_away_stats().score() == stats.score() {
                            return true;
                        }
                    }
                    false
                })
                .collect();
            let gd: i16 = goals_for as i16 - goals_against as i16;
            t.set_gd(gd);
        }

        let mut entries: Vec<_> = self.standings.drain().collect();
        entries.sort_by(|(_, a), (_, b)| {
            if a.pts() == b.pts() {
                b.gd().cmp(&a.gd())
            } else {
                b.pts().cmp(&a.pts())
            }
        });

        self.standings = entries
            .into_iter()
            .enumerate()
            .map(|(i, (_, team))| ((i + 1) as u8, team))
            .collect();
    }
    fn gen_all_matches(&mut self) {
        let it: Vec<_> = self.standings.iter().collect();
        for (_, team) in &it[..] {
            self.match_count += 1;
            for (_, opponent) in &it[..] {
                if team.name() != opponent.name() {
                    self.matches
                        .push(Match::new(self.match_count, team.name(), opponent.name()));
                }
            }
        }
        self.matches.shuffle(&mut self.rng);
    }
    pub fn sim_all_matches(&mut self) {
        for m in self.matches.iter_mut() {
            let result = m.sim_match(&mut self.rng).to_owned();
            let match_num = m.match_num();
            let home_name = m.get_home_team();
            let away_name = m.get_away_team();
            let (home_pts, away_pts) = if result == "DRAW" {
                (1, 1)
            } else if result == home_name {
                (3, 0)
            } else {
                (0, 3)
            };

            let (_, home) = self
                .standings
                .iter_mut()
                .find(|(_, t)| t.name() == home_name)
                .unwrap();
            home.resolve_match(home_pts, m.match_num());

            let (_, away) = self
                .standings
                .iter_mut()
                .find(|(_, t)| t.name() == away_name)
                .unwrap();
            away.resolve_match(away_pts, match_num);
        }
        self.update_table();
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show_teams_stats())
    }
}
