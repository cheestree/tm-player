use std::fs;

pub fn get_audio_file_paths_in_directory(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Err(e) = fs::metadata(path) {
        eprintln!("Error accessing directory {}: {}", path, e);
        return files;
    }
    get_files_recursively(path, &mut files);
    files
}

fn get_files_recursively(path: &str, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_dir() {
                    if let Some(dir_str) = file_path.to_str() {
                        get_files_recursively(dir_str, files);
                    }
                } else if let Some(file_str) = file_path.to_str() {
                    if is_audio_file(file_str) {
                        files.push(file_str.to_string());
                    }
                }
            }
        }
    }
}

fn is_audio_file(file_name: &str) -> bool {
    file_name.ends_with(".mp3")
        || file_name.ends_with(".wav")
        || file_name.ends_with(".flac")
        || file_name.ends_with(".aac")
}
