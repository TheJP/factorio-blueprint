pub mod utility;

use crate::model::{self, CircuitId, ConnectionPoint};

#[derive(Debug)]
pub struct Blueprint {
    pub entities: Vec<Entity>,
    pub version: u64,
    pub icons: Vec<model::Icon>,
}

#[derive(Clone, Debug)]
pub enum Entity {
    DeciderCombinator {
        id: usize,
        position: model::Position,
        direction: model::Direction,

        connections: Vec<Connection>,

        condition: model::DeciderCondition,
    },
    ArithmeticCombinator {
        id: usize,
        position: model::Position,
        direction: model::Direction,

        connections: Vec<Connection>,

        condition: model::ArithmeticCondition,
    },
    ElectricPole {
        id: usize,
        pole_type: PoleType,
        position: model::Position,
        neighbours: Vec<usize>,
        connections: Vec<Connection>,
    },
    Unknown(model::Entity),
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub from_side: Side,
    pub to: Connector,
    pub wire: Wire,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Connector {
    pub id: usize,
    pub side: Side,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    One,
    Two,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Wire {
    Red,
    Green,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PoleType {
    Medium,
}

impl From<model::BlueprintContainer> for Blueprint {
    fn from(bc: model::BlueprintContainer) -> Self {
        let b = bc.blueprint;
        Self::assert_compact_ascending_ids(&b);

        Blueprint {
            version: b.version,
            icons: b.icons,
            entities: b.entities.into_iter().map(Entity::from).collect(),
        }
    }
}

impl Blueprint {
    fn assert_compact_ascending_ids(b: &model::Blueprint) {
        let mut expected_id = 1u32;
        for entity in &b.entities {
            assert_eq!(expected_id, entity.entity_number);
            expected_id += 1;
        }
    }
}

impl Into<model::BlueprintContainer> for Blueprint {
    fn into(self) -> model::BlueprintContainer {
        model::BlueprintContainer {
            blueprint: model::Blueprint {
                version: self.version,
                icons: self.icons,
                item: model::Item::Blueprint,
                entities: self.entities.into_iter().map(Entity::into).collect(),
            },
        }
    }
}

impl From<model::Entity> for Entity {
    fn from(e: model::Entity) -> Self {
        let id = (e.entity_number - 1) as usize;
        match e.name.as_str() {
            "decider-combinator" => Self::decider_combinator(id, e),
            "arithmetic-combinator" => Self::arithmetic_combinator(id, e),
            "medium-electric-pole" => Self::electric_pole(id, PoleType::Medium, e),
            _ => Entity::Unknown(e),
        }
    }
}

impl Into<model::Entity> for Entity {
    fn into(self) -> model::Entity {
        match self {
            Self::Unknown(e) => e,
            Self::DeciderCombinator {
                id,
                position,
                direction,
                connections,
                condition,
            } => model::Entity {
                entity_number: (id + 1) as u32,
                name: "decider-combinator".into(),
                position,
                direction: Some(direction),
                neighbours: None,
                control_behavior: Some(model::ControlBehavior {
                    arithmetic_conditions: None,
                    decider_conditions: Some(condition),
                }),
                connections: Connection::to_model(connections),
            },
            Self::ArithmeticCombinator {
                id,
                position,
                direction,
                connections,
                condition,
            } => model::Entity {
                entity_number: (id + 1) as u32,
                name: "arithmetic-combinator".into(),
                position,
                direction: Some(direction),
                neighbours: None,
                control_behavior: Some(model::ControlBehavior {
                    arithmetic_conditions: Some(condition),
                    decider_conditions: None,
                }),
                connections: Connection::to_model(connections),
            },
            Self::ElectricPole {
                id,
                pole_type,
                position,
                neighbours,
                connections,
            } => model::Entity {
                entity_number: (id + 1) as u32,
                name: pole_type.name().into(),
                position,
                direction: None,
                neighbours: Some(neighbours.into_iter().map(|id| (id + 1) as u32).collect()),
                control_behavior: None,
                connections: Connection::to_model(connections),
            },
        }
    }
}

impl Entity {
    fn decider_combinator(id: usize, e: model::Entity) -> Self {
        Entity::DeciderCombinator {
            id,
            position: e.position,
            direction: e.direction.unwrap(),

            connections: Connection::from_model(e.connections),

            condition: e.control_behavior.unwrap().decider_conditions.unwrap(),
        }
    }

    fn arithmetic_combinator(id: usize, e: model::Entity) -> Self {
        Entity::ArithmeticCombinator {
            id,
            position: e.position,
            direction: e.direction.unwrap(),

            connections: Connection::from_model(e.connections),

            condition: e.control_behavior.unwrap().arithmetic_conditions.unwrap(),
        }
    }

    fn electric_pole(id: usize, pole_type: PoleType, e: model::Entity) -> Self {
        let neighbours = e
            .neighbours
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|n| (n - 1) as usize)
            .collect();
        Entity::ElectricPole {
            id,
            pole_type,
            neighbours,
            position: e.position,

            connections: Connection::from_model(e.connections),
        }
    }
}

impl Connection {
    pub fn from_model(cs: Option<model::Connection>) -> Vec<Connection> {
        match cs {
            None => Vec::new(),
            Some(cs) => {
                let mut v1 = cs
                    .connection1
                    .map_or(Vec::new(), |cp| Self::with_side(Side::One, cp));
                let mut v2 = cs
                    .connection2
                    .map_or(Vec::new(), |cp| Self::with_side(Side::Two, cp));

                v1.append(&mut v2);
                v1
            }
        }
    }

    fn with_side(from_side: Side, cs: model::ConnectionPoint) -> Vec<Connection> {
        let mut red = cs.red.map_or(Vec::new(), |c| {
            Self::with_side_and_wire(from_side, Wire::Red, c)
        });
        let mut green = cs.green.map_or(Vec::new(), |c| {
            Self::with_side_and_wire(from_side, Wire::Green, c)
        });

        red.append(&mut green);
        red
    }

    fn with_side_and_wire(
        from_side: Side,
        wire: Wire,
        cs: Vec<model::ConnectionData>,
    ) -> Vec<Connection> {
        cs.into_iter()
            .map(|c| Connection {
                from_side,
                to: Connector {
                    id: (c.entity_id - 1) as usize,
                    side: c.circuit_id.into(),
                },
                wire,
            })
            .collect()
    }

    pub fn to_model(cs: Vec<Connection>) -> Option<model::Connection> {
        if cs.is_empty() {
            return None;
        }

        let (mut one_red, mut one_green, mut two_red, mut two_green) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for c in cs {
            match c.from_side {
                Side::One => match c.wire {
                    Wire::Red => one_red.push(c),
                    Wire::Green => one_green.push(c),
                },
                Side::Two => match c.wire {
                    Wire::Red => two_red.push(c),
                    Wire::Green => two_green.push(c),
                },
            }
        }

        Some(model::Connection {
            connection1: Self::connection_point(one_red, one_green),
            connection2: Self::connection_point(two_red, two_green),
        })
    }

    fn connection_point(red: Vec<Connection>, green: Vec<Connection>) -> Option<ConnectionPoint> {
        if green.is_empty() && red.is_empty() {
            None
        } else {
            Some(model::ConnectionPoint {
                red: Self::connection_data(red),
                green: Self::connection_data(green),
            })
        }
    }

    fn connection_data(cs: Vec<Connection>) -> Option<Vec<model::ConnectionData>> {
        if cs.is_empty() {
            None
        } else {
            let data = cs
                .into_iter()
                .map(|c| model::ConnectionData {
                    entity_id: (c.to.id + 1) as u32,
                    circuit_id: c.to.side.into(),
                })
                .collect();
            Some(data)
        }
    }
}

impl From<Option<CircuitId>> for Side {
    fn from(c_id: Option<CircuitId>) -> Self {
        match c_id {
            None | Some(CircuitId::One) => Self::One,
            Some(CircuitId::Two) => Self::Two,
        }
    }
}

impl Into<Option<CircuitId>> for Side {
    fn into(self) -> Option<CircuitId> {
        match self {
            Self::One => Some(CircuitId::One), // TODO: Is Some(One) instead of None a problem?
            Self::Two => Some(CircuitId::Two),
        }
    }
}

impl PoleType {
    pub fn name(&self) -> &'static str {
        match &self {
            Self::Medium => "medium-electric-pole",
        }
    }
}
