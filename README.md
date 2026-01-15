# Stree

**S-tree: Summary Statistics Tree** - Fast, informative directory tree viewer written in Rust.

## Overview

Stree is a modern alternative to the Unix `tree` command, built in Rust for speed and efficiency. The unique "S" stands for **Statistics** - it displays your directory structure while providing insightful summary information about files, sizes, and directory composition.

## Features

- 🌳 **Directory tree visualization** - Classic tree-style directory structure display
- 📊 **Built-in statistics** - Automatic collection and display of:
  - Total file count
  - Total directory count
  - Cumulative file size (with smart unit conversion: B/KB/MB/GB)
  - Average file size
  - Largest file in the tree
- ⚡ **Fast performance** - Leverages Rust's speed and efficiency
- 🚀 **Smart defaults** - Limits initial display to 5 items per directory (TOP_K), preventing overwhelming output on large directories
- ⚠️ **Error handling** - Gracefully handles permission errors and inaccessible directories

## Installation

### From source

```bash
git clone https://github.com/shohei81/Stree.git
cd Stree
cargo build --release
./target/release/stree [directory]
```

### Add to PATH

```bash
# Copy the binary to a location in your PATH
cp target/release/stree /usr/local/bin/
# Now you can use 'stree' from anywhere
```

## Usage

```bash
# Display current directory
stree

# Display specific directory
stree /path/to/directory

# Display home directory
stree ~
```

## Example Output

```
.
├── src/
│   └── main.rs
├── target/
├── Cargo.toml
├── LICENSE
└── ... (+3 more)

📊 Statistics:
   - Files: 9
   - Directories: 1
   - Total size: 2.6 KB
   - Average file size: 298 B
   - Largest file: LICENSE (1.0 KB)
```

## Design Philosophy

Stree embodies **ruthless simplicity**:

- **One job, done well** - Display directory structure + summary statistics
- **Smart limitations** - TOP_K=5 default prevents overwhelming output
- **Zero unnecessary features** - No interactive mode, no git integration (yet)
- **Fast enough** - Compiled Rust for instant results

## Implementation Highlights

### Statistics Collection
Statistics are collected during the single tree traversal pass, ensuring:
- Zero performance overhead
- Consistent view of the directory
- All files counted, even those not displayed (due to TOP_K limit)

### Error Handling
Permission errors and inaccessible directories are:
- Logged during traversal
- Reported in a warning section
- Never crash the entire operation

### File Size Display
Smart unit conversion automatically chooses the best representation:
- `< 1 KB`: Displayed in bytes
- `< 1 MB`: Displayed in KB
- `< 1 GB`: Displayed in MB
- `>= 1 GB`: Displayed in GB

## Roadmap

Future enhancements to consider:
- [ ] Filter by file pattern (`-f "*.rs"`)
- [ ] Size-aware sorting (`-s` flag)
- [ ] Git integration (`-g` flag for git status)
- [ ] Colored output based on file type
- [ ] Configurable depth limit
- [ ] Export to JSON/CSV

## Comparison with Alternatives

| Feature | tree | lstr | broot | **Stree** |
|---------|------|------|-------|-----------|
| Tree display | ✅ | ✅ | ✅ | ✅ |
| File size info | ❌ | ✅ | ⭕ | ✅ |
| **Statistics summary** | ❌ | ❌ | ❌ | ✅ |
| Interactive mode | ❌ | ❌ | ✅ | ❌ |
| High speed | ⭕ | ✅ | ⭕ | ✅ |

## License

MIT - See LICENSE file for details

## Contributing

Contributions welcome! Feel free to open issues or submit PRs.
