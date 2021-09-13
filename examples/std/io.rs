use anyhow::Result;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let key = "PATH";
    let mut path_vec = Vec::new();
    if let Some(paths) = env::var_os(key) {
        for path in env::split_paths(&paths) {
            if path.is_dir() {
                path_vec.push(path);
            }
        }
    }
    if let Ok(cur_dir) = env::current_dir() {
        path_vec.push(cur_dir);
    }
    let mut file_vec = Vec::new();
    for p in path_vec {
        walk_dirs(&mut file_vec, &p);
    }
    for p in file_vec {
        println!("{}", p.display());
    }

    Ok(())
}

fn walk_dirs(file_vec: &mut Vec<PathBuf>, path: &PathBuf) {
    let dir = match fs::read_dir(path) {
        Ok(d) => d,
        Err(err) => {
            println!("walk {} error with {:?}", path.display(), err);
            return;
        }
    };
    for dir_entry in dir {
        let file_entry = match dir_entry {
            Ok(f) => f,
            Err(err) => {
                println!("walk {} error with {:?}", path.display(), err);
                return;
            }
        };
        let file = file_entry.path();
        if file.is_dir() {
            walk_dirs(file_vec, &file);
        } else {
            file_vec.push(file);
        }
    }
}
