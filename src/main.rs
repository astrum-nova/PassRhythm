#![feature(hash_map_macro)]

use std::collections::HashMap;
use std::hash_map;
use pass_rhythm::config_services::read_rhythm_config;
use pass_rhythm::snap_ms_to_beat;

fn main() {
    //let chart_file = open_file("chart.qua", false);
    //let parsed_payload = assemble_master_key(chart_file);
    //let _ = update_master_key(&parsed_payload, String::new());
    let rc = read_rhythm_config();
}
