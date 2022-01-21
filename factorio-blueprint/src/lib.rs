mod model;
mod abstract_model;

use core::fmt;
use std::{io::Read, error::Error};

use base64::DecodeError;
use flate2::read::ZlibDecoder;

use crate::{model::BlueprintContainer, abstract_model::Blueprint};

#[derive(Debug, PartialEq)]
pub enum BlueprintError {
    InvalidVersion,
    Base64Decode(DecodeError),
    ZlibInflate,
    JsonDecode,
    JsonDeserialize,
    JsonEncode,
    JsonSerialize,
}

impl fmt::Display for BlueprintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidVersion => write!(f, "blueprint has invalid version"),
            Self::Base64Decode(ref cause) => write!(f, "base64 decoding of blueprint failed: {}", cause),
            Self::ZlibInflate => write!(f, "zlib inflation of blueprint failed"),
            Self::JsonDecode => write!(f, "json decode of blueprint failed"),
            Self::JsonDeserialize => write!(f, "json deserialize of blueprint failed"),
            Self::JsonEncode => write!(f, "json encode of blueprint failed"),
            Self::JsonSerialize => write!(f, "json serialize of blueprint failed"),
        }
    }
}

impl Error for BlueprintError {}

pub type Result<T> = core::result::Result<T, BlueprintError>;

pub fn blueprint_string_to_json(blueprint: &str) -> Result<serde_json::Value> {
    // Check version
    match blueprint.chars().nth(0) {
        Some('0') => {}
        _ => return Err(BlueprintError::InvalidVersion)
    }

    // Base64 Decode
    let decoded = base64::decode(&blueprint[1..])
        .map_err(|cause|BlueprintError::Base64Decode(cause))?;

    // Zlib Inflate
    let mut inflator = ZlibDecoder::new(&decoded[..]);
    let mut json = String::new();
    inflator.read_to_string(&mut json)
        .or(Err(BlueprintError::ZlibInflate))?;

    // Json Decode
    serde_json::from_str(&json)
        .or(Err(BlueprintError::JsonDecode))
}

pub fn blueprint_string_to_raw_model(blueprint: &str) -> Result<BlueprintContainer> {
    let json = blueprint_string_to_json(blueprint)?;
    let pretty = serde_json::to_string_pretty(&json)
        .or(Err(BlueprintError::JsonEncode))?;

    // Json Deserialize
    // Using pretty instead of the raw json string makes errors use more usefull line numbers.
    serde_json::from_str(&pretty)
        .or(Err(BlueprintError::JsonDeserialize))
}

pub fn blueprint_string_to_model(blueprint: &str) -> Result<Blueprint> {
    let raw_model = blueprint_string_to_raw_model(blueprint)?;
    Ok(Blueprint::from(raw_model))
}

pub fn model_to_pretty_json(model: Blueprint) -> Result<String> {
    let raw_model: BlueprintContainer = model.into();
    serde_json::to_string_pretty(&raw_model)
        .or(Err(BlueprintError::JsonEncode))
}
