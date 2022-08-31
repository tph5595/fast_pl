use crate::birthdeath::BirthDeath;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct PersistenceMountain {
    position: i32,
    slope_rising: bool,
    birth: Point,
    middle: Point,
    death: Point,
    id: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
enum EventType {
    Death,
    Birth,
    Middle,
    Intersection,
}

#[derive(Debug, Clone, Copy)]
struct Event {
    value: Point,
    event_type: EventType,
    parent_mountain_id: i32,
    parent_mountain2_id: Option<i32>,
}

fn create_mountain(birth: f32, death: f32, index: usize) -> PersistenceMountain {
    let half_dist = (death - birth) / 2.0;
    return PersistenceMountain {
        position: -1,
        slope_rising: true,
        birth: Point { x: birth, y: 0.0 },
        middle: Point {
            x: half_dist + birth,
            y: half_dist,
        },
        death: Point { x: death, y: 0.0 },
        id: index,
    };
}

fn generate_mountains(bd_pairs: Vec<BirthDeath>) -> Vec<PersistenceMountain> {
    return bd_pairs
        .into_iter()
        .filter(|BirthDeath { birth, death }| death.is_finite() && birth.is_finite())
        .enumerate()
        .map(|(i, BirthDeath { birth, death })| create_mountain(birth, death, i))
        .collect::<Vec<_>>();
}

fn generate_initial_events(bd_pairs: Vec<PersistenceMountain>) -> Vec<Event> {
    return Vec::new();
}

pub fn generate(bd_pairs: Vec<BirthDeath>, k: i32) -> Vec<Vec<Point>> {
    let landscapes: Vec<Vec<Point>> = Vec::with_capacity(k as usize);
    let mountains = generate_mountains(bd_pairs);
    let events = BinaryHeap::from(generate_initial_events(mountains));

    return landscapes;
}
