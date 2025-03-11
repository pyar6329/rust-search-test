mod decimal_value;
mod deseralize;
mod load_data;
mod movie;
mod player;

pub use decimal_value::*;
pub use deseralize::*;
pub use load_data::*;
pub use movie::*;
pub use player::*;

use anyhow::{Error, Result, anyhow};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::search::GetAttributes;
