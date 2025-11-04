use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn sync(source: &str, target: &str) {
    //
    let source_hashes = read_paths_and_hashes(source);
    let target_hashes = read_paths_and_hashes(target);

    //
    let actions = determine_actions(&source_hashes, &target_hashes, source, target);

    //
    for (action, path) in actions {
        println!("Action: {}, Path: {:?}", action, path);
        match action {
            "copy" => {
                fs::copy(path.0.unwrap(), path.1.unwrap()).unwrap();
            }
            "move" => {
                fs::rename(path.0.unwrap(), path.1.unwrap()).unwrap();
            }
            "delete" => {
                fs::remove_file(path.1.unwrap()).unwrap();
            }
            _ => {
                panic!("Unknown action: {}", action);
            }
        }
    }
}

fn read_paths_and_hashes(dir: &str) -> HashMap<String, PathBuf> {
    let mut hashes = HashMap::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            let hash = calculate_hash(&path);
            hashes.insert(hash, path);
        }
    }
    hashes
}

fn calculate_hash(path: &Path) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    hasher
        .finalize()
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn determine_actions(
    source_hashes: &HashMap<String, PathBuf>,
    target_hashes: &HashMap<String, PathBuf>,
    source: &str,
    target: &str,
) -> Vec<(&'static str, (Option<PathBuf>, Option<PathBuf>))> {
    let mut actions = Vec::new();

    for (hash, path) in source_hashes {
        // If the file does not exist in the target, copy it
        if !target_hashes.contains_key(hash) {
            let sourcepath = Path::new(source).join(path.file_name().unwrap());
            let targetpath = Path::new(target).join(path.file_name().unwrap());
            actions.push(("copy", (Some(sourcepath), Some(targetpath))));
        } else {
            let target_path = target_hashes.get(hash).unwrap();
            // If the file exists but in a different location, move it
            if path.strip_prefix(source).unwrap() != target_path.strip_prefix(target).unwrap() {
                let sourcepath = Path::new(target).join(target_path.file_name().unwrap());
                let targetpath = Path::new(target).join(path.file_name().unwrap());
                actions.push(("move", (Some(sourcepath), Some(targetpath))));
            }
        }
    }

    for (hash, path) in target_hashes {
        // If the file does not exist in the source, delete it
        if !source_hashes.contains_key(hash) {
            let targetpath = Path::new(target).join(path.file_name().unwrap());
            actions.push(("delete", (None, Some(targetpath))));
        }
    }

    actions
}
