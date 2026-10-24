use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::config_services::{write_rhythm_config, RhythmConfig, TimeSignature};

#[derive(Debug)]
/// Holds note lane, start time, and end time for a note
struct Note {
    //.0: note on lane, .1: release on lane
    lanes: Vec<(i32, Option<i32>)>,
    start_time: i32,
}

/// Extracts necessary data from a chart file, and writes to rhythm config.
/// > Side effect: writes to RhythmConfig.json
fn parse_chart_file(chart_file: File) -> Vec<Note> {
    let mut rhythm_config = RhythmConfig {
        first_input: -1,
        bpm: -1,
        time_signature: TimeSignature {
            numerator: -1,
            denominator: -1
        },
    };
    let mut last_start_time = -1;
    let mut notes: Vec<Note> = vec![];
    for line_result in BufReader::new(chart_file).lines() {
        if let Ok(line) = line_result {
            let line_data: Vec<&str> = line.split(":").collect();
            match line_data[0].replace("-", "").replace(" ", "").as_str() {
                "AudioFile" => todo!("BRING AUDIO FILE HERE"),
                "StartTime" => {
                    let value = parse_value(line_data[1]);
                    last_start_time = value;
                    if rhythm_config.bpm == -1 {
                        rhythm_config.first_input = value;
                        continue;
                    }
                    if let Some(note) = notes.last_mut() && note.start_time != value {
                        notes.push(Note {
                            lanes: vec![],
                            start_time: value,
                        })
                    }
                },
                "Lane" => if let Some(note) = notes.last_mut() && note.start_time == last_start_time {
                    note.lanes.push((parse_value(line_data[1]), None))
                },
                "EndTime" => if let Some(note) = notes.last_mut() && let Some(lane) = note.lanes.last_mut() {
                    lane.1 = Some(parse_value(line_data[1]));
                },
                "Bpm" => rhythm_config.bpm = parse_value(line_data[1]),
                _ignored => println!("Line {_ignored} is ignored")
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
fn parse_value(value: &str) -> i32 {
    match value.trim().parse::<i32>() {
        Ok(num) => num,
        Err(err) => panic!("Failed parsing value in chart file for line: {value}\n{err}")
    }
}

/// Assembles the notes collected from the chart data into the master key format
fn assemble_master_key(notes: Vec<Note>) -> String {
    for mut note in notes {
        note.lanes.sort();
    }
    todo!()
}

/// Glues together the functions of the chart parsing module
pub fn get_master_key_from_chart(chart_file: File) -> String {
    assemble_master_key(parse_chart_file(chart_file))
}