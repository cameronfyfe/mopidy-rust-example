//! mopidy_json_rpc_example
//! author: Cameron Fyfe, cameron.j.fyfe@gmail.com
//! Jan 11 2021
//!
//! This code is an example project for querying playback state from a Mopidy server.
//! (code for dealing with API in src/mopidy.rs)
//!
//! This program queries the Mopidy server at 'MOPIDY_ADDR' and displays the result.

use reqwest;
mod mopidy;


static MOPIDY_ADDR: &str = "http://localhost";
static MOPIDY_PORT: u32 = 6680;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mopidy = mopidy::Client::new(MOPIDY_ADDR, MOPIDY_PORT, reqwest::Client::new());

    println!("Making Mopidy RPC request to [{}]...", mopidy.api_url);
    match mopidy.core_playback_get_state().await {
        Ok(result) => {
            println!("Done.");
            println!("Playback State: {}", result);
        }
        Err(err_str) => {
            println!("Failed.");
            println!("Error: {}", err_str);
        }
    }

    Ok(())
}
