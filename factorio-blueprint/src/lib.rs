mod abstract_model;
mod model;

use core::fmt;
use std::{
    error::Error,
    io::{Read, Write},
};

use base64::DecodeError;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

use crate::{abstract_model::Blueprint, model::BlueprintContainer};

#[derive(Debug, PartialEq)]
pub enum BlueprintError {
    InvalidVersion,
    Base64Encode,
    Base64Decode(DecodeError),
    ZlibInflate,
    ZlibDeflate,
    JsonEncode,
    JsonDecode,
    JsonSerialize,
    JsonDeserialize,
}

impl fmt::Display for BlueprintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidVersion => write!(f, "blueprint has invalid version"),
            Self::Base64Encode => write!(f, "base64 encoding of blueprint failed"),
            Self::Base64Decode(ref cause) => {
                write!(f, "base64 decoding of blueprint failed: {}", cause)
            }
            Self::ZlibInflate => write!(f, "zlib inflation of blueprint failed"),
            Self::ZlibDeflate => write!(f, "zlib deflation of blueprint failed"),
            Self::JsonEncode => write!(f, "json encode of blueprint failed"),
            Self::JsonDecode => write!(f, "json decode of blueprint failed"),
            Self::JsonSerialize => write!(f, "json serialize of blueprint failed"),
            Self::JsonDeserialize => write!(f, "json deserialize of blueprint failed"),
        }
    }
}

impl Error for BlueprintError {}

pub type Result<T> = core::result::Result<T, BlueprintError>;

pub fn blueprint_string_to_json(blueprint: &str) -> Result<serde_json::Value> {
    // Check version
    match blueprint.chars().nth(0) {
        Some('0') => {}
        _ => return Err(BlueprintError::InvalidVersion),
    }

    // Base64 Decode
    let decoded = base64::decode(&blueprint[1..])
        .map_err(|cause| BlueprintError::Base64Decode(cause))?;

    // Zlib Inflate
    let mut inflator = ZlibDecoder::new(&decoded[..]);
    let mut json = String::new();
    inflator.read_to_string(&mut json)
        .or(Err(BlueprintError::ZlibInflate))?;

    // Json Decode
    serde_json::from_str(&json)
        .or(Err(BlueprintError::JsonDecode))
}

pub fn blueprint_string_to_pretty_json(blueprint: &str) -> Result<String> {
    let json = blueprint_string_to_json(blueprint)?;
    serde_json::to_string_pretty(&json)
        .or(Err(BlueprintError::JsonEncode))
}

pub fn blueprint_string_to_raw_model(blueprint: &str) -> Result<BlueprintContainer> {
    let pretty = blueprint_string_to_pretty_json(blueprint)?;

    // Json Deserialize
    // Using pretty instead of the raw json string makes errors use more usefull line numbers.
    serde_json::from_str(&pretty)
        .or(Err(BlueprintError::JsonDeserialize))
}

pub fn blueprint_string_to_model(blueprint: &str) -> Result<Blueprint> {
    let raw_model = blueprint_string_to_raw_model(blueprint)?;
    Ok(Blueprint::from(raw_model))
}

pub fn raw_model_to_pretty_json(raw_model: &BlueprintContainer) -> Result<String> {
    // Json Serialize
    serde_json::to_string_pretty(raw_model)
        .or(Err(BlueprintError::JsonEncode))
}

pub fn model_to_pretty_json(model: Blueprint) -> Result<String> {
    let raw_model: BlueprintContainer = model.into();
    raw_model_to_pretty_json(&raw_model)
}

pub fn raw_model_to_blueprint_string(raw_model: &BlueprintContainer) -> Result<String> {
    let json = raw_model_to_pretty_json(raw_model)?;

    // Zlib Deflate
    let mut deflator = ZlibEncoder::new(Vec::new(), Compression::fast());
    deflator.write_all(json.as_bytes())
        .or(Err(BlueprintError::ZlibDeflate))?;
    let compressed = deflator.finish()
        .or(Err(BlueprintError::ZlibDeflate))?;

    // Add Version
    let mut encoded = String::from("0");

    // Base64 Encode
    base64::encode_config_buf(&compressed, base64::STANDARD, &mut encoded);

    Ok(encoded)
}

pub fn model_to_blueprint_string(model: Blueprint) -> Result<String> {
    let raw_model: BlueprintContainer = model.into();
    raw_model_to_blueprint_string(&raw_model)
}
