use pass_rhythm::open_file;
use crate::rhythm::chart_parsing::get_master_key_from_chart;

mod config_services;
mod rhythm;
mod kdbx_services;

fn main() {
    let chart_file = open_file("chart.qua");
    let master_key = get_master_key_from_chart(chart_file);
    println!("{master_key}")
}
