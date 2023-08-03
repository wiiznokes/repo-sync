use std::path::Path;

pub fn is_repo(path: &Path) -> bool {
    path.is_dir()
}
