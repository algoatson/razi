use std::path::PathBuf;

pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

pub struct FilePreview {
    pub path: PathBuf,
    pub content: String,
}

pub enum Preview {
    None,
    Directory(Vec<FileEntry>),
    File(FilePreview),
}
