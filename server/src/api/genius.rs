use serde_json::{Value};
use reqwest;

use super::errors;

// get_details returns the title and artist of a song lyric
pub fn get_details(lyric: String) -> Result<(String, String, String), errors::NoSongError> {
    let songs: Value = match get_songs(lyric) {
        Ok(songs) => songs,
        Err(_e) => return Err(errors::NoSongError),
    };
    parse_details(songs)
}

// parse_details takes the genius response json of a search
// request and extracts the artist and title of the song.
fn parse_details(songs: Value) -> Result<(String, String, String), errors::NoSongError> {
    // Extract the useful data from the response
    if songs["meta"]["status"] == 200 {
        if songs["response"]["hits"].is_array() {
            for hit in songs["response"]["hits"].as_array().unwrap() {
                if hit["type"] == "song" {
                    let song_data = ( 
                        hit["result"]["title"].as_str().unwrap().to_string(), 
                        hit["result"]["primary_artist"]["name"].as_str().unwrap().to_string(),
                        hit["result"]["url"].as_str().unwrap().to_string()
                    );
                    return Ok(song_data);
                }
            }
        }
    }
    Err(errors::NoSongError)
}

// get_song queries the genius API for the song that the lyric
// is a part of.
fn get_songs(lyric: String) -> Result<Value, reqwest::Error> {
    // The auth_token needed to complete the request
    let token = "AnkBFMxFU4x5-ucmG8ze4ctZzHIF9cCzlOg9kMLBpO2I2ogcdRTcSGqlZ9jS9GmS  ";
    
    let req_url = format!(
        "https://api.genius.com/search?q={query}",
        query = lyric,
    );
    
    // Create a client to send the request
    let client = reqwest::blocking::Client::new();
    let result: Value = client.get(&req_url)
        .bearer_auth(token)
        .send()?.json()?;

    Ok(result)
}