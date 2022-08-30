use crate::birthdeath::BirthDeath;
use std::collections::BinaryHeap;

#[derive(Debug)]
enum EventType {
    Birth,
    Death,
}

#[derive(Debug)]
struct Node {
    birth_event: Event,
    death_event: Event,
    id: usize,
    alive: bool,
    in_top_k: bool,
    is_dead: bool,
}

fn get_value(n: &Node) -> &Event {
    match n.alive {
        true => &n.death_event,
        false => &n.birth_event,
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let me = get_value(self).value;
        let oth = get_value(other).value;
        if me < oth {
            return std::cmp::Ordering::Less;
        }
        if oth < me {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
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

#[derive(Debug)]
struct Event {
    event_type: EventType,
    value: f32,
}

fn create_event(birth: f32, death: f32, i: usize) -> Node {
    return Node {
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
    };
}

fn generate_events(bd_pairs: Vec<BirthDeath>) -> Vec<Node> {
    return bd_pairs
        .into_iter()
        .filter(|BirthDeath { birth, death }| death.is_finite() && birth.is_finite())
        .enumerate()
        .map(|(i, BirthDeath { birth, death })| create_event(birth, death, i))
        .collect::<Vec<_>>();
}

pub fn barcode_filter(bd_pairs: Vec<BirthDeath>, k: i32) -> Vec<BirthDeath> {
    let events = generate_events(bd_pairs);
    let mut event_stack = BinaryHeap::from(events);

    return Vec::new();
}
