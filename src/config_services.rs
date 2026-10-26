use rdev::Key;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use crate::open_file;

/// Holds information for a timing change in the song
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingPoint {
    pub start_time: f32,
    pub bpm: f32,
    pub time_signature: Option<f32>
}

/// Struct containing the current passrhythm information
/// first_input is the time of the song where the first input happens, needed for offset settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmConfig {
    pub first_input: Option<f32>,
    pub timing_points: Vec<TimingPoint>
}

/// Reads RhythmConfig.json and returns a RhythmConfig struct
pub fn read_rhythm_config() -> RhythmConfig {
    if let Ok(user_config) = serde_json::from_reader::<File, RhythmConfig>(open_file("RhythmConfig.json", false)) {
        user_config
    } else {
        //TODO: Add backups or a way to attempt recovery if possible
        panic!("Couldn't read your passrhythm configuration");
    }
}

/// Updates RhythmConfig.json from a RhythmConfig struct, only confirm chart file deletion if result is Ok
pub fn write_rhythm_config(config: &RhythmConfig) {
    if let Err(_) = serde_json::to_writer_pretty(BufWriter::new(open_file("RhythmConfig.json", true)), &config) {
        panic!("Failed to write RhythmConfig to file, try manually updating the file, refreshing the vault backup")
    }
}

/// Struct containing user configs, filled by reading from UserConfig.json, and used to write to UserConfig.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub offset: i32,
    pub keybinds: [Key; 10]
}

/// Reads UserConfig.json and returns a UserConfig struct
pub fn read_user_config() -> UserConfig {
    if let Ok(user_config) = serde_json::from_reader::<File, UserConfig>(open_file("UserConfig.json", false)) {
        user_config
    } else {
        println!("Failed to deserialize user config!\nDefault settings will be used: offset = 0, keybinds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]\nMake sure you bind all 10 lanes!");
        UserConfig {
            offset: 0,
            keybinds: [
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Num4,
                Key::Num5,
                Key::Num6,
                Key::Num7,
                Key::Num8,
                Key::Num9,
                Key::Num0
            ],
        }
    }
}

/// Updates UserConfig.json from a UserConfig struct
pub fn write_user_config(config: UserConfig) {
    if let Err(_) = serde_json::to_writer_pretty(BufWriter::new(open_file("UserConfig.json", true)), &config) {
        println!("Could not save user config, will live in memory for now");
    }
}
