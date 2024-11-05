use hashbrown::HashMap;

pub struct Agent {
    pub name: String,
    pub played: u32,
    pub won: u32,
    pub lost: u32,
    pub nmp: u32,
    pub nmw: u32,
    pub nml: u32,
}

impl Agent {
    pub fn new(agent: String) -> Self {
        Agent {
            name: agent.to_string(),
            played: 0,
            won: 0,
            lost: 0,
            nmp: 0,
            nmw: 0,
            nml: 0,
        }
    }
}

pub fn write_agents(agents: HashMap<String, Agent>, games: u32) -> String {
    let mut csv = "AGENT,PICKED,PR,NMPR,WON,LOST,WR,NMWR\n".to_string();

    for (_, agent) in agents {
        csv.push_str(&format!("{},{},{},{},{},{},{},{}\n",
            agent.name,
            agent.played,
            agent.played as f32 / (2 * games) as f32,
            agent.nmp as f32 / (2 * games) as f32,
            agent.won,
            agent.lost,
            agent.won as f32 / agent.played as f32,
            agent.nmw as f32 / agent.nmp as f32,
        ));
    }

    csv
}