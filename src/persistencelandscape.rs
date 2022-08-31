use crate::birthdeath::BirthDeath;
use float_ord::FloatOrd;
use geo::{
    line_intersection::line_intersection, line_intersection::LineIntersection, Coordinate, Line,
};
use std::collections::{BinaryHeap, VecDeque};

#[derive(Debug, Clone, Copy)]
struct PersistenceMountain {
    position: Option<usize>,
    slope_rising: bool,
    birth: PointOrd,
    middle: PointOrd,
    death: PointOrd,
    id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointOrd {
    pub x: FloatOrd<f32>,
    pub y: FloatOrd<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Death,
    Birth,
    Middle,
    Intersection,
}

#[derive(Debug, Clone, Copy)]
struct Event {
    value: PointOrd,
    event_type: EventType,
    parent_mountain_id: usize,
    parent_mountain2_id: Option<usize>,
}

// NOTE: This is opposite on purpose to flip to built in BinaryHeap
impl Ord for Event {
    // Compare points then event_type
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value < other.value {
            return std::cmp::Ordering::Greater;
        } else if other.value < self.value {
            return std::cmp::Ordering::Less;
        } else if self.event_type < other.event_type {
            return std::cmp::Ordering::Greater;
        } else if other.event_type < self.event_type {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.parent_mountain_id == other.parent_mountain_id
            && self.parent_mountain2_id == other.parent_mountain2_id
    }
}

impl Eq for Event {}

fn create_mountain(birth: f32, death: f32, index: usize) -> PersistenceMountain {
    let half_dist = (death - birth) / 2.0;
    return PersistenceMountain {
        position: None,
        slope_rising: true,
        birth: PointOrd {
            x: FloatOrd(birth),
            y: FloatOrd(0.0),
        },
        middle: PointOrd {
            x: FloatOrd(half_dist + birth),
            y: FloatOrd(half_dist),
        },
        death: PointOrd {
            x: FloatOrd(death),
            y: FloatOrd(0.0),
        },
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

fn generate_initial_events(mountains: Vec<PersistenceMountain>) -> Vec<Event> {
    return mountains
        .into_iter()
        .map(
            |PersistenceMountain {
                 birth,
                 middle,
                 death,
                 id,
                 ..
             }| {
                vec![
                    Event {
                        value: birth,
                        event_type: EventType::Birth,
                        parent_mountain_id: id,
                        parent_mountain2_id: None,
                    },
                    Event {
                        value: middle,
                        event_type: EventType::Middle,
                        parent_mountain_id: id,
                        parent_mountain2_id: None,
                    },
                    Event {
                        value: death,
                        event_type: EventType::Death,
                        parent_mountain_id: id,
                        parent_mountain2_id: None,
                    },
                ]
            },
        )
        .flatten()
        .collect();
}

fn current_segment_start(mountain: PersistenceMountain) -> (f32, f32) {
    return match mountain.slope_rising {
        true => (mountain.birth.x.0, mountain.birth.y.0),
        false => (mountain.middle.x.0, mountain.middle.y.0),
    };
}

fn current_segment_end(mountain: PersistenceMountain) -> (f32, f32) {
    return match mountain.slope_rising {
        true => (mountain.middle.x.0, mountain.middle.y.0),
        false => (mountain.death.x.0, mountain.death.y.0),
    };
}

fn create_line_segment(mountain: PersistenceMountain) -> Line<f32> {
    return Line {
        start: current_segment_start(mountain).into(),
        end: current_segment_end(mountain).into(),
    };
}

fn intersects_with_neighbor(m1: PersistenceMountain, m2: PersistenceMountain) -> Option<PointOrd> {
    if m1.slope_rising == m2.slope_rising {
        return None;
    }
    return match line_intersection(create_line_segment(m1), create_line_segment(m2)) {
        Some(LineIntersection::SinglePoint {
            intersection: Coordinate { x, y },
            is_proper: true,
        }) => Some(PointOrd {
            x: FloatOrd(x),
            y: FloatOrd(y),
        }),
        // Ignore all colinnear, not proper and no intersection results these will be resolved on
        // slope change or do not matter
        _ => None,
    };
}

fn log_to_landscape(
    mountain: PersistenceMountain,
    event: Event,
    landscapes: &mut Vec<Vec<PointOrd>>,
    k: usize,
) {
    let position = mountain.position.expect("Mountain with event is dead");
    if position < k {
        landscapes[position].push(event.value);
    }
}

fn handle_intersection(
    status: &mut VecDeque<usize>,
    m1: PersistenceMountain,
    mountains: &mut Vec<PersistenceMountain>,
    offset: i8,
) -> Option<Event> {
    let position = m1.position.expect("Mountain with event is dead");
    // Stop underflow of unsigned number
    if position == 0 {
        return None;
    }
    let neighbor_index = match offset {
        1 => position + 1,
        -1 => position - 1,
        _ => unreachable!("Can only look at neighbors in status"),
    };

    if let Some(neighbor) = status.get(neighbor_index) {
        if let Some(intersection) = intersects_with_neighbor(m1, mountains[*neighbor]) {
            return Some(Event {
                value: intersection,
                event_type: EventType::Intersection,
                parent_mountain_id: m1.id,
                parent_mountain2_id: Some(*neighbor),
            });
        }
    }
    return None;
}

pub fn generate(bd_pairs: Vec<BirthDeath>, k: usize) -> Vec<Vec<PointOrd>> {
    let landscapes = &mut Vec::with_capacity(k as usize);
    (0..k).for_each(|_| {
        let arr = Vec::new();
        landscapes.push(arr);
    });
    let mountains = &mut generate_mountains(bd_pairs);
    let events = &mut BinaryHeap::from(generate_initial_events(mountains.to_vec()));
    let status = &mut VecDeque::new();

    while let Some(event) = events.pop() {
        match event.event_type {
            EventType::Birth => {
                // Add to status structure
                status.push_back(event.parent_mountain_id);
                let position = status.len() - 1;
                mountains[event.parent_mountain_id].position = Some(position);
                // Add to output if needed
                log_to_landscape(mountains[event.parent_mountain_id], event, landscapes, k);
                // Check for intersections
                if let Some(new_event) =
                    handle_intersection(status, mountains[event.parent_mountain_id], mountains, -1)
                {
                    events.push(new_event);
                }
            }
            EventType::Middle => {
                // Update status structures
                mountains[event.parent_mountain_id].slope_rising = false;
                // Add to ouput if needed
                log_to_landscape(mountains[event.parent_mountain_id], event, landscapes, k);
                // Check for intersections
                if let Some(new_event) =
                    handle_intersection(status, mountains[event.parent_mountain_id], mountains, 1)
                {
                    events.push(new_event);
                }
            }
            EventType::Death => {
                // Add to ouput if needed
                log_to_landscape(mountains[event.parent_mountain_id], event, landscapes, k);
                // remove and disable
                status.pop_back();
                mountains[event.parent_mountain_id].position = None;
            }
            EventType::Intersection => {
                // Add to ouput if needed
                log_to_landscape(mountains[event.parent_mountain_id], event, landscapes, k);
                log_to_landscape(
                    mountains[event
                        .parent_mountain2_id
                        .expect("Intersection event with no second mountain")],
                    event,
                    landscapes,
                    k,
                );
                let (mut lower, mut upper) = match mountains[event.parent_mountain_id].slope_rising
                {
                    true => (
                        mountains[event.parent_mountain_id],
                        mountains[event.parent_mountain2_id.unwrap()],
                    ),
                    false => (
                        mountains[event.parent_mountain2_id.unwrap()],
                        mountains[event.parent_mountain_id],
                    ),
                };
                // Swap
                status.swap(
                    lower.position.expect("Dead mountain in intersection event"),
                    upper.position.expect("Dead mountain in intersection event"),
                );
                (lower.position, upper.position) = (upper.position, lower.position);
                // Check for intersections
                if let Some(new_event) = handle_intersection(status, lower, mountains, -1) {
                    events.push(new_event);
                }
                if let Some(new_event) = handle_intersection(status, upper, mountains, 1) {
                    events.push(new_event);
                }
            }
        }
    }

    return landscapes.to_vec();
}
