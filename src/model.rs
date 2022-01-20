use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BlueprintContainer {
    blueprint: Blueprint,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Blueprint {
    entities: Vec<Entity>,
    version: u64,
    item: Item,
    icons: Vec<Icon>,
}

/// Forces item to be set to "blueprint".
/// Possibly also other values if we find that they can occur.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Item {
    #[serde(rename = "blueprint")]
    Blueprint,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Icon {
    index: u32,
    signal: Signal,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entity {
    entity_number: u32,
    name: String,
    position: Position,

    #[serde(default)]
    direction: Option<Direction>,

    #[serde(default)]
    connections: Option<Connection>,

    #[serde(default)]
    control_behavior: Option<ControlBehavior>,

    #[serde(default)]
    neighbours: Option<Vec<u32>>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Connection {
    #[serde(rename = "1")]
    #[serde(default)]
    connection1: Option<ConnectionPoint>,

    #[serde(rename = "2")]
    #[serde(default)]
    connection2: Option<ConnectionPoint>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConnectionPoint {
    #[serde(default)]
    green: Option<Vec<ConnectionData>>,

    #[serde(default)]
    red: Option<Vec<ConnectionData>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConnectionData {
    /// Id (also called entity_number) of the target of the connection.
    entity_id: u32,

    /// Used for targets with multiple possible connection points,
    /// to determine where this wire connects to.
    #[serde(default)]
    circuit_id: Option<CircuitId>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CircuitId {
    One = 1,
    Two = 2,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ControlBehavior {
    #[serde(default)]
    decider_conditions: Option<DeciderCondition>,

    #[serde(default)]
    arithmetic_conditions: Option<ArithmeticCondition>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Signal {
    name: String,

    #[serde(rename = "type")]
    signal_type: SignalType,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SignalType {
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "item")]
    Item,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeciderCondition {
    comparator: Comparator,
    copy_count_from_input: bool,

    #[serde(default)]
    constant: Option<i32>,


    #[serde(default)]
    first_signal: Option<Signal>,

    #[serde(default)]
    second_signal: Option<Signal>,

    #[serde(default)]
    output_signal: Option<Signal>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Comparator {
    /// Greater than (>)
    #[serde(rename = ">")]
    Gt,

    // Less than (<)
    #[serde(rename = "<")]
    Lt,

    // Equals (=)
    #[serde(rename = "=")]
    Eq,

    // Greater than or equal to (>=)
    #[serde(rename = "≥")]
    Ge,

    // Less than or equal to (<=)
    #[serde(rename = "≤")]
    Le,

    // Not equal to (!=)
    #[serde(rename = "≠")]
    Neq,
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ArithmeticCondition {
    operation: Operation,

    #[serde(default)]
    first_constant: Option<i32>,

    #[serde(default)]
    second_constant: Option<i32>,

    #[serde(default)]
    first_signal: Option<Signal>,

    #[serde(default)]
    second_signal: Option<Signal>,

    #[serde(default)]
    output_signal: Option<Signal>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Operation {
    /// Addition (+)
    #[serde(rename = "+")]
    Add,

    /// Subtraction (-)
    #[serde(rename = "-")]
        Sub,

    /// Multiplication (*)
    #[serde(rename = "*")]
    Mul,

    /// Division (/)
    #[serde(rename = "/")]
    Div,

    /// Modulo (%)
    #[serde(rename = "%")]
    Mod,

    /// Exponentiation
    #[serde(rename = "^")]
    Pow,

    /// Bit Left Shift (<<)
    #[serde(rename = "<<")]
    Shl,

    /// Bit Right Shift (>>)
    #[serde(rename = ">>")]
    Shr,

    /// Bitwise AND (&)
    #[serde(rename = "AND")]
    And,

    /// Bitwise OR (|)
    #[serde(rename = "OR")]
    Or,

    /// Bitwise XOR (^)
    #[serde(rename = "XOR")]
    Xor,
}
