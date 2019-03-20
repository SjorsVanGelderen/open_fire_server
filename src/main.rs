// Copyright 2019, Sjors van Gelderen

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

mod game_state;

use crate::game_state::{
    GameInitInfo,
    GameState,
};

use rocket::{
    http::RawStr,
    State,
};

use rocket_contrib::json::Json;

use std::fmt;

#[get("/")]
fn index(s: State<GameState>) -> String {
    format!("The server is active :). Game started? -> {}", *s.started.read().unwrap())
}

#[get("/start")]
fn start(s: State<GameState>) -> &'static str {
    let mut started = s.started.write().unwrap();
    *started = true;
    
    "OK"
}

// #[post("/start", format = "json", data = "<game_init_info>")]
// fn start(s: State<GameState>, game_init_info: Json<GameInitInfo>) -> &'static str {
//     // TODO: Use game_init_info here to start the game
//     // TODO: Check if the game has already started

//     let gii = game_init_info.into_inner();

//     if *s.started.read().unwrap() {
//         "FAIL"
//     }
//     else {
//         let gs = GameState::start(gii);

//         let mut started = s.started.write().unwrap();
//         *started = false;

//         "OK"
//     }
// }

#[get("/stop")]
fn stop() -> &'static str {
    // TODO: Stop the game here

    "OK"
}

#[get("/move_player/<player>")]
fn move_player(player: &RawStr) -> &'static str {
    "OK"
}

// TODO: Push latest game data to connected clients (using reqwest?)

fn main() {
    rocket::ignite()
    .manage(GameState::new())
    .mount("/", routes![index, start, move_player])
    .launch();
}