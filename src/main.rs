use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use hashbrown::HashMap;
use crate::game::*;
use crate::player::Player;
use crate::team::*;

pub mod game;
pub mod team;
pub mod player;

fn read(path: &str) -> String {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

fn main() {
    let teams = parse_teams(read("data/teams.csv"));

    let mut players: HashMap<String, Player> = teams.values().flat_map(|team| {
        team.players.iter().cloned().map(|p| (p.to_lowercase().clone(), Player::new(p, team.name.clone())))
    }).filter(|p| !p.1.name.is_empty()).collect();

    let mut agents = HashMap::new();

    let mut games = 0;
    for game in fs::read_dir("data/matches/").unwrap() {
        let game = game.unwrap();
        let path = game.path().to_str().unwrap().to_string();
        let data = parse_game(&path, read(&path));
        apply(&mut players, &teams, &mut agents, data);
        games += 1;
    }

    let mut s = Player::csv_header();

    for p in players.values_mut() {
        p.finish();
        s.push_str(&p.to_csv());
    }

    let mut output = OpenOptions::new().write(true).truncate(true).create(true).open("data/stats.csv").unwrap();
    write!(&mut output, "{}", s).unwrap();

    let csv = write_agents(agents, games);
    let mut agents = OpenOptions::new().write(true).truncate(true).create(true).open("data/agents.csv").unwrap();
    write!(&mut agents, "{}", csv).unwrap();
}
