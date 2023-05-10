#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use crate::birthdeath::BirthDeath;
use float_ord::FloatOrd;
use geo::{
    line_intersection::line_intersection, line_intersection::LineIntersection, Coord, Line,
};
use std::cmp::min;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointOrd {
    pub x: FloatOrd<f32>,
    pub y: FloatOrd<f32>,
}

impl Ord for PointOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
         self.x.cmp(&other.x)
    }
}

impl PartialOrd for PointOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Above,
    Below,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Intersection,
    Death,
    Middle,
    Birth,
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
        if self.value == other.value{
            return self.event_type.cmp(&other.event_type).reverse();
        }
        self.value.cmp(&other.value).reverse()
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

    PersistenceMountain {
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
    }
}

fn generate_mountains(bd_pairs: Vec<BirthDeath>) -> Vec<PersistenceMountain> {
    bd_pairs
        .into_iter()
        .filter(|BirthDeath { birth, death }| death.is_finite() && birth.is_finite())
        .enumerate()
        .map(|(i, BirthDeath { birth, death })| create_mountain(birth, death, i))
        .collect::<Vec<_>>()
}

fn generate_initial_events(mountains: Vec<PersistenceMountain>) -> Vec<Event> {
    mountains
        .into_iter()
        .flat_map(
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
        .collect()
}

const fn current_segment_start(mountain: PersistenceMountain) -> (f32, f32) {
    if mountain.slope_rising {
        (mountain.birth.x.0, mountain.birth.y.0)
    } else { 
        (mountain.middle.x.0, mountain.middle.y.0)
    }
}

const fn current_segment_end(mountain: PersistenceMountain) -> (f32, f32) {
    if mountain.slope_rising {
        (mountain.middle.x.0, mountain.middle.y.0)
    } else {
        (mountain.death.x.0, mountain.death.y.0)
    }
}

fn create_line_segment(mountain: PersistenceMountain) -> Line<f32> {
    Line {
        start: current_segment_start(mountain).into(),
        end: current_segment_end(mountain).into(),
    }
}

fn intersects_with_neighbor(m1: PersistenceMountain, m2: PersistenceMountain) -> Option<PointOrd> {
    if m1.slope_rising == m2.slope_rising {
        return None;
    }
    let inter = line_intersection(create_line_segment(m1), create_line_segment(m2));
    match inter {
        Some(LineIntersection::SinglePoint {
            intersection: Coord { x, y },
            is_proper: true
        }) => Some(PointOrd {
            x: min(FloatOrd(x), min(m1.death.x, m2.death.x)),
            y: FloatOrd(y),
        }),
        // Ignore all colinnear, not proper and no intersection results these will be resolved on
        // slope change or do not matter
        Some(i) => match i {
            LineIntersection::SinglePoint { intersection: _, is_proper: _ } => None,
            LineIntersection::Collinear { intersection: _ } => None

        },
        None => None
    }
}

fn log_to_landscape(
    mountain: PersistenceMountain,
    value: PointOrd,
    landscapes: &mut [Vec<PointOrd>],
    k: usize,
) {
    let position = mountain.position.expect("Mountain with event is dead");
    if position < k {
        landscapes[position].push(value);
    }
}

fn find_intersection(
    status: &VecDeque<usize>,
    m1: PersistenceMountain,
    mountains: &[PersistenceMountain],
    direction_to_check: Direction,
) -> Option<Event> {
    let position = m1.position.expect("Intersection check for dead mountain");
    // Stop underflow of unsigned number
    if position == 0 && direction_to_check == Direction::Above {
        return None;
    }
    let neighbor_index = match direction_to_check {
        Direction::Below => position + 1,
        Direction::Above => position - 1,
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
    None
}

#[must_use]
pub fn empty_landscape(k: usize) -> Vec<Vec<PointOrd>>{
    let mut landscapes = Vec::with_capacity(k);
    (0..k).for_each(|_| {
        let arr = Vec::new();
        landscapes.push(arr);
    });
    landscapes
}

fn handle_birth(state: &mut State, event: &Event){
    // Add to status structure
    let start_len = state.status.len();
    state.status.push_back(event.parent_mountain_id);
    assert!(start_len + 1 == state.status.len());
    let position = state.status.len() - 1;
    state.mountains[event.parent_mountain_id].position = Some(position);
    // Add to output if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event.value,
        &mut state.landscapes,
        state.k,
        );
    // Check for intersections
    if let Some(new_event) = find_intersection(
        & state.status,
        state.mountains[event.parent_mountain_id],
        &state.mountains,
        Direction::Above,
        ) {
        state.events.events_int.push_back(new_event);
    }

}

fn handle_intersection(state: &mut State, event: &Event){
    let parent_mountain2_id = event
        .parent_mountain2_id
        .expect("Intersection event with no second mountain");
    // Add to ouput if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event.value,
        &mut state.landscapes,
        state.k,
        );
    log_to_landscape(
        state.mountains[parent_mountain2_id], 
        event.value, 
        &mut state.landscapes, 
        state.k
        );
    let (lower, upper) = if state.mountains[event.parent_mountain_id].slope_rising {(
            state.mountains[event.parent_mountain_id],
            state.mountains[parent_mountain2_id])
    } else{(
            state.mountains[parent_mountain2_id],
            state.mountains[event.parent_mountain_id],
            )};
    // Swap
    state.status.swap(
        upper.position.expect("Dead mountain in intersection event"),
        lower.position.expect("Dead mountain in intersection event"),
        );
    assert!(upper.position != lower.position);
    (state.mountains[lower.id].position, state.mountains[upper.id].position) =
        (upper.position, lower.position);
    // Check for intersections
    if let Some(new_event) =
        find_intersection(&state.status, state.mountains[lower.id], &state.mountains, Direction::Above)
        {
            state.events.events_int.push_back(new_event);
        }
    if let Some(new_event) =
        find_intersection(&state.status, state.mountains[upper.id], &state.mountains, Direction::Below)
        {
            state.events.events_int.push_back(new_event);
        }
}


fn handle_death(state: &mut State, event: &Event){
    let pos = state.mountains[event.parent_mountain_id]
        .position
        .expect("Death of dead mountain");
    // Check for floating point mess up on death/intersection Ordering
    let weird_q = &mut VecDeque::new();
    if pos != state.status.len() - 1 {
        while pos < state.status.len() - 1 {
            weird_q.push_back(state.status.pop_back().unwrap());
        }
    }
    // Add to ouput if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event.value,
        &mut state.landscapes,
        state.k,
        );
    // remove and disable
    state.status.pop_back();
    state.mountains[event.parent_mountain_id].position = None;
    while !weird_q.is_empty() {
        let element = weird_q.pop_back().unwrap();
        state.mountains[element].position = Some(state.mountains[element].position.unwrap() - 1);
        log_to_landscape(
            state.mountains[element],
            event.value,
            &mut state.landscapes,
            state.k,
            );
        state.status.push_back(element);
    }
}

fn handle_middle(state: &mut State, event: &Event){
    // Update status structures
    state.mountains[event.parent_mountain_id].slope_rising = false;
    // Add to ouput if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event.value,
        &mut state.landscapes,
        state.k,
        );
    // Check for intersections
    if let Some(new_event) = find_intersection(
        &state.status,
        state.mountains[event.parent_mountain_id],
        &state.mountains,
        Direction::Below,
        ) {
        state.events.events_int.push_back(new_event);
    }
}

#[derive(Debug)]
struct Events{
    events_int: VecDeque<Event>,
    events_base: BinaryHeap<Event>,
}

#[derive(Debug)]
struct State{
    status: VecDeque<usize>,
    mountains: Vec<PersistenceMountain>,
    landscapes: Vec<Vec<PointOrd>>,
    events: Events,
    k: usize,
    debug: bool
}

impl Iterator for State{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let event = if !self.events.events_int.is_empty() {
            Some(self.events.events_int.pop_front().unwrap())
        }
        else if !self.events.events_base.is_empty() {
            Some(self.events.events_base.pop().unwrap())
        }
        else {
            None
        };
        // let int_head = self.events.events_int.pop_front().unwrap();
        // Opposite on purpose due to cmp from bin heap
        // let event = if self.events.events_base.peek().unwrap() < &int_head{
            // int_head
        // }else{
            // self.events.events_int.push_front(int_head); // Add value back in 
            // self.events.events_base.pop().unwrap()
        // };
        if self.debug {
            println!("{event:?}");
            println!("{:?}", self.status);
            println!("================================================================");
        }
        event
    }
}

/// # Panics
///
/// Will panic if invalid state is discovered during generation
#[must_use]
pub fn generate(bd_pairs: Vec<BirthDeath>, k: usize, debug: bool) -> Vec<Vec<PointOrd>> {
    let mountains = generate_mountains(bd_pairs);

    let mut state = State{
        status: VecDeque::new(),
        mountains: mountains.clone(),
        landscapes: empty_landscape(k),
        events: Events {
            events_int: VecDeque::new(),
            events_base: BinaryHeap::from(generate_initial_events(mountains)),
        },
        k,
        debug
    };

    while let Some(event) = state.next(){
        match event.event_type {
            EventType::Birth => {
                handle_birth(&mut state, &event);
            }
            EventType::Middle => {
                handle_middle(&mut state, &event);
            }
            EventType::Death => {
                handle_death(&mut state, &event);
            }
            EventType::Intersection => {
                handle_intersection(&mut state, &event);
            }
        }
    }

    state.landscapes
}
