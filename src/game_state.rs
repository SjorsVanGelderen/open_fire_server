// Copyright 2019, Sjors van Gelderen

// use cgmath::Vector3;

use im::{
    hashmap::HashMap,
};

use std::sync::RwLock;

// #[derive(Clone)]
pub struct GameState {
    pub started: RwLock<bool>,
    pub flags: HashMap<String, Flag>,
    pub players: HashMap<String, Player>,
    pub projectiles: HashMap<String, Vec<Projectile>>,
}

impl GameState {
    // TODO: See if this can be replaced with a parent Option<GameState>
    pub fn new() -> Self {
        let started = RwLock::new(false);
        let flags = HashMap::new();
        let players = HashMap::new();
        let projectiles = HashMap::new();

        Self {
            started,
            flags,
            players,
            projectiles,
        }
    }

    pub fn start(info: GameInitInfo) -> Self {
        let started = RwLock::new(true);

        let mut flags = HashMap::new();
        for f in info.flags {
            flags.insert(f.owner, Flag::new(f.transform));
        }

        let mut players = HashMap::new();
        for p in info.players {
            players.insert(p.name, Player::new(p.transform));
        }

        let projectiles = HashMap::new();

        Self {
            started,
            flags,
            players,
            projectiles,
        }
    }

    // pub fn add_player(self, name: &'static str, player: Player) -> Self {
    //     let players = self.players.update(name, player);

    //     Self {
    //         players,
    //         ..self
    //     }
    // }
}

#[derive(Deserialize, Serialize)]
pub struct GameInitInfo {
    pub flags: Vec<FlagInitInfo>,
    pub players: Vec<PlayerInitInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct FlagInitInfo {
    pub owner: String,
    pub transform: Transform,
}

#[derive(Deserialize, Serialize)]
pub struct PlayerInitInfo {
    pub name: String,
    pub transform: Transform,
}

#[derive(Clone)]
pub struct Player {
    pub transform: Transform,
}

impl Player {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
        }
    }
}

#[derive(Clone)]
pub struct Projectile {
    pub transform: Transform,
}

impl Projectile {
    pub fn new(transform: Transform) {

    }
}

#[derive(Clone)]
pub struct Flag {
    pub captor: Option<&'static str>,
    pub transform: Transform,
}

impl Flag {
    pub fn new(transform: Transform) -> Self {
        Self {
            captor: None,
            transform,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: [f32; 3], rotation: f32) -> Self {
        Self {
            position,
            rotation,
        }
    }
}