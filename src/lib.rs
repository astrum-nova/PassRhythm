pub mod config_services;
pub mod chart_parsing;
pub mod kdbx_services;
pub mod record_rhythm;

use std::fs::{File, OpenOptions};
use crate::config_services::{RhythmConfig, TimingPoint};

/// General file opening function
pub fn open_file(file_path: &str, truncate: bool) -> File {
    if let Ok(file) = OpenOptions::new().read(true).write(true).create(true).truncate(truncate).open(file_path) {
        file
    } else {
        panic!("Failed to open or create {file_path}")
    }
}

#[derive(Debug)]
pub struct KeyTiming {
    pub beat: i32,
    pub slot: i32
}
const MINUTE_FACTOR: f32 = 60000.0;
pub fn snap_ms_to_beat(rhythm_config: &RhythmConfig, ms: f32) -> KeyTiming {
    let mut current_timing_point = TimingPoint {
            start_time: 0.0,
            bpm: 0.0,
            time_signature: Some(4.0),
        };

    let mut beat_offset = 0.0;
    for timing_point in &rhythm_config.timing_points {
        if timing_point.start_time < ms {
            beat_offset += (timing_point.start_time - current_timing_point.start_time) * current_timing_point.bpm / MINUTE_FACTOR;
            current_timing_point.start_time = timing_point.start_time;
            current_timing_point.bpm = timing_point.bpm;
            if timing_point.time_signature != None {
                current_timing_point.time_signature = timing_point.time_signature;
            }
        }
    }
    let beat = beat_offset + (ms - current_timing_point.start_time) * current_timing_point.bpm / MINUTE_FACTOR;
    let mut floored_beat = beat.floor();
    let snapper = beat - floored_beat;
    let mut result = 0;
    let mut closest = f32::INFINITY;
    let time_signature = current_timing_point.time_signature.unwrap_or_else(|| unreachable!());
    for i in 0..=time_signature as i32 {
        let snap = i as f32 / time_signature;
        let distance = f32::abs(snap - snapper);
        if distance < closest {
            closest = distance;
            result = i;
        }
    }
    if result == time_signature as i32 {
        result = 0;
        floored_beat += 1.0;
    }
    KeyTiming {
        beat: floored_beat as i32,
        slot: result,
    }
}