# cover_art

Fetches high-resolution album artwork from Apple Music by searching for an artist and downloading all their albums in maximum available quality

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

## License

This project is released into the public domain under the Unlicense. See the LICENSE file for details.
