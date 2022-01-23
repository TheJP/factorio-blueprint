use std::{collections::HashMap, error::Error, fmt, cmp::{min, max}};

use crate::model::Position;

use super::{Blueprint, Connection, Entity};

#[derive(Debug, PartialEq)]
pub enum UtilityError {
    InvalidId(usize),
    DuplicateIds,
}

impl fmt::Display for UtilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidId(id) => write!(f, "id '{}' is invalid", id),
            Self::DuplicateIds => write!(
                f,
                "received duplicate ids which is not allowed for this function"
            ),
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
        for &id in ids {
            let had_value = id_map.insert(id, next_id);
            next_id += 1;

            if let Some(_) = had_value {
                return Err(UtilityError::DuplicateIds);
            }
        }

        let mut new_entities = Vec::new();
        for (&old_id, &new_id) in &id_map {
            let mut entity = self.entities[old_id].clone();
            entity.update_id(new_id);
            entity.update_connections(&id_map);
            new_entities.push(entity);
        }

        new_entities.sort_by(|a, b|a.id().cmp(&b.id()));
        self.entities.append(&mut new_entities);

        Ok(id_map.into_values().collect())
    }

    fn contains_invalid_id(&self, ids: &Vec<usize>) -> Option<usize> {
        for &id in ids {
            if self.id_invalid(id) {
                return Some(id);
            }
        }

        None
    }

    fn id_invalid(&self, id: usize) -> bool {
        if id >= self.entities.len() {
            return true;
        }

        if let Entity::Unknown(_) = self.entities[id] {
            return true;
        }

        false
    }

    pub fn connect_electric_poles(&mut self, id1: usize, id2: usize) -> Result<()> {
        if id1 == id2 {
            return Err(UtilityError::DuplicateIds);
        }

        if self.id_invalid(id1) {
            return Err(UtilityError::InvalidId(id1));
        }

        if self.id_invalid(id2) {
            return Err(UtilityError::InvalidId(id2));
        }

        let (id1, id2) = (min(id1, id2), max(id1, id2));

        let (left, right) = self.entities.split_at_mut(id2);

        match (&mut left[id1], &mut right[0]) {
            (
                Entity::ElectricPole { neighbours: neighbours1, .. },
                Entity::ElectricPole { neighbours: neighbours2, .. },
            ) => {
                if !neighbours1.contains(&id2) {
                    neighbours1.push(id2);
                }

                if !neighbours2.contains(&id1) {
                    neighbours2.push(id1);
                }

                Ok(())
            },
            (Entity::ElectricPole { .. }, _) => Err(UtilityError::InvalidId(id2)),
            _ => Err(UtilityError::InvalidId(id1)),
        }
    }
}

impl Entity {
    pub fn id(&self) -> usize {
        match self {
            Entity::DeciderCombinator { id, .. }
            | Entity::ArithmeticCombinator { id, .. }
            | Entity::ElectricPole { id, .. } => *id,
            Entity::Unknown(e) => (e.entity_number - 1) as usize,
        }
    }

    fn update_id(&mut self, new_id: usize) {
        match self {
            Entity::DeciderCombinator { id, .. }
            | Entity::ArithmeticCombinator { id, .. }
            | Entity::ElectricPole { id, .. } => *id = new_id,
            Entity::Unknown(e) => e.entity_number = (new_id + 1) as u32,
        }
    }

    pub fn position(&self) -> &Position {
        match self {
            Entity::DeciderCombinator { position, .. }
            | Entity::ArithmeticCombinator { position, .. }
            | Entity::ElectricPole { position, .. } => position,
            Entity::Unknown(e) => &e.position,
        }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        match self {
            Entity::DeciderCombinator { position, .. }
            | Entity::ArithmeticCombinator { position, .. }
            | Entity::ElectricPole { position, .. } => position,
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
            Entity::DeciderCombinator { connections, .. }
            | Entity::ArithmeticCombinator { connections, .. } => update(connections),
            Entity::ElectricPole {
                connections,
                neighbours,
                ..
            } => {
                update(connections);

                for neighbour in neighbours {
                    if let Some(new_id) = id_map.get(&neighbour) {
                        *neighbour = *new_id;
                    }
                }
            }
            Entity::Unknown(_) => {}
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
