use rdev::Key;
use serde::{Deserialize, Serialize};

/// Simple struct to have labeled entries in the serialized JSON, instead of a tuple or fixed size array of 2 elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSignature {
    numerator: i32,
    denominator: i32
}

/// Struct containing the current passrhythm information
/// first_input is the time of the song where the first input happens, needed for offset settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmConfig {
    pub first_input: i32,
    pub bpm: i32,
    pub time_signature: TimeSignature
}

/// Reads RhythmConfig.json and returns a RhythmConfig struct
pub fn read_rhythm_config() -> Result<RhythmConfig, String> {
    Ok(RhythmConfig {
        first_input: 0,
        bpm: 0,
        time_signature: TimeSignature {
            numerator: 4,
            denominator: 4
        }
    })
}

/// Updates RhythmConfig.json from a RhythmConfig struct
pub fn write_rhythm_config(rhythm_config: &RhythmConfig) {
    todo!()
}

/// Struct containing user configs, filled by reading from UserConfig.json, and used to write to UserConfig.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub offset: i32,
    pub keybinds: [Option<Key>; 10]
}

/// Reads UserConfig.json and returns a UserConfig struct
pub fn read_user_config() -> Result<UserConfig, String> {
    Ok(UserConfig {
        offset: 0,
        keybinds: [
            Some(Key::KeyC),
            Some(Key::KeyV),
            Some(Key::Kp1),
            Some(Key::Kp2),
            None,
            None,
            None,
            None,
            None,
            None
        ]
    })
}

/// Updates UserConfig.json from a UserConfig struct
pub fn write_user_config(user_config: &UserConfig) {
    todo!()
}