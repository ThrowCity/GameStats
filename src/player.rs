use hashbrown::HashMap;
use crate::game::GameStats;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub gp: u32,
    pub gw: u32,
    pub gl: u32,
    pub rw: u32,
    pub rl: u32,
    pub stats: Vec<GameStats>,
    pub averaged: Option<PlayerStats>,
    pub agents: HashMap<String, u32>,
}

impl Player {
    pub fn new(name: String, team: String) -> Self {
        Player {
            name,
            team,
            gp: 0,
            gw: 0,
            gl: 0,
            rw: 0,
            rl: 0,
            stats: vec![],
            averaged: None,
            agents: HashMap::new(),
        }
    }

    pub fn finish(&mut self) {
        let sum: GameStats = self.stats.drain(..).sum();
        let games = self.gp as f32;
        let mut sub_tracker_games = self.gp as f32 - sum.sub_tracker as f32;
        if sub_tracker_games <= 0.0 {
            sub_tracker_games = 1.0;
        }
        self.averaged = Some(PlayerStats {
            combat_score: sum.combat_score as f32 / games,
            kills: sum.kills,
            kpg: sum.kills as f32 / games,
            deaths: sum.deaths,
            dpg: sum.deaths as f32 / games,
            assists: sum.assists,
            apg: sum.assists as f32 / games,
            kd: sum.kd / games,
            hs: sum.hs as f32 / sub_tracker_games,
        });
    }

    pub fn to_csv(&self) -> String {
        let averaged = self.averaged.as_ref().unwrap();
        format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.name,
            self.agents.keys().cloned().collect::<Vec<_>>().join(" "),
            self.gp,
            self.gw,
            self.gl,
            self.rw,
            self.rl,
            averaged.combat_score,
            averaged.kills,
            averaged.kpg,
            averaged.deaths,
            averaged.dpg,
            averaged.assists,
            averaged.apg,
            averaged.kd,
            averaged.hs,
        )
    }

    pub fn csv_header() -> String {
        format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            "Name",
            "Agents",
            "Games played",
            "Games won",
            "Games lost",
            "Total score",
            "Total loss score",
            "ACS",
            "Kills",
            "Kills / Game",
            "Deaths",
            "Deaths / Game",
            "Assists",
            "Assists / Game",
            "K/D",
            "HS %",
        )
    }
}

#[derive(Debug)]
pub struct PlayerStats {
    pub combat_score: f32,
    pub kills: u32,
    pub kpg: f32,
    pub deaths: u32,
    pub dpg: f32,
    pub assists: u32,
    pub apg: f32,
    pub kd: f32,
    pub hs: f32,
}