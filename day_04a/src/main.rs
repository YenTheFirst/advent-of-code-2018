
extern crate regex;
use regex::Regex;
use std::io::BufRead;
use std::collections::HashMap;

enum State {
    NoGuard,
    WokeGuard { guard: u32 },
    SleepGuard { guard: u32, minute: u8 }
}

enum Event {
    ShiftChange { guard: u32 },
    Wake { minute: u8 },
    Sleep { minute: u8 },
}

fn main() {

    let stdin = std::io::stdin();
    let re = Regex::new(r"\[(?P<date>\d{4}-\d{2}-\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<event>Guard #(?P<guard_id>\d+) begins shift|falls asleep|wakes up)").unwrap();
    let mut lines : Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    lines.sort();

    let mut records : HashMap<u32, [u32; 60]> = HashMap::new();
    let mut state = State::NoGuard;

    for l in lines.iter() {
        let caps = re.captures(l).expect("couldn't match");
        let minute = caps["minute"].parse().expect("not a minute");
        let event = match &caps["event"] {
            "falls asleep" => Event::Sleep { minute },
            "wakes up" => Event::Wake { minute },
            _ => Event::ShiftChange { guard: caps["guard_id"].parse().unwrap() }
        };
        state = match (state, event) {
            (_, Event::ShiftChange{guard}) => State::WokeGuard{guard},
            (State::SleepGuard{guard,minute: sleep_minute}, Event::Wake{minute: wake_minute}) => {
                let record = records.entry(guard).or_insert([0; 60]);
                for t in sleep_minute..wake_minute {
                    (*record)[t as usize] += 1
                };
                State::WokeGuard{guard}
            }
            (State::WokeGuard{guard}, Event::Sleep{minute}) => State::SleepGuard{guard, minute},
            _ => panic!("bad combo")
        }
    };

    let (guard, sleepy) = records.iter().max_by_key(|&(&_, &sleepytime)| sleepytime.iter().sum::<u32>()).expect("empty records?");
    let (most_sleep, _) = sleepy.iter().enumerate().max_by_key(|&(_,v)| v).unwrap();

    println!("{}", guard * most_sleep as u32);
}

