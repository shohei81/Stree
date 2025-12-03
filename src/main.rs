use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_dir = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let root = PathBuf::from(target_dir);

    println!("{}", root.display());

    print_tree(&root, "");
}

fn print_tree(path: &Path, prefix: &str) {
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("Cannot read directory {:?}: {}", path, e);
            return;
        }
    };

    let mut entries: Vec<fs::DirEntry> = read_dir.flatten().collect();

    entries.sort_by_key(|e| e.file_name());

    let len = entries.len();

    for (idx, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        let is_last = idx + 1 == len;

        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<invalid utf-8>");

        let branch = if is_last { "└── " } else { "├── " };

        if path.is_dir() {
            println!("{prefix}{branch}{}/", name);

            let child_prefix = if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };

            print_tree(&path, &child_prefix);
        } else {
            println!("{prefix}{branch}{name}");
        }
    }
}