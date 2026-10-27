use crate::chart_parsing::ParsedPayload;
use crate::config_services::write_rhythm_config;
use keepass::{Database, DatabaseKey};
use std::{fs, fs::File, path::Path};

const PATH_TO_VAULT: &str = "vault.kdbx";

/// Ok(None): no vault yet. Err: vault exists but couldn't be opened (wrong password, corrupt, I/O).
fn open_vault(password: &str) -> Result<Option<Database>, String> {
    if !Path::new(PATH_TO_VAULT).exists() {
        return Ok(None);
    }
    let mut file = match File::open(PATH_TO_VAULT) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read {PATH_TO_VAULT}: {err}")),
    };
    let key = DatabaseKey::new().with_password(password);
    match Database::open(&mut file, key) {
        Ok(db) => Ok(Some(db)),
        Err(err) => Err(format!("Failed to open {PATH_TO_VAULT} (wrong password or corrupt file): {err}")),
    }
}

pub fn update_master_key(parsed_payload: &ParsedPayload, old_key: String) -> Result<(), String> {
    let vault_exists = Path::new(PATH_TO_VAULT).exists();
    let db = match open_vault(&old_key) {
        Ok(Some(db)) => db,
        Ok(None) => Database::new(),
        Err(err) => return Err(err),
    };
    let tmp_path = format!("{PATH_TO_VAULT}.tmp");
    let bak_path = format!("{PATH_TO_VAULT}.bak");
    {
        let mut tmp_file = match File::create(&tmp_path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Couldn't create temporary file: {err}")),
        };
        let new_key = DatabaseKey::new().with_password(&parsed_payload.master_key);
        if let Err(err) = db.save(&mut tmp_file, new_key) {
            let _ = fs::remove_file(&tmp_path);
            return Err(format!("Couldn't save temporary vault: {err}"));
        }
        if let Err(err) = tmp_file.sync_all() {
            let _ = fs::remove_file(&tmp_path);
            return Err(format!("Couldn't flush temporary vault to disk: {err}"));
        }
    }
    if !match File::open(&tmp_path) {
        Ok(mut file) => Database::open(&mut file, DatabaseKey::new().with_password(&parsed_payload.master_key)).is_ok(),
        Err(_) => false,
    } {
        let _ = fs::remove_file(&tmp_path);
        return Err("Temporary vault failed verification, nothing was changed".to_string());
    }
    if vault_exists && let Err(err) = fs::copy(PATH_TO_VAULT, &bak_path) {
        let _ = fs::remove_file(&tmp_path);
        return Err(format!("Couldn't back up the current vault, nothing was changed: {err}"));
    }
    if let Err(err) = fs::rename(&tmp_path, PATH_TO_VAULT) {
        let _ = fs::remove_file(&tmp_path);
        return Err(format!("Couldn't replace the vault, nothing was changed: {err}"));
    }
    write_rhythm_config(&parsed_payload.rhythm_config);
    Ok(())
}

fn get_db_entries() {

}

fn get_entry_data() {

}