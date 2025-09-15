use rand::rngs::ThreadRng;

#[derive(Default)]
enum MatchResult {
    Win,
    Loss,
    #[default]
    Draw,
}

#[derive(Default)]
struct Stats {
    team_name: String,
    score: u8,
    fouls: u8,
    yellow_cards: u8,
    red_cards: u8,
    offsides: u8,
    result: MatchResult,
}

#[derive(Default)]
pub struct Match {
    match_number: u16,
    home_team: Stats,
    away_team: Stats,
}

impl Match {
    fn new() -> Self {
        Default::default()
    }
}
