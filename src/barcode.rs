use crate::birthdeath::BirthDeath;

#[derive(Debug)]
enum EventType {
    Birth,
    Death,
}

#[derive(Debug)]
struct Event {
    event_type: EventType,
    value: f32,
    id: usize,
}

fn generate_events(bd_pairs: Vec<BirthDeath>) -> Vec<BirthDeath> {
    let _tmp = bd_pairs.into_iter().enumerate().map(|(i, bd)| match bd {
        BirthDeath { birth, death } if death.is_finite() && birth.is_finite() => {
            vec![
                Event {
                    event_type: EventType::Birth,
                    value: birth,
                    id: i,
                },
                Event {
                    event_type: EventType::Death,
                    value: death,
                    id: i,
                },
            ]
        }
        _ => Vec::new(),
    });
    return Vec::new();
}

pub fn barcode_filter(bd_pairs: Vec<BirthDeath>, k: i32) -> Vec<BirthDeath> {
    let events = generate_events(bd_pairs);

    return events;
}
