use crate::config_services::{RhythmConfig, TimingPoint, write_rhythm_config};
use std::fs::File;
use std::io::{BufRead, BufReader};
use zeroize::Zeroize;

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
    pub key: String
}

/// Extracts necessary data from a chart file, and writes to rhythm config.
/// > Side effect: writes to RhythmConfig.json
fn parse_chart_file(chart_file: File) -> ParsedPayload {
    let mut rhythm_config = RhythmConfig {
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
                "AudioFile" => /*TODO*/(),
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
        key: String::new()  
    }
}

/// Parses the numeric value of the chart data
fn parse_value(value: &str) -> f32 {
    match value.trim().parse::<f32>() {
        Ok(num) => num,
        Err(err) => panic!("Failed parsing value in chart file for line: {value}\n{err}")
    }
}

/// Assembles the notes collected from the chart data into the master key format
pub fn assemble_master_key(chart_file: File) -> ParsedPayload {
    let mut payload = parse_chart_file(chart_file);
    let notes = &mut payload.notes;
    // Remove possible stub
    if notes[0].lanes.is_empty() {
        notes.remove(0);
    }
    let mut key: String = String::new();
    for note in notes {
        key += note.start_time.to_string().as_str();
        key += ":";
        note.lanes.sort();
        for lane in &note.lanes {
            key += lane.to_string().as_str();
            key += ",";
        }
        key.remove(key.len() - 1);
        key += "|";
    }
    key.remove(key.len() - 1);
    Zeroize::zeroize(&mut payload.notes);
    payload.key = key;
    payload
}