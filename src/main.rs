use pass_rhythm::open_file;
use crate::kdbx_services::update_master_key;
use crate::rhythm::chart_parsing::{assemble_master_key};

mod config_services;
mod rhythm;
mod kdbx_services;

fn main() {
    let chart_file = open_file("chart.qua", false);
    let parsed_payload = assemble_master_key(chart_file);
    let _ = update_master_key(&parsed_payload, String::new());
}
