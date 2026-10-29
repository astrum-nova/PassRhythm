use crate::assemble_master_key;
use crate::chart_parsing::Note;
use crate::config_services::read_rhythm_config;

fn play_passrhythm() -> String {
    let rhythm_config = read_rhythm_config();
    let mut notes: Vec<Note> = vec![];

    //TODO: ASSEMBLE NOTES VEC FROM LISTENING, MATCH THEM TO AUDIO MS TIME

    assemble_master_key(&rhythm_config, &mut notes)
}