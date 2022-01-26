use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BlueprintContainer {
    pub blueprint: Blueprint,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Blueprint {
    pub entities: Vec<Entity>,
    pub version: u64,
    pub item: Item,
    pub icons: Vec<Icon>,
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
    pub index: u32,
    pub signal: Signal,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Entity {
    pub entity_number: u32,
    pub name: String,
    pub position: Position,

    #[serde(default)]
    pub direction: Option<Direction>,

    #[serde(default)]
    pub connections: Option<Connection>,

    #[serde(default)]
    pub control_behavior: Option<ControlBehavior>,

    #[serde(default)]
    pub neighbours: Option<Vec<u32>>,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Connection {
    #[serde(rename = "1")]
    #[serde(default)]
    pub connection1: Option<ConnectionPoint>,

    #[serde(rename = "2")]
    #[serde(default)]
    pub connection2: Option<ConnectionPoint>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ConnectionPoint {
    #[serde(default)]
    pub green: Option<Vec<ConnectionData>>,

    #[serde(default)]
    pub red: Option<Vec<ConnectionData>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ConnectionData {
    /// Id (also called entity_number) of the target of the connection.
    pub entity_id: u32,

    /// Used for targets with multiple possible connection points,
    /// to determine where this wire connects to.
    #[serde(default)]
    pub circuit_id: Option<CircuitId>,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum CircuitId {
    One = 1,
    Two = 2,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ControlBehavior {
    #[serde(default)]
    pub decider_conditions: Option<DeciderCondition>,

    #[serde(default)]
    pub arithmetic_conditions: Option<ArithmeticCondition>,

    #[serde(default)]
    pub filters: Option<Vec<ConstantCondition>>,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Signal {
    pub name: String,

    #[serde(rename = "type")]
    pub signal_type: SignalType,
}


#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum SignalType {
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "item")]
    Item,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DeciderCondition {
    pub comparator: Comparator,
    pub copy_count_from_input: bool,

    #[serde(default)]
    pub constant: Option<i32>,

    #[serde(default)]
    pub first_signal: Option<Signal>,

    #[serde(default)]
    pub second_signal: Option<Signal>,

    #[serde(default)]
    pub output_signal: Option<Signal>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
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
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ArithmeticCondition {
    pub operation: Operation,

    #[serde(default)]
    pub first_constant: Option<i32>,

    #[serde(default)]
    pub second_constant: Option<i32>,

    #[serde(default)]
    pub first_signal: Option<Signal>,

    #[serde(default)]
    pub second_signal: Option<Signal>,

    #[serde(default)]
    pub output_signal: Option<Signal>,
}


#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ConstantCondition {
    pub count: i32,
    pub index: u8, // only values in range (0,21] are valid
    pub signal: Signal,
}
