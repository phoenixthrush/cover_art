# cover_art
Fetches Apple Music album artwork in maximum available resolution

## Usage

```bash
# Basic usage (creates Artists folder)
cargo run --release "Artist Name"

# Custom output directory
cargo run --release "Artist Name" --output "Music"
cargo run --release "Artist Name" -o "Music"
```

## Arguments

- `<ARTIST>`: Artist name to search for (required)
- `-o, --output <FOLDER>`: Output directory for artist folders (default: Artists)
