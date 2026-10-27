#![feature(hash_map_macro)]

use std::collections::HashMap;
use std::hash_map;
use pass_rhythm::config_services::read_rhythm_config;
use pass_rhythm::{open_file, snap_ms_to_beat};
use pass_rhythm::chart_parsing::{import_chart_from_dir};

fn main() {
    //let rhythm_config = read_rhythm_config();
    let path_to_chart = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Quaver\\Songs\\1793118025000";
    let payload = import_chart_from_dir(path_to_chart);
    println!("{}", payload.master_key);
    //println!("{:#?}", );
    /*
    let chart_file = open_file("", false);
    let parsed_payload = assemble_master_key(&rhythm_config, chart_file);
    println!("{}", parsed_payload.master_key)
    */
}
