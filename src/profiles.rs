use crate::*;
use std::{collections::HashMap, str::FromStr};
use evdev_rs::enums::EV_REL;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ABS {
    pub mouse: String,
    pub keyboard: String,
    pub events: HashMap<String, String>
}

#[derive(Clone, Debug)]
pub struct CustomEvent {
    pub trigger: Result<EV_REL, EV_KEY>,
    pub event: Result<EV_ABS, EV_KEY>,
    pub multi: i32,
    pub sign: i32
}

pub fn start_env() -> String {
    dotenvy::dotenv().ok();
    let profiles_path = dotenvy::var("GAMEPAD_PROFILES").expect("Unable to read profiles dir");
    let ev = format!("{}/virtualconf.json", &profiles_path);
    ev
}

pub fn build() -> ((String, String), Vec<CustomEvent>) {
    let ev = start_env();
    build_events(read(ev))
}

pub fn read_file(path: String) -> String {
    std::fs::read_to_string(path).expect("unable to read file")
}

pub fn read(path: String) -> ABS {
    let file = read_file(path);
    serde_json::from_str(&file).expect("unable to parse into json")
}

pub fn build_events(abs: ABS) -> ((String, String), Vec<CustomEvent>) {
    let keyboard = abs.keyboard;
    let mouse = abs.mouse;
    let mut events = Vec::<CustomEvent>::new();
    for (key, value) in abs.events.iter() {
        let mod_esp : Vec<String> = value.split("|").map(|x| x.to_string()).collect();
        let ev_mul = mod_esp[0].parse::<i32>().unwrap();
        let ev_sig = &mod_esp[1];
        let ev = &mod_esp[2];
        events.push(CustomEvent {
            trigger: parse_trigger(&key),
            event: parse_event(ev),
            multi: ev_mul,
            sign: match ev_sig.as_str() {
                "-" => -1,
                _ => 1
            }
        });
    }
    println!("{:?}", events);
    ((mouse, keyboard), events)
}

pub fn parse_trigger(trigger: &str) -> Result<EV_REL, EV_KEY> {
    let rel = EV_REL::from_str(trigger);
    if rel.is_ok() {
        return Ok(rel.unwrap());
    }
    return Err(EV_KEY::from_str(trigger).unwrap());
}

pub fn parse_event(event: &str) -> Result<EV_ABS, EV_KEY> {
    let abs = EV_ABS::from_str(event);
    if abs.is_ok() {
        return Ok(abs.unwrap());
    }
    Err(EV_KEY::from_str(event).unwrap())
}
