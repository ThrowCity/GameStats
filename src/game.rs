use std::default::Default;
use std::iter::Sum;
use std::ops::Add;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct GameStats {
    pub combat_score: u32,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub kd: f32,
    pub adr: f32,
    pub kast: u32,
    pub fk: u32,
    pub fd: u32,
    pub hs: u32,
    pub plants: u32,
    pub defuses: u32,
    pub eco: u32,
    pub sub: u32,
}

impl Add for GameStats {
    type Output = GameStats;
    fn add(mut self, other: GameStats) -> GameStats {
        self.combat_score += other.combat_score;
        self.kills += other.kills;
        self.deaths += other.deaths;
        self.assists += other.assists;
        self.kd += other.kd;
        self.adr += other.adr;
        self.kast += other.kast;
        self.fk += other.fk;
        self.fd += other.fd;
        self.hs += other.hs;
        self.plants += other.plants;
        self.defuses += other.defuses;
        self.eco += other.eco;
        self.sub += other.sub;

        self
    }
}

impl Sum for GameStats {
    fn sum<I: Iterator<Item = GameStats>>(iter: I) -> GameStats {
        iter.fold(GameStats::default(), |a, b| a + b)
    }
}

#[derive(Debug)]
pub struct GameData {
    pub a: String,
    pub b: String,
    pub ra: u32,
    pub rb: u32,
    pub data: HashMap<String, GameStats>,
    pub agents: HashMap<String, String>,
}

pub fn parse_game(name: &str, input: String) -> GameData {
    let mut data = HashMap::new();
    let mut agents = HashMap::new();
    let lines = input.lines().collect::<Vec<&str>>();
    if lines.len() < 13 { panic!("Invalid format for input '{name}'"); }
    for i in 1..11 {
        let mut iter = lines[i].split(',');
        let agent = iter.next().expect(name).to_string();
        if agent.is_empty() { continue; }
        let player = iter.next().expect(name).to_lowercase();
        let combat_score = iter.next().expect(name).parse::<u32>().expect(name);
        let kills = iter.next().expect(name).parse::<u32>().expect(name);
        let deaths = iter.next().expect(name).parse::<u32>().expect(name);
        let assists = iter.next().expect(name).parse::<u32>().expect(name);
        let kd = iter.next().expect(name).parse::<f32>().expect(name);
        let adr = iter.next().expect(name).parse::<f32>().expect(name);
        let kast = iter.next().expect(name).replace('%', "").parse::<f32>().expect(name) * 100.0;
        let fk = iter.next().expect(name).parse::<u32>().expect(name);
        let fd = iter.next().expect(name).parse::<u32>().expect(name);
        let _ = iter.next();
        let hs = iter.next().expect(name).replace('%', "").parse::<f32>().expect(name) * 100.0;
        let plants = iter.next().expect(name).parse::<u32>().unwrap_or(0);
        let defuses = iter.next().expect(name).parse::<u32>().unwrap_or(0);
        let eco = iter.next().expect(name).parse::<u32>().unwrap_or(0);
        let sub = if plants == 0 && defuses == 0 && eco == 0 { 1 } else { 0 };
        data.insert(player.clone(), GameStats {
            combat_score,
            kills,
            deaths,
            assists,
            kd,
            adr,
            kast: kast as u32,
            fk,
            fd,
            hs: hs as u32,
            plants,
            defuses,
            eco,
            sub,
        });
        agents.insert(player, agent);
    }
    let mut iter = lines[12].split(',');
    let a = iter.next().expect(name).to_string();
    let b = iter.next().expect(name).to_string();
    let _ = iter.next();
    let ra = iter.next().expect(name).parse::<u32>().expect(name);
    let _ = iter.next();
    let rb = iter.next().expect(name).parse::<u32>().expect(name);
    GameData {
        a,
        b,
        ra,
        rb,
        data,
        agents,
    }
}