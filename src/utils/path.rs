use std::path::Path;

pub fn display_path(path: &Path) -> String {
    let home = std::env::var("HOME").ok();

    if let Some(home) = home {
        let home = Path::new(&home);

        if path == home {
            return "~".to_string();
        }

        if let Ok(stripped) = path.strip_prefix(home) {
            return format!("~/{}", stripped.display());
        }
    }

    path.display().to_string()
}