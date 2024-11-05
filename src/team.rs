use hashbrown::HashMap;
use crate::game::GameData;
use crate::player::Player;

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub players: Vec<String>,
}

pub fn parse_teams(input: String) -> HashMap<String, Team> {
    let mut teams = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split(',');
        let _ = iter.next();
        let name = iter.next().unwrap().to_string();
        if name.trim().is_empty() { continue; }
        let players = iter.map(ToString::to_string).collect();
        teams.insert(name.to_lowercase(), Team { players, name });
    }
    teams
}

pub fn apply(players: &mut HashMap<String, Player>, teams: &HashMap<String, Team>, agents: &mut HashMap<String, (u32, u32)>, mut data: GameData) {
    let a = teams.get(&data.a.to_lowercase()).expect(&data.a);
    let b = teams.get(&data.b.to_lowercase()).expect(&data.b);

    fn apply_team(players: &mut HashMap<String, Player>, team: &[String], agents: &mut HashMap<String, (u32, u32)>, win: u32, loss: u32, data: &mut GameData) {
        for player in team {
            if !data.data.contains_key(&player.to_lowercase()) { continue; }
            let stats = data.data.remove(&player.to_lowercase()).unwrap();
            let agent = data.agents.remove(&player.to_lowercase()).unwrap();
            let p = players.get_mut(&player.to_lowercase()).expect(player);
            *p.agents.entry(agent.clone()).or_insert(0) += 1;
            p.gp += 1;
            let agent = agents.entry(agent).or_insert((0, 0));
            if win > loss {
                p.gw += 1;
                agent.0 += 1;
            } else {
                p.gl += 1;
                agent.1 += 1;
            }
            p.rw += win;
            p.rl += loss;
            p.rp += win + loss;
            p.stats.push(stats);
        }
    }

    apply_team(players, &a.players, agents, data.ra, data.rb, &mut data);
    apply_team(players, &b.players, agents, data.rb, data.ra, &mut data);
}

pub fn write_agents(agents: HashMap<String, (u32, u32)>, games: u32) -> String {
    let mut csv = "AGENT,PICKED,PR,WON,LOST,WR\n".to_string();

    for (agent, (win, loss)) in agents {
        csv.push_str(&format!("{},{},{},{},{},{}\n",
            agent,
            win + loss,
            (win + loss) as f32 / (2 * games) as f32,
            win,
            loss,
            win as f32 / (win + loss) as f32
        ));
    }

    csv
}