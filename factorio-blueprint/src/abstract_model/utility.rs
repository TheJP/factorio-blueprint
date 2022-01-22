use std::{fmt, error::Error, collections::HashMap};

use crate::model::Position;

use super::{Blueprint, Entity, Connection};

#[derive(Debug, PartialEq)]
pub enum UtilityError {
    InvalidId(usize),
    DuplicateIds,
}

impl fmt::Display for UtilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidId(id) => write!(f, "id '{}' is invalid", id),
            Self::DuplicateIds => write!(f, "received duplicate ids which is not allowed for this function"),
        }
    }
}

impl Error for UtilityError {}

pub type Result<T> = core::result::Result<T, UtilityError>;

impl Blueprint {
    pub fn clone_entities(&mut self, ids: &Vec<usize>) -> Result<Vec<usize>> {
        if let Some(id) = self.contains_invalid_id(&ids) {
            return Err(UtilityError::InvalidId(id));
        }

        let mut next_id = self.entities.len();
        let mut id_map = HashMap::new();
        for id in ids {
            let had_value = id_map.insert(*id, next_id);
            next_id += 1;

            if let Some(_) = had_value {
                return Err(UtilityError::DuplicateIds);
            }
        }

        for (old_id, new_id) in &id_map {
            let mut entity = self.entities[*old_id].clone();
            entity.update_id(*new_id);
            entity.update_connections(&id_map);
            self.entities.push(entity);
        }

        Ok(id_map.into_values().collect())
    }

    fn contains_invalid_id(&self, ids: &Vec<usize>) -> Option<usize> {
        for id in ids {
            let id = *id;
            if id >= self.entities.len() {
                return Some(id);
            }
            if let Entity::Unknown(_) = self.entities[id] {
                return Some(id);
            }
        }

        None
    }
}

impl Entity {
    pub fn id(&self) -> usize {
        match self {
            Entity::DeciderCombinator(dc) => dc.id,
            Entity::ArithmeticCombinator(ac) => ac.id,
            Entity::ElectricPole(ep) => ep.id,
            Entity::Unknown(e) => (e.entity_number - 1) as usize,
        }
    }

    fn update_id(&mut self, new_id: usize) {
        match self {
            Entity::DeciderCombinator(e) => e.id = new_id,
            Entity::ArithmeticCombinator(e) => e.id = new_id,
            Entity::ElectricPole(e) => e.id = new_id,
            Entity::Unknown(e) => e.entity_number = (new_id + 1) as u32,
        }
    }

    pub fn position(&self) -> &Position {
        match self {
            Entity::DeciderCombinator(e) => &e.position,
            Entity::ArithmeticCombinator(e) => &e.position,
            Entity::ElectricPole(e) => &e.position,
            Entity::Unknown(e) => &e.position,
        }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        match self {
            Entity::DeciderCombinator(e) => &mut e.position,
            Entity::ArithmeticCombinator(e) => &mut e.position,
            Entity::ElectricPole(e) => &mut e.position,
            Entity::Unknown(e) => &mut e.position,
        }
    }

    fn update_connections(&mut self, id_map: &HashMap<usize, usize>) {
        let update = |connections: &mut Vec<Connection>| {
            for c in connections {
                c.update_connections(id_map)
            }
        };

        match self {
            Entity::DeciderCombinator(e) => update(&mut e.connections),
            Entity::ArithmeticCombinator(e) => update(&mut e.connections),
            Entity::ElectricPole(e) => {
                update(&mut e.connections);

                for neighbour in &mut e.neighbours {
                    if let Some(new_id) = id_map.get(&neighbour) {
                        *neighbour = *new_id;
                    }
                }
            },
            Entity::Unknown(_) => {},
        }
    }
}

impl Connection {
    fn update_connections(&mut self, id_map: &HashMap<usize, usize>) {
        if let Some(new_id) = id_map.get(&self.to.id) {
            self.to.id = *new_id;
        }
    }
}
