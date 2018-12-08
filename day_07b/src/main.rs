use std::io::{stdin,BufRead};
use std::collections::{HashSet,BinaryHeap};
use std::cmp::Ordering;

const WORKER_COUNT : usize = 5;
const JOB_BASE : u32 = 60;

#[derive(Eq, PartialEq)]
struct Job {
    complete_at: u32,
    step: char,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.complete_at.cmp(&other.complete_at).reverse()
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = stdin();
    let mut deps : Vec<(char,char)> = stdin.lock().lines().map(|l| {
        let line = l.unwrap();
        let mut words = line.split_whitespace();
        (
            words.nth(1).unwrap().chars().next().unwrap(),
            words.nth(5).unwrap().chars().next().unwrap()
        )
    }).collect();
    
    let (l,r): (HashSet<char>, HashSet<char>) = deps.iter().cloned().unzip();
    let mut uniq : HashSet<char> = l.union(&r).cloned().collect();

    let mut in_flight : BinaryHeap<Job> = BinaryHeap::new();
    let mut time = 0;

    while !uniq.is_empty() {
        while in_flight.len() < WORKER_COUNT {
            let (_, blocked): (HashSet<char>, HashSet<char>) = deps.iter().cloned().unzip();
            let take = uniq.difference(&blocked).cloned().min();
            match take {
                Some(j) => {
                    let complete_at = time + (j as u32) - ('A' as u32) + 1 + JOB_BASE;
                    uniq.remove(&j);
                    in_flight.push(Job{complete_at: complete_at, step: j});
                }
                None => {
                    break;
                }
            }
        }

        let job = in_flight.pop().unwrap();
        time = job.complete_at;
        println!("finished {} at {:?}", job.step, job.complete_at);
        deps.retain(|(bl,_)| *bl != job.step);
    }
    while !in_flight.is_empty() {
        time = in_flight.pop().unwrap().complete_at;
    }
    println!("{}", time);
}