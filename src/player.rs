use hashbrown::HashMap;
use crate::game::GameStats;
use crate::team::Team;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub gp: u32,
    pub gw: u32,
    pub gl: u32,
    pub rp: u32,
    pub rw: u32,
    pub rl: u32,
    pub tr: u32,
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
            rp: 0,
            rw: 0,
            rl: 0,
            tr: 0,
            stats: vec![],
            averaged: None,
            agents: HashMap::new(),
        }
    }

    pub fn finish(&mut self) {
        let sum: GameStats = self.stats.drain(..).sum();
        let rounds = self.rp as f32;
        let games = self.gp as f32;
        let mut sub_games = self.gp as f32 - sum.sub as f32;
        if sub_games <= 0.0 {
            sub_games = 1.0;
        }
        let mut sub_tracker_games = self.gp as f32 - sum.sub_tracker as f32;
        if sub_tracker_games <= 0.0 {
            sub_tracker_games = 1.0;
        }
        self.averaged = Some(PlayerStats {
            combat_score: sum.combat_score as f32 / games,
            kills: sum.kills,
            kpg: sum.kills as f32 / games,
            kpr: sum.kills as f32 / rounds,
            deaths: sum.deaths,
            dpg: sum.deaths as f32 / games,
            dpr: sum.deaths as f32 / rounds,
            assists: sum.assists,
            apg: sum.assists as f32 / games,
            apr: sum.assists as f32 / rounds,
            kd: sum.kd / games,
            adr: sum.adr / self.tr as f32,
            kast: sum.kast as f32 / sub_tracker_games,
            fk:sum.fk,
            fkpg: sum.fk as f32 / games,
            fd: sum.fd,
            fdpg: sum.fd as f32 / sub_tracker_games,
            hs: sum.hs as f32 / sub_tracker_games,
            plants: sum.plants,
            ppg: sum.plants as f32 / sub_games,
            defuses: sum.defuses,
            dfpg: sum.defuses as f32 / sub_games,
            eco: sum.eco as f32 / sub_games,
        });
    }

    pub fn to_csv(&self) -> String {
        let averaged = self.averaged.as_ref().unwrap();
        format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.name,
            self.agents.keys().cloned().collect::<Vec<_>>().join(" "),
            self.gp,
            self.gw,
            self.gl,
            self.rp,
            self.rw,
            self.rl,
            averaged.combat_score,
            averaged.kills,
            averaged.kpg,
            averaged.kpr,
            averaged.deaths,
            averaged.dpg,
            averaged.dpr,
            averaged.assists,
            averaged.apg,
            averaged.apr,
            averaged.kd,
            averaged.adr,
            averaged.kast,
            averaged.fk,
            averaged.fkpg,
            averaged.fd,
            averaged.fdpg,
            averaged.hs,
            averaged.plants,
            averaged.ppg,
            averaged.defuses,
            averaged.dfpg,
            averaged.eco,
        )
    }

    pub fn csv_header() -> String {
        format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            "Name",
            "Agents",
            "Games played",
            "Games won",
            "Games lost",
            "Rounds played",
            "Rounds won",
            "Rounds lost",
            "ACS",
            "Kills",
            "Kills / Game",
            "Kills / Round",
            "Deaths",
            "Deaths / Game",
            "Deaths / Round",
            "Assists",
            "Assists / Game",
            "Assists / Round",
            "K/D",
            "ADR",
            "KAST",
            "First Kills",
            "First Kills / Game",
            "First Deaths",
            "First Deaths / Game",
            "HS %",
            "Plants",
            "Plants / Game",
            "Defuses",
            "Defuses / Game",
            "Eco Score",
        )
    }
}

#[derive(Debug)]
pub struct PlayerStats {
    pub combat_score: f32,
    pub kills: u32,
    pub kpg: f32,
    pub kpr: f32,
    pub deaths: u32,
    pub dpg: f32,
    pub dpr: f32,
    pub assists: u32,
    pub apg: f32,
    pub apr: f32,
    pub kd: f32,
    pub adr: f32,
    pub kast: f32,
    pub fk: u32,
    pub fkpg: f32,
    pub fd: u32,
    pub fdpg: f32,
    pub hs: f32,
    pub plants: u32,
    pub ppg: f32,
    pub defuses: u32,
    pub dfpg: f32,
    pub eco: f32,
}

pub fn parse_duplicates(input: String) -> HashMap<String, String> {
    let mut duplicates = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split(',');
        if let Some(name) = iter.next().map(String::from) {
            let name = name.to_lowercase();
            for alternate in iter {
                duplicates.insert(alternate.to_string().to_lowercase(), name.clone());
            }
        }
    }
    duplicates
}