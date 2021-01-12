use reqwest;
use serde::{de, Deserialize, Serialize};
use serde_json;

/// Client for making async requests to Mopidy API
/// This holds the reqwest client and constructs/parses API requests/responses
pub struct Client {
    pub api_url: String,
    pub client: reqwest::Client,
}

impl Client {
    /// Returns Mopdiy Client for making API calls with
    ///
    /// # Arguments
    ///
    /// * `server_addr` - String slice holding server address
    /// * `port` - Port Mopidy is listening on
    /// * `client` - Reqwest client that handles async HTTP requests
    pub fn new(server_addr: &str, port: u32, client: reqwest::Client) -> Client {
        Client {
            api_url: format!("{}:{}/mopidy/rpc", server_addr, port),
            client: client,
        }
    }
}

impl Client {
    /// Returns current playback state of Mopidy server
    pub async fn core_playback_get_state(&self) -> Result<String, String> {
        self.call_api::<String>("core.playback.get_state").await
    }
    /// Returns response to Mopidy API request
    /// <T>: data type expected in "result" field of API response (string or object depending on method)
    /// This folds in different error types from serde and reqwest calls so just pass strings through for any errors
    async fn call_api<T: de::DeserializeOwned>(&self, method: &str) -> Result<T, String> {
        match serde_json::from_str::<ApiResponse<T>>(
            &self
                .client
                .post(&self.api_url)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&MethodCall {
                        jsonrpc: "2.0".to_string(),
                        id: 1,
                        method: method.to_string(),
                    })
                    .map_err(|e| e.to_string())?,
                )
                .send()
                .await
                .map_err(|e| e.to_string())?
                .text()
                .await
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
        {
            Ok(response) => Ok(response.result),
            Err(error) => Err(error),
        }
    }
}

/// POST body json for requests to Mopidy API
#[derive(Serialize, Deserialize)]
struct MethodCall {
    jsonrpc: String,
    id: i32,
    method: String,
}

/// Json returned by Mopidy API
#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    jsonrpc: String,
    id: i32,
    result: T,
}
