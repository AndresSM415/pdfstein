use std::fs;

pub fn ensure_output_dir_exists(output_dir: &str) -> Result<(), String> {
    if !fs::exists(output_dir).unwrap_or(false) {
        fs::create_dir_all(output_dir).map_err(|e| e.to_string())?
    }
    Ok(())
}
