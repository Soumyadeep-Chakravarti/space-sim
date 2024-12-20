// src/structures/nasa.rs

use serde::{Deserialize, Serialize};
use crate::structures::validation::{Validatable, ValidationError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    version: String,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinDv {
    dv: String,
    dur: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinDur {
    dv: String,
    dur: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsteroidData {
    des: String,
    fullname: String,
    orbit_id: String,
    h: String,
    min_size: String,
    max_size: String,
    size: Option<String>,
    size_sigma: Option<String>,
    occ: String,
    min_dv: MinDv,
    min_dur: MinDur,
    n_via_traj: u32,
    obs_start: String,
    obs_end: String,
    obs_mag: String,
    obs_flag: String,
    radar_obs_a: Option<String>,
    radar_snr_a: Option<String>,
    radar_obs_g: Option<String>,
    radar_snr_g: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NHAT_NASAResponse {
    signature: Signature,
    count: u32,
    data: Vec<AsteroidData>,
}

impl NHAT_NASAResponse {
    fn validate_signature(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.signature.version.is_empty() {
            errors.push(ValidationError::new("signature.version", "Version is required"));
        }
        if self.signature.source.is_empty() {
            errors.push(ValidationError::new("signature.source", "Source is required"));
        }
        errors
    }

    fn validate_asteroid_data(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        for (index, asteroid) in self.data.iter().enumerate() {
            if asteroid.des.is_empty() {
                errors.push(ValidationError::new(
                    &format!("data[{}].des", index),
                    "Description (des) is required",
                ));
            }
            if asteroid.fullname.is_empty() {
                errors.push(ValidationError::new(
                    &format!("data[{}].fullname", index),
                    "Full name is required",
                ));
            }
            if asteroid.obs_start.is_empty() || asteroid.obs_end.is_empty() {
                errors.push(ValidationError::new(
                    &format!("data[{}].obs_start/obs_end", index),
                    "Observation start or end date is required",
                ));
            }
        }
        errors
    }
}

impl Validatable for NasaResponse {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate signature and asteroid data
        errors.extend(self.validate_signature());
        errors.extend(self.validate_asteroid_data());

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
