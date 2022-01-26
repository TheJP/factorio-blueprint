use std::{
    cmp::{max, min},
    collections::HashMap,
    error::Error,
    fmt,
};

use crate::model::Position;

use super::{Blueprint, Connection, Connector, Entity, Side, Wire};

#[derive(Debug, PartialEq)]
pub enum UtilityError {
    InvalidId(usize),
    DuplicateIds,
    InvalidOperation,
}

impl fmt::Display for UtilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidId(id) => write!(f, "id '{}' is invalid", id),
            Self::DuplicateIds => write!(
                f,
                "received duplicate ids which is not allowed for this function"
            ),
            Self::InvalidOperation => write!(f, "tried to perform an invalid operation"),
        }
    }
}

impl Error for UtilityError {}

pub type Result<T> = core::result::Result<T, UtilityError>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SideCount {
    Zero,
    One,
    Two,
}

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

        new_entities.sort_by(|a, b| a.id().cmp(&b.id()));
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
                Entity::ElectricPole {
                    neighbours: neighbours1,
                    ..
                },
                Entity::ElectricPole {
                    neighbours: neighbours2,
                    ..
                },
            ) => {
                if !neighbours1.contains(&id2) {
                    neighbours1.push(id2);
                }

                if !neighbours2.contains(&id1) {
                    neighbours2.push(id1);
                }

                Ok(())
            }
            (Entity::ElectricPole { .. }, _) => Err(UtilityError::InvalidId(id2)),
            _ => Err(UtilityError::InvalidId(id1)),
        }
    }

    pub fn connect_wire(&mut self, id1: usize, id2: usize, wire: Wire) -> Result<()> {
        self.connect_wire_with_side(
            Connector {
                id: id1,
                side: Side::One,
            },
            Connector {
                id: id2,
                side: Side::One,
            },
            wire,
        )
    }

    pub fn connect_wire_with_side(
        &mut self,
        c1: Connector,
        c2: Connector,
        wire: Wire,
    ) -> Result<()> {
        if c1.id == c2.id && c1.side == c2.side {
            return Err(UtilityError::DuplicateIds);
        }

        if self.id_invalid(c1.id) || !self.entities[c1.id].can_connect(c1.side) {
            return Err(UtilityError::InvalidId(c1.id));
        }

        if self.id_invalid(c2.id) || !self.entities[c2.id].can_connect(c2.side) {
            return Err(UtilityError::InvalidId(c2.id));
        }

        let result1 = self.entities[c1.id].connections_mut().map(|cs| {
            cs.push(Connection {
                from_side: c1.side,
                to: c2.clone(),
                wire,
            })
        });

        let result2 = self.entities[c2.id].connections_mut().map(|cs| {
            cs.push(Connection {
                from_side: c2.side,
                to: c1.clone(),
                wire,
            })
        });

        match (result1, result2) {
            (Some(_), Some(_)) => Ok(()),
            _ => unreachable!(),
        }
    }
}

impl Entity {
    pub fn id(&self) -> usize {
        match self {
            Entity::DeciderCombinator { id, .. }
            | Entity::ArithmeticCombinator { id, .. }
            | Entity::ConstantCombinator { id, .. }
            | Entity::ElectricPole { id, .. } => *id,
            Entity::Unknown(e) => (e.entity_number - 1) as usize,
        }
    }

    fn update_id(&mut self, new_id: usize) {
        match self {
            Entity::DeciderCombinator { id, .. }
            | Entity::ArithmeticCombinator { id, .. }
            | Entity::ConstantCombinator { id, .. }
            | Entity::ElectricPole { id, .. } => *id = new_id,
            Entity::Unknown(e) => e.entity_number = (new_id + 1) as u32,
        }
    }

    pub fn position(&self) -> &Position {
        match self {
            Entity::DeciderCombinator { position, .. }
            | Entity::ArithmeticCombinator { position, .. }
            | Entity::ConstantCombinator { position, .. }
            | Entity::ElectricPole { position, .. } => position,
            Entity::Unknown(e) => &e.position,
        }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        match self {
            Entity::DeciderCombinator { position, .. }
            | Entity::ArithmeticCombinator { position, .. }
            | Entity::ConstantCombinator { position, .. }
            | Entity::ElectricPole { position, .. } => position,
            Entity::Unknown(e) => &mut e.position,
        }
    }

    pub fn side_count(&self) -> SideCount {
        match self {
            Entity::DeciderCombinator { .. }
            | Entity::ArithmeticCombinator { .. }
            | Entity::ConstantCombinator { .. } => SideCount::Two,
            Entity::ElectricPole { .. } => SideCount::One,
            Entity::Unknown(_) => SideCount::Zero,
        }
    }

    pub fn can_connect(&self, side: Side) -> bool {
        match (self.side_count(), side) {
            (SideCount::Two, _) | (SideCount::One, Side::One) => true,
            _ => false,
        }
    }

    pub fn connections(&self) -> Option<&Vec<Connection>> {
        match self {
            Entity::DeciderCombinator { connections, .. }
            | Entity::ArithmeticCombinator { connections, .. }
            | Entity::ConstantCombinator { connections, .. }
            | Entity::ElectricPole { connections, .. } => Some(connections),
            Entity::Unknown(_) => None,
        }
    }

    pub fn connections_mut(&mut self) -> Option<&mut Vec<Connection>> {
        match self {
            Entity::DeciderCombinator { connections, .. }
            | Entity::ArithmeticCombinator { connections, .. }
            | Entity::ConstantCombinator { connections, .. }
            | Entity::ElectricPole { connections, .. } => Some(connections),
            Entity::Unknown(_) => None,
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
            | Entity::ArithmeticCombinator { connections, .. }
            | Entity::ConstantCombinator { connections, .. } => update(connections),
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
