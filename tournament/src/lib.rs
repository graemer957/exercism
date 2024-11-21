use std::collections::HashMap;
use std::ops::Not;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default)]
struct Team<'a> {
    name: &'a str,
    played: u8,
    won: u8,
    drawn: u8,
    lost: u8,
    points: u8,
}

impl<'a> Team<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    fn win(&mut self) {
        self.played += 1;
        self.won += 1;
        self.points += 3;
    }

    fn draw(&mut self) {
        self.played += 1;
        self.drawn += 1;
        self.points += 1;
    }

    fn loss(&mut self) {
        self.played += 1;
        self.lost += 1;
    }

    fn played_match(&mut self, outcome: &Outcome) {
        match outcome {
            Outcome::Win => self.win(),
            Outcome::Draw => self.draw(),
            Outcome::Loss => self.loss(),
        }
    }

    fn result_entry(&self) -> String {
        format!(
            "\n{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name, self.played, self.won, self.drawn, self.lost, self.points
        )
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Not for Outcome {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Outcome::Win => Outcome::Loss,
            Outcome::Draw => Outcome::Draw,
            Outcome::Loss => Outcome::Win,
        }
    }
}

impl From<&str> for Outcome {
    fn from(outcome: &str) -> Self {
        match outcome {
            "win" => Outcome::Win,
            "draw" => Outcome::Draw,
            "loss" => Outcome::Loss,
            _ => panic!("Unknown outcome!"),
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, Team> = HashMap::new();

    for result in match_results.lines() {
        let mut components = result.split(';');
        if let (Some(first_team), Some(second_team), Some(outcome)) =
            (components.next(), components.next(), components.next())
        {
            let outcome = outcome.into();
            teams
                .entry(first_team)
                .or_insert_with(|| Team::new(first_team))
                .played_match(&outcome);
            teams
                .entry(second_team)
                .or_insert_with(|| Team::new(second_team))
                .played_match(&!outcome);
        }
    }

    let mut teams = teams.values().collect::<Vec<&Team>>();
    teams.sort_by(|lhs, rhs| {
        rhs.points
            .cmp(&lhs.points)
            .then_with(|| lhs.name.cmp(rhs.name))
    });

    HEADER.to_string()
        + &teams
            .iter()
            .map(|team| team.result_entry())
            .collect::<String>()
}
