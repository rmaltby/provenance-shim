// SPDX-License-Identifier: GPL-3.0-or-later OR Apache-2.0

use crate::model::Provenance;

pub fn to_yaml(prov: &Provenance) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(prov)
}

pub fn from_yaml(s: &str) -> Result<Provenance, serde_yaml::Error> {
    serde_yaml::from_str(s)
}

pub fn to_json(prov: &Provenance) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(prov)
}

pub fn from_json(s: &str) -> Result<Provenance, serde_json::Error> {
    serde_json::from_str(s)
}
