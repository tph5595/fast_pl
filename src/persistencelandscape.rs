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

#[derive(Debug)]
struct PersistenceMountain {
    position: Option<usize>,
    slope_rising: bool,
    birth: PointOrd,
    middle: PointOrd,
    death: PointOrd,
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, PartialEq)]
enum Direction {
    Above,
    Below,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Death,
    Up,
    Down,
    Intersection
}

#[derive(Debug)]
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
        if self.value != other.value{
            return self.value.cmp(&other.value).reverse()
        }
        // if self.event_type != other.event_type{
        self.event_type.cmp(&other.event_type).reverse()
        // }
        // if self.parent_mountain_id != other.parent_mountain_id {
        //     return self.parent_mountain_id.cmp(&other.parent_mountain_id).reverse();
        // }
        // self.parent_mountain2_id.cmp(&other.parent_mountain2_id).reverse()
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
        .collect::<Vec<PersistenceMountain>>()
}

fn generate_initial_events(mountains: &Vec<&mut PersistenceMountain>) -> Vec<Event> {
    mountains
        .iter()
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
                        value: birth.clone(),
                        event_type: EventType::Up,
                        parent_mountain_id: *id,
                        parent_mountain2_id: None,
                    },
                    Event {
                        value: middle.clone(),
                        event_type: EventType::Down,
                        parent_mountain_id: *id,
                        parent_mountain2_id: None,
                    },
                    Event {
                        value: death.clone(),
                        event_type: EventType::Death,
                        parent_mountain_id: *id,
                        parent_mountain2_id: None,
                    },
                ]
            },
        )
        .collect()
}

const fn current_segment_start(mountain: &PersistenceMountain) -> (f32, f32) {
    if mountain.slope_rising {
        (mountain.birth.x.0, mountain.birth.y.0)
    } else { 
        (mountain.middle.x.0, mountain.middle.y.0)
    }
}

const fn current_segment_end(mountain: &PersistenceMountain) -> (f32, f32) {
    if mountain.slope_rising {
        (mountain.middle.x.0, mountain.middle.y.0)
    } else {
        (mountain.death.x.0, mountain.death.y.0)
    }
}

fn create_line_segment(mountain: &PersistenceMountain) -> Line<f32> {
    Line {
        start: current_segment_start(mountain).into(),
        end: current_segment_end(mountain).into(),
    }
}

fn intersects_with_neighbor(m1: &PersistenceMountain, m2: &PersistenceMountain) -> Option<PointOrd> {
    if m1.slope_rising == m2.slope_rising {
        return None;
    }
    let inter = line_intersection(create_line_segment(m1), create_line_segment(m2));
    match inter {
        Some(LineIntersection::SinglePoint {
            intersection: Coord { x, y },
            ..
        }) => Some(PointOrd {
            x: min(FloatOrd(x), min(m1.death.x, m2.death.x)),
            y: FloatOrd(y),
        }),
        // Ignore all colinnear, not proper and no intersection results these will be resolved on
        // slope change or do not matter
        Some(i) => match i {
            LineIntersection::SinglePoint { intersection: _, is_proper: _ } |
                LineIntersection::Collinear { intersection: _ } => None

        },
        _ => None
    }
}

fn float_point_check(p1: &PointOrd, p2: &PointOrd)-> bool{
    (p1.x.0 - p2.x.0).abs() < f32::EPSILON && 
        (p1.y.0 - p2.y.0).abs() < f32::EPSILON 
}

fn log_checks(
    _mountain: &PersistenceMountain,
    event: &Event,
    landscapes: &[Vec<PointOrd>],
    _k: usize,
    position: usize
    ){
        // Don't log points twice This is fine to prevent ordering problems if start and end points
        // are the same, avoids perfect ordering
        // if ! landscapes[*position].is_empty(){
        //     assert_ne!(*landscapes[*position].last().unwrap(), event.value);
        // }
        // Ensure points are increasing x (except if points are exactly the same)
        if ! landscapes[position].is_empty(){
            if float_point_check(landscapes[position].last().unwrap(), &event.value) {
                // Ignore, this is fine. They are the same
            }
            else{
                assert!(landscapes[position].last().unwrap().x.0 < event.value.x.0,
                "Last x in landscape {} is ({},{}) but new point to be added has an x of ({},{})", 
                position,
                landscapes[position].last().unwrap().x.0, 
                landscapes[position].last().unwrap().y.0, 
                event.value.x.0,
                event.value.y.0
                ); 
            }
        }
        // Ensure birth/death is in bottom most landscape (exception if the nearest is a tie, they
        // are just dieing out of order and the other must die right after)
        let below = position + 1;
        if event.value.y.0 == 0.0 &&
            below < landscapes.len() && 
            ! landscapes[below].is_empty(){
                if float_point_check(landscapes[below].last().unwrap(), landscapes[position].last().unwrap()){
                    // This is fine, ignore. See above comment
                }
                else{
                    // println!("{:?}", landscapes[below].last().unwrap());
                    // println!("{:?}", landscapes[position].last().unwrap());
                    // println!("{:?}", mountain);
                    assert!(landscapes[below].last().unwrap().y.0 == 0.0,
                        "Attempting to add a birth/death ({},{}) to higher landscape {} when {} is non zero ({},{})", 
                        event.value.x.0,
                        event.value.y.0,
                        position,
                        below,
                        landscapes[below].last().unwrap().x.0,
                        landscapes[below].last().unwrap().y.0,
                    ); 
                }
        }
}

fn log_to_landscape(
    mountain: &PersistenceMountain,
    event: &Event,
    landscapes: &mut [Vec<(f32,f32)>],
    k: usize,
) {
    let position = mountain.position.expect("Mountain with event is dead");
    if position < k {
        // log_checks(mountain, &event, landscapes, k, position);
        landscapes[position].push((event.value.x.0, event.value.y.0));
    }

}

fn find_intersection(
    status: &VecDeque<usize>,
    parent_mountain_id: usize,
    mountains: &[&mut PersistenceMountain],
    direction_to_check: &Direction,
) -> Option<Event> {
    let position = mountains[parent_mountain_id].position.expect("Intersection check for dead mountain");
    // Stop underflow of unsigned number
    if position == 0 && *direction_to_check == Direction::Above {
        return None;
    }
    let neighbor_index = match direction_to_check {
        Direction::Below => position + 1,
        Direction::Above => position - 1,
    };

    if let Some(neighbor) = status.get(neighbor_index) {
        if let Some(intersection) = intersects_with_neighbor(mountains[parent_mountain_id], mountains[*neighbor]) {
           return Some( Event {
                value: intersection,
                event_type: EventType::Intersection,
                parent_mountain_id,
                parent_mountain2_id: Some(*neighbor),
            });
            // println!("{intersection:?}");
            // return Some(intersection);
        }
    }
    None
}

#[must_use]
pub fn empty_landscape(k: usize) -> Vec<Vec<(f32,f32)>>{
    let mut landscapes = Vec::with_capacity(k);
    (0..k).for_each(|_| {
        let arr = Vec::new();
        landscapes.push(arr);
    });
    landscapes
}

fn handle_up(state: &mut State, event: &Event){
    // Add to status structure
    let start_len = state.status.len();
    state.status.push_back(event.parent_mountain_id);
    assert!(start_len + 1 == state.status.len());
    let position = state.status.len() - 1;
    state.mountains[event.parent_mountain_id].position = Some(position);

    // Add to output if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event,
        &mut state.landscapes,
        state.k,
        );
    // Check and handle all intersections
    let new_event = find_intersection(
        &state.status,
        event.parent_mountain_id,
        state.mountains,
        &Direction::Above,
        );
    if let Some(intersection) = new_event{
        handle_intersection(state, intersection);
    }
}

fn handle_intersection(state: &mut State, event: Event){
    state.weird_q.push_back(event);
    while ! state.weird_q.is_empty(){
        let event = state.weird_q.pop_front().unwrap();
        let parent_mountain2_id = event
            .parent_mountain2_id
            .expect("Intersection event with no second mountain");

        // Add to ouput if needed
        log_to_landscape(
            state.mountains[event.parent_mountain_id],
            &event,
            &mut state.landscapes,
            state.k,
        );
        log_to_landscape(
            state.mountains[parent_mountain2_id], 
            &event, 
            &mut state.landscapes, 
            state.k
        );
        let (upper_id, lower_id) = 
        if state.mountains[event.parent_mountain_id].slope_rising {
            // let upper_id = 
            (parent_mountain2_id,
            // let lower_id = 
                event.parent_mountain_id)
        } else{
            // let upper_id = 
            (event.parent_mountain_id,
            // let lower_id = 
                parent_mountain2_id)
        };
        // Swap
        state.status.swap(
            state.mountains[upper_id].position.expect("Dead mountain in intersection event"),
            state.mountains[lower_id].position.expect("Dead mountain in intersection event"),
        );
        assert!(state.mountains[upper_id].position != state.mountains[lower_id].position);
        let tmp = state.mountains[lower_id].position;
        state.mountains[lower_id].position = state.mountains[upper_id].position;
        state.mountains[upper_id].position = tmp;
        // Check for intersections
        // Must check both ways because of no sorting, intersections can be discovered in both
        // directions
        if let Some(new_event) =
            find_intersection(&state.status, lower_id, state.mountains, &Direction::Above)
        {
            // handle_intersection(state, &new_event);
            state.weird_q.push_back(new_event);
        }
        if let Some(new_event) =
            find_intersection(&state.status, upper_id, state.mountains, &Direction::Below)
        {
            // handle_intersection(state, new_event);
            state.weird_q.push_back(new_event);
        }
    }
}


fn handle_death(state: &mut State, event: &Event){
    let _pos = state.mountains[event.parent_mountain_id]
        .position
        .expect("Death of dead mountain");
    // Check for floating point mess up on death/intersection Ordering
    // TODO: What is this???? feels like a bug and a bad hotfix
    // let weird_q = &mut VecDeque::new();
    // if pos != state.status.len() - 1 {
    //     while pos < state.status.len() - 1 {
    //         weird_q.push_back(state.status.pop_back().unwrap());
    //     }
    // }
    let parent_mountain_id = event.parent_mountain_id;

    // Add to ouput if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event,
        &mut state.landscapes,
        state.k,
        );
    // remove and disable
    state.status.pop_back();
    state.mountains[parent_mountain_id].position = None;
    // TODO: Same here???? L -17
    // while !weird_q.is_empty() {
    //     let element = weird_q.pop_back().unwrap();
    //     state.mountains[element].position = Some(state.mountains[element].position.unwrap() - 1);
    //     log_to_landscape(
    //         state.mountains[element],
    //         event.value,
    //         &mut state.landscapes,
    //         state.k,
    //         );
    //     state.status.push_back(element);
    // }
}

fn handle_down(state: &mut State, event: &Event){
    // Update status structures
    state.mountains[event.parent_mountain_id].slope_rising = false;

    // Add to ouput if needed
    log_to_landscape(
        state.mountains[event.parent_mountain_id],
        event,
        &mut state.landscapes,
        state.k,
        );
    // Check for intersections
    let new_event = find_intersection(
        &state.status,
        event.parent_mountain_id,
        state.mountains,
        &Direction::Below,
        );
    if let Some(intersection) = new_event{
        handle_intersection(state, intersection);
    }
    
}


#[derive(Debug)]
struct State<'a>{
    status: VecDeque<usize>,
    mountains: &'a mut Vec<&'a mut PersistenceMountain>,
    landscapes: Vec<Vec<(f32,f32)>>,
    events: BinaryHeap<Event>,
    k: usize,
    weird_q: VecDeque<Event>
}

/// # Panics
///
/// Will panic if invalid state is discovered during generation
#[must_use]
pub fn generate(bd_pairs: Vec<BirthDeath>, k: usize, debug: bool) -> Vec<Vec<(f32,f32)>> {
    let mut binding = generate_mountains(bd_pairs);
    let mut mountains: Vec<&mut PersistenceMountain> 
        = binding.iter_mut().collect();

    let mut state = State{
        events: BinaryHeap::from(generate_initial_events(&mountains)),
        status: VecDeque::new(),
        mountains: &mut mountains,
        landscapes: empty_landscape(k),
        k,
        weird_q: VecDeque::new(),
    };

    while let Some(event) = state.events.pop(){
        if debug{
            println!("{event:?}");
        }
        match event.event_type {
            EventType::Up => {
                handle_up(&mut state, &event);
            }
            EventType::Down => {
                handle_down(&mut state, &event);
            }
            EventType::Death => {
                handle_death(&mut state, &event);
            }
            EventType::Intersection => unreachable!("Event type should not be here")
        }
    }

    state.landscapes
}
