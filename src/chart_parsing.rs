use std::fs;
use crate::config_services::{RhythmConfig, TimingPoint, write_rhythm_config};
use std::fs::{File, ReadDir};
use std::io::{BufRead, BufReader};
use zeroize::Zeroize;
use crate::{open_file, snap_ms_to_beat};

#[derive(Debug, Zeroize)]
/// Holds note lane, start time, and end time for a note
pub struct Note {
    pub lanes: Vec<i32>,
    pub start_time: f32,
}

#[derive(Debug)]
pub struct ParsedPayload {
    pub notes: Vec<Note>,
    pub rhythm_config: RhythmConfig,
    pub master_key: String
}

/// Extracts necessary data from a chart file, and writes to rhythm config.
/// > Side effect: writes to RhythmConfig.json
fn parse_chart_file(chart_file: File) -> ParsedPayload {
    let mut rhythm_config = RhythmConfig {
        audio_file_name: None,
        first_input: None,
        timing_points: vec![TimingPoint {
            start_time: 0f32,
            bpm: 0f32,
            time_signature: Some(4f32),
        }]
    };
    let mut hit_objects = false;
    let mut slider_velocities = false;
    let mut notes: Vec<Note> = vec![];
    for line_result in BufReader::new(chart_file).lines() {
        if let Ok(line) = line_result {
            let line_data: Vec<&str> = line.split(":").collect();
            match line_data[0].replace("-", "").replace(" ", "").as_str() {
                "AudioFile" => rhythm_config.audio_file_name = Some(line_data[1].replace(" ", "")),
                "StartTime" => {
                    let value = parse_value(line_data[1]);
                    if !hit_objects && !slider_velocities {
                        rhythm_config.first_input = Some(value);
                        rhythm_config.timing_points.push(TimingPoint {
                            start_time: value,
                            bpm: 0f32,
                            time_signature: Some(4f32),
                        });
                        continue;
                    }
                    if let Some(note) = notes.last_mut() && note.start_time != value {
                        notes.push(Note {
                            lanes: vec![],
                            start_time: value,
                        });
                    }
                },
                "SliderVelocities" => slider_velocities = true,
                "HitObjects" => hit_objects = true,
                "Lane" => if let Some(note) = notes.last_mut() {
                    note.lanes.push(parse_value(line_data[1]) as i32)
                },
                "Bpm" => {
                    if let Some(timing_point) = rhythm_config.timing_points.last_mut() {
                        timing_point.bpm = parse_value(line_data[1]);
                    }
                    if notes.is_empty() {
                        notes.push(Note {
                            lanes: vec![],
                            start_time: rhythm_config.timing_points[0].start_time,
                        });
                    }
                },
                "Signature" => if let Some(timing_point) = rhythm_config.timing_points.last_mut() {
                    timing_point.time_signature = Some(parse_value(line_data[1]));
                },
                _ => (),
            }
        }
    }
    //TODO: HANDLE ERROR
    write_rhythm_config(&rhythm_config);
    ParsedPayload {
        notes,
        rhythm_config,
        master_key: String::new()
    }
}

/// Parses the numeric value of the chart data
fn parse_value(value: &str) -> f32 {
    match value.trim().parse::<f32>() {
        Ok(num) => num,
        Err(err) => panic!("Failed parsing value in chart file for line: {value}\n{err}")
    }
}

fn get_chart_file(path_to_chart: &str) -> File {
    let chart_dir: ReadDir = match std::fs::read_dir(path_to_chart) {
        Ok(read_dir) => read_dir,
        Err(_) => panic!("Failed to read chart directory")
    };
    for entry in chart_dir {
        if let Ok(entry) = entry && let Some(file_name) = entry.file_name().to_str() {
            if file_name.ends_with(".qua") {
                return open_file(format!("{path_to_chart}\\{file_name}").as_str(), false);
            }
        }
    }
    panic!("Could not find chart file");
}

/// Assembles the notes collected from the chart data into the master key format
pub fn import_chart_from_dir(path_to_chart: &str) -> ParsedPayload {
    let mut payload = parse_chart_file(get_chart_file(path_to_chart));
    if let Some(audio_file_name) = &payload.rhythm_config.audio_file_name {
        match fs::copy(format!("{path_to_chart}\\{audio_file_name}"), audio_file_name) {
            Ok(_) => println!("Successfully imported audio file"),
            Err(err) => println!("Failed to copy audio file at: {path_to_chart}\\{audio_file_name}\n{err}")
        }
    }
    let notes = &mut payload.notes;
    // Remove possible stub
    if notes[0].lanes.is_empty() {
        notes.remove(0);
    }
    let mut master_key: String = String::new();
    for note in notes {
        let key_timing = snap_ms_to_beat(&payload.rhythm_config, note.start_time);
        master_key += &format!("{}.{}", key_timing.beat, key_timing.slot);
        master_key += ":";
        note.lanes.sort();
        for lane in &note.lanes {
            master_key += lane.to_string().as_str();
            master_key += ",";
        }
        master_key.remove(master_key.len() - 1);
        master_key += "|";
    }
    master_key.remove(master_key.len() - 1);
    Zeroize::zeroize(&mut payload.notes);
    payload.master_key = master_key;
    payload
}