use pass_rhythm::config_services::read_rhythm_config;
use pass_rhythm::snap_ms_to_beat;

fn main() {
    //let chart_file = open_file("chart.qua", false);
    //let parsed_payload = assemble_master_key(chart_file);
    //let _ = update_master_key(&parsed_payload, String::new());
    println!("{:#?}", snap_ms_to_beat(read_rhythm_config(), 0965.0))
}
