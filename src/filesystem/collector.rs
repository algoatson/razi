use std::path::Path;
use super::model;

pub fn read_directory(path: &Path) -> std::io::Result<Vec<model::FileEntry>> {
     let mut entries = Vec::new();

     for entry in std::fs::read_dir(path)? {
         let entry = entry?;
         let file_entry = model::FileEntry {
             name: entry.file_name().to_string_lossy().into_owned(),
             path: entry.path(),
             is_dir: entry.metadata()?.is_dir(),
         };
         entries.push(file_entry);
     }

     entries.sort_by_key(|e| (!e.is_dir, e.name.clone()));

     Ok(entries)
}