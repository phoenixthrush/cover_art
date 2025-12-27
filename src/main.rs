use clap::Parser;
use reqwest::blocking::get;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "cover_art")]
#[command(about = "Download album cover art for a given artist")]
struct Cli {
    /// Artist name to search for
    artist: String,

    /// Output directory for artist folders (default: Artists)
    #[arg(short, long, default_value = "Artists")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let artist = if cli.artist.is_empty() {
        print!("Enter artist name: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    } else {
        cli.artist
    };

    let search_url = format!(
        "https://itunes.apple.com/search?term={}&entity=musicArtist&limit=1",
        urlencoding::encode(&artist)
    );
    let search_json: Value = get(&search_url)?.json()?;

    let artist_id = search_json["results"][0]["artistId"]
        .as_u64()
        .ok_or("NoArtistFound")?;
    let lookup_url = format!(
        "https://itunes.apple.com/lookup?id={}&entity=album",
        artist_id
    );
    let lookup_json: Value = get(&lookup_url)?.json()?;

    let prefix = "https://a1.mzstatic.com/us/r1000/0/";

    let mut albums = Vec::new();
    for mut item in lookup_json["results"]
        .as_array()
        .cloned()
        .unwrap_or_default()
    {
        // Filter albums by exact artist name match
        if let Some(album_artist) = item["artistName"].as_str() {
            if album_artist.to_lowercase() != artist.to_lowercase() {
                continue; // Skip albums where artist name doesn't match exactly
            }
        } else {
            continue; // Skip if no artist name found
        }

        if let Some(url) = item["artworkUrl100"].as_str() {
            let path = url.split("/image/thumb/").nth(1).unwrap_or("");
            let mut parts: Vec<&str> = path.split('/').collect();
            if parts.len() > 1 {
                parts.pop();
                let uncompressed = format!("{}{}", prefix, parts.join("/"));
                item["artworkUrl"] = Value::String(uncompressed);
            }
        }
        albums.push(item);
    }

    // Print JSON first
    println!("{}", serde_json::to_string_pretty(&albums)?);

    // Download album covers
    let album_count = albums
        .iter()
        .filter(|album| album["collectionName"].as_str().is_some())
        .count();
    let mut current_album = 1;

    for album in &albums {
        if let (Some(artist_name), Some(album_name), Some(artwork_url)) = (
            album["artistName"].as_str(),
            album["collectionName"].as_str(),
            album["artworkUrl"].as_str(),
        ) {
            // Create directory structure
            let album_dir = Path::new(&cli.output)
                .join(sanitize_filename(artist_name))
                .join(sanitize_filename(album_name));
            fs::create_dir_all(&album_dir)?;

            // Download cover image
            let cover_path = album_dir.join("cover.jpg");
            println!(
                "[{}/{}] Downloading cover for {} - {}",
                current_album, album_count, artist_name, album_name
            );

            let response = get(artwork_url)?;
            let bytes = response.bytes()?;
            fs::write(&cover_path, bytes)?;

            println!(
                "[{}/{}] Saved: {}",
                current_album,
                album_count,
                cover_path.display()
            );
            current_album += 1;
        }
    }
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}
