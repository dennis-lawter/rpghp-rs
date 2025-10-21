use std::fs;
use std::path::Path;

use handlebars::Handlebars;

use crate::prelude::*;

/// Recursively registers every `.hbs` file in the given dir
/// Indexes share the file name (relative to the given dir) with the `.hbs` stripped
pub fn register_hbs_files_from_dir(
    hb: &mut Handlebars<'static>,
    dir: &str,
) -> CrateResult<()> {
    let base_path = Path::new(dir);
    register_recursive(hb, base_path, base_path)
}

fn register_recursive(
    hb: &mut Handlebars<'static>,
    base: &Path,
    current: &Path,
) -> CrateResult<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            register_recursive(hb, base, &path)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("hbs") {
            let rel = path.strip_prefix(base).map_err(|_| {
                CrateError::PathStripPrefixError(
                    path.display().to_string(),
                    base.display().to_string(),
                )
            })?;
            let name = rel
                .to_string_lossy()
                .trim_end_matches(".hbs")
                .replace('\\', "/");

            log::info!(
                "Registered a new HB template: {} => {}",
                name,
                path.display()
            );
            hb.register_template_file(&name, &path)?;
        } else {
            log::warn!(
                "Skipped file for HB registration (not .hbs file?): {}",
                path.display()
            );
        }
    }
    Ok(())
}
