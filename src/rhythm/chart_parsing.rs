use crate::config_services::{RhythmConfig, TimingPoint, write_rhythm_config};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
/// Holds note lane, start time, and end time for a note
struct Note {
    //.0: note on lane, .1: release on lane
    lanes: Vec<(i32, Option<f32>)>,
    start_time: f32,
}

/// Extracts necessary data from a chart file, and writes to rhythm config.
/// > Side effect: writes to RhythmConfig.json
fn parse_chart_file(chart_file: File) -> Vec<Note> {
    let mut rhythm_config = RhythmConfig {
        first_input: None,
        timing_points: vec![TimingPoint {
            start_time: 0f32,
            bpm: 0f32,
            time_signature: 0f32,
        }]
    };
    let mut hit_objects = false;
    let mut notes: Vec<Note> = vec![];
    for line_result in BufReader::new(chart_file).lines() {
        if let Ok(line) = line_result {
            let line_data: Vec<&str> = line.split(":").collect();
            match line_data[0].replace("-", "").replace(" ", "").as_str() {
                "AudioFile" => /*TODO*/(),
                "StartTime" => {
                    let value = parse_value(line_data[1]);
                    if !hit_objects {
                        rhythm_config.first_input = Some(value);
                        rhythm_config.timing_points.push(TimingPoint {
                            start_time: value,
                            bpm: 0f32,
                            time_signature: 0f32,
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
                "HitObjects" => hit_objects = true,
                "Lane" => if let Some(note) = notes.last_mut() {
                    note.lanes.push((parse_value(line_data[1]) as i32, None))
                },
                "EndTime" => if let Some(note) = notes.last_mut() && let Some(lane) = note.lanes.last_mut() {
                    lane.1 = Some(parse_value(line_data[1]));
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
                    timing_point.time_signature = parse_value(line_data[1]);
                },
                _ => (),
            }
        }
    }
    //TODO: HANDLE ERROR
    if let Err(err) = write_rhythm_config(rhythm_config) {
        panic!("{err}")
    }
    notes
}

/// Parses the numeric value of the chart data
fn parse_value(value: &str) -> f32 {
    match value.trim().parse::<f32>() {
        Ok(num) => num,
        Err(err) => panic!("Failed parsing value in chart file for line: {value}\n{err}")
    }
}

/// Assembles the notes collected from the chart data into the master key format
fn assemble_master_key(mut notes: Vec<Note>) -> String {
    if notes[0].lanes.is_empty() {
        notes.remove(0);
    }
    for note in &mut notes {
        note.lanes.sort_by(|lane_a, lane_b| lane_a.0.cmp(&lane_b.0));
    }
    println!("{notes:#?}");
    //TODO
    String::new()
}

/// Glues together the functions of the chart parsing module
pub fn get_master_key_from_chart(chart_file: File) -> String {
    assemble_master_key(parse_chart_file(chart_file))
}