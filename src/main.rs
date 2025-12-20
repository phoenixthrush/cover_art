use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artist = env::args().nth(1).unwrap_or_else(|| {
        print!("Enter artist name: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    });

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

    // Download album covers
    for album in &albums {
        if let (Some(artist_name), Some(album_name), Some(artwork_url)) = (
            album["artistName"].as_str(),
            album["collectionName"].as_str(),
            album["artworkUrl"].as_str(),
        ) {
            // Create directory structure
            let album_dir = Path::new("Artists")
                .join(sanitize_filename(artist_name))
                .join(sanitize_filename(album_name));
            fs::create_dir_all(&album_dir)?;

            // Download cover image
            let cover_path = album_dir.join("cover.jpg");
            println!("Downloading cover for {} - {}", artist_name, album_name);

            let response = get(artwork_url)?;
            let bytes = response.bytes()?;
            fs::write(&cover_path, bytes)?;

            println!("Saved: {}", cover_path.display());
        }
    }

    println!("{}", serde_json::to_string_pretty(&albums)?);
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
