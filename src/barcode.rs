#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use crate::birthdeath::BirthDeath;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Debug, Clone)]
enum EventType {
    Birth,
    Death,
}

#[derive(Debug, Clone)]
struct Node {
    birth_event: Event,
    death_event: Event,
    id: usize,
    alive: bool,
    in_top_k: bool,
    is_dead: bool,
}

const fn get_value(n: &Node) -> &Event {
    if n.alive { 
        &n.death_event 
    } else { 
        &n.birth_event 
    }
}

// NOTE: This is opposite on purpose to flip to built in BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let me = get_value(self).value;
        let oth = get_value(other).value;
        if me < oth {
            return std::cmp::Ordering::Greater;
        }
        if oth < me {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

#[derive(Debug, Clone)]
struct Event {
    event_type: EventType,
    value: f32,
}

const fn create_event(birth: f32, death: f32, i: usize) -> Node {
    Node {
        birth_event: Event {
            event_type: EventType::Birth,
            value: birth,
        },
        death_event: Event {
            event_type: EventType::Death,
            value: death,
        },
        id: i,
        alive: false,
        in_top_k: false,
        is_dead: false,
    }
}

fn generate_events(bd_pairs: Vec<BirthDeath>) -> Vec<Node> {
    bd_pairs
        .into_iter()
        .filter(|BirthDeath { birth, death }| death.is_finite() && birth.is_finite())
        .enumerate()
        .map(|(i, BirthDeath { birth, death })| create_event(birth, death, i))
        .collect::<Vec<_>>()
}

const fn node_to_birthdeath(n: &Node) -> BirthDeath {
    BirthDeath {
        birth: n.birth_event.value,
        death: n.death_event.value,
    }
}

/// # Panics
///
/// Will panic is structure gets corrupted
#[must_use]
pub fn filter(bd_pairs: Vec<BirthDeath>, k: usize) -> Vec<BirthDeath> {
    let mut nodes = generate_events(bd_pairs);
    let mut event_stack = BinaryHeap::from(nodes.clone());
    let sweep_status: &mut VecDeque<usize> = &mut VecDeque::new();
    let mut filtered_output: Vec<BirthDeath> = Vec::new();
    let mut in_top = 0;
    let mut waiting = 0;

    while let Some(mut event) = event_stack.pop(){
        match get_value(&event).event_type {
            EventType::Birth => {
                // Check if in top-k and handle
                if in_top < k {
                    in_top += 1;
                    filtered_output.push(node_to_birthdeath(&event));
                    // Mark so we know when it dies
                    nodes[event.id].in_top_k = true;
                }
                // Normal processing of value
                nodes[event.id].alive = true;
                event.alive = true;
                sweep_status.push_back(event.id);
                // Add the event back into to register its death_event
                event_stack.push(event);
            }
            EventType::Death => {
                // mark as dead
                nodes[event.id].is_dead = true;
                // if was in top-k promote next canidate
                // lazy add
                if nodes[event.id].in_top_k {
                    waiting += 1;
                }
                // check if next element should be in top k
                if waiting > 0 {
                    let mut front_index = sweep_status.get(0);
                    while front_index.is_some() && nodes[*front_index.unwrap()].is_dead {
                        sweep_status.pop_front();
                        front_index = sweep_status.get(0);
                    }
                    // If queue has no canidate events move on
                    if front_index.is_none() {
                        continue;
                    }
                    // Found a canidate, add it to top k
                    let front = *front_index.unwrap();
                    if !nodes[front].in_top_k {
                        nodes[front].in_top_k = true;
                        filtered_output.push(node_to_birthdeath(&nodes[front]));
                    }
                }
            }
        }
    }
    filtered_output
}
