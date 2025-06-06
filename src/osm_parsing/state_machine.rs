use crate::osm_parsing::{CurrentlyReading, StateMachine};

impl StateMachine {
    pub fn new() -> Self {
        let currently_reading = CurrentlyReading::None;

        StateMachine { currently_reading }
    }

    pub fn update(&mut self, currently_reading: CurrentlyReading) {
        self.currently_reading = currently_reading;
    }

    pub fn current_status(&self) -> &CurrentlyReading {
        &self.currently_reading
    }

    fn _current_id(&self) -> Option<u64> {
        match self.currently_reading {
            CurrentlyReading::Node(id) => Some(id),
            CurrentlyReading::Way(id) => Some(id),
            CurrentlyReading::Relation(id) => Some(id),
            CurrentlyReading::None => None,
        }
    }
}
