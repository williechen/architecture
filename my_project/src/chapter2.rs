use sha2::{Digest, Sha256};
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

fn read_paths_and_hashes(dir: &str) -> Vec<(PathBuf, String)> {
    let mut file_list = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            let hash = calculate_hash(&path);
            file_list.push((path, hash));
        }
    }
    file_list
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
    source_hashes: &Vec<(PathBuf, String)>,
    target_hashes: &Vec<(PathBuf, String)>,
    source: &str,
    target: &str,
) -> Vec<(&'static str, (Option<PathBuf>, Option<PathBuf>))> {
    let mut actions = Vec::new();

    for (path, hash) in source_hashes {
        if !target_hashes
            .iter()
            .any(|(p, _h)| p.file_name() == path.file_name())
        {
            // If the file does not exist in the target, copy it
            let sourcepath = Path::new(source).join(path.file_name().unwrap());
            let targetpath = Path::new(target).join(path.file_name().unwrap());
            actions.push(("copy", (Some(sourcepath), Some(targetpath))));
        } else if target_hashes.iter().any(|(_p, h)| h == hash) {
            // If the file exists in the target but with a different name, move it
            let target_path = target_hashes
                .iter()
                .find(|(_p, h)| h == hash)
                .unwrap()
                .0
                .clone();

            let sourcepath = Path::new(target).join(target_path.file_name().unwrap());
            let targetpath = Path::new(target).join(path.file_name().unwrap());
            actions.push(("move", (Some(sourcepath), Some(targetpath))));
        }
    }

    for (path, _hash) in target_hashes {
        // If the file does not exist in the source, delete it
        if !source_hashes
            .iter()
            .any(|(p, _h)| p.file_name() == path.file_name())
        {
            actions.push(("delete", (None, Some(path.clone()))));
        }
    }

    actions
}
