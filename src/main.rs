use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::io::{self, Write};

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
    println!("{}", serde_json::to_string_pretty(&albums)?);
    Ok(())
}
