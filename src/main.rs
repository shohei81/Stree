use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

const TOP_K: usize = 5;

#[derive(Debug)]
struct Statistics {
    files: usize,
    dirs: usize,
    total_size: u64,
    max_file: Option<(String, u64)>,
    errors: Vec<String>,
}

impl Statistics {
    fn new() -> Self {
        Statistics {
            files: 0,
            dirs: 0,
            total_size: 0,
            max_file: None,
            errors: Vec::new(),
        }
    }

    fn record_file(&mut self, name: String, size: u64) {
        self.files += 1;
        self.total_size += size;

        // 最大ファイルを追跡
        match &self.max_file {
            None => {
                self.max_file = Some((name, size));
            }
            Some((_, max_size)) => {
                if size > *max_size {
                    self.max_file = Some((name, size));
                }
            }
        }
    }

    fn record_dir(&mut self) {
        self.dirs += 1;
    }

    fn record_error(&mut self, error: String) {
        self.errors.push(error);
    }

    fn print_summary(&self) {
        println!("\n📊 Statistics:");
        println!("   - Files: {}", self.files);
        println!("   - Directories: {}", self.dirs);
        println!("   - Total size: {}", format_size(self.total_size));

        if self.files > 0 {
            let avg_size = self.total_size / self.files as u64;
            println!("   - Average file size: {}", format_size(avg_size));
        }

        if let Some((name, size)) = &self.max_file {
            println!("   - Largest file: {} ({})", name, format_size(*size));
        }

        if !self.errors.is_empty() {
            println!("\n⚠️  Warnings:");
            for error in &self.errors {
                println!("   - {}", error);
            }
        }
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    const THRESHOLD: f64 = 1024.0;

    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= THRESHOLD && unit_idx < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        // バイト単位の場合は整数表示
        format!("{} {}", bytes, UNITS[0])
    } else {
        // それ以上の単位は小数第1位まで
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_dir = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let root = PathBuf::from(target_dir);

    println!("{}", root.display());

    let mut stats = Statistics::new();
    print_tree(&root, "", &mut stats);
    
    stats.print_summary();
}

fn print_tree(path: &Path, prefix: &str, stats: &mut Statistics) {
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => {
            let error_msg = format!("Cannot read directory {:?}: {}", path, e);
            eprintln!("{}", error_msg);
            stats.record_error(error_msg);
            return;
        }
    };

    let mut entries: Vec<fs::DirEntry> = read_dir.flatten().collect();

    entries.sort_by_key(|e| e.file_name());

    let total = entries.len();
    if total == 0 {
        return;
    }

    let show = std::cmp::min(total, TOP_K);
    let has_more = total > show;

    for (idx, entry) in entries.into_iter().take(show).enumerate() {
        let path = entry.path();

        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<invalid utf-8>");

        let is_last = if has_more {
            false
        } else {
            idx + 1 == show
        };

        let branch = if is_last { "└── " } else { "├── " };

        // メタデータを取得して統計情報を記録
        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    println!("{prefix}{branch}{}/", name);
                    stats.record_dir();

                    let child_prefix = if is_last {
                        format!("{prefix}    ")
                    } else {
                        format!("{prefix}│   ")
                    };

                    print_tree(&path, &child_prefix, stats);
                } else {
                    let size = metadata.len();
                    println!("{prefix}{branch}{}", name);
                    stats.record_file(name.to_string(), size);
                }
            }
            Err(e) => {
                let error_msg = format!("Cannot read metadata for {:?}: {}", path, e);
                eprintln!("{}", error_msg);
                stats.record_error(error_msg);
                println!("{prefix}{branch}{}", name);
            }
        }
    }

    if has_more {
        let remaining = total - show;
        println!("{prefix}└── ... (+{} more)", remaining);
    }
}
