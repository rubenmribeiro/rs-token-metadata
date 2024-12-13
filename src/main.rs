use std::error::Error;
use serde::{Deserialize, Serialize};
use redis::{Client, Commands};

#[derive(Debug, Serialize, Deserialize)]
struct TokenInfo {
    name: String,
    symbol: String,
    decimals: String,
}

trait TokenClient {
    fn get_token_info(&self, token_address: &str) -> Result<TokenInfo, Box<dyn Error>>;
}

struct RedisClient {
    client: Client,
}

impl RedisClient {
    fn new(redis_url: String) -> Self {
        let client = Client::open(redis_url).expect("Failed to connect to Redis");
        Self { client }
    }

    fn get_cached(&self, key: &str) -> Option<String> {
        if let Ok(mut con) = self.client.get_connection() {
            return con.get(key).ok();
        }
        None
    }

    fn set_cached(&self, key: &str, value: &str, expiry: u32) -> bool {
        if let Ok(mut con) = self.client.get_connection() {
            return con.set_ex(key, value, expiry as u64).unwrap_or(false);
        }
        false
    }
}

struct MoralisClient {
    api_key: String,
    redis: Option<RedisClient>,
}

impl MoralisClient {
    fn new(api_key: String, redis_client: Option<RedisClient>) -> Self {
        Self { 
            api_key,
            redis: redis_client
        }
    }
}

impl TokenClient for MoralisClient {
    fn get_token_info(&self, token_address: &str) -> Result<TokenInfo, Box<dyn Error>> {
        // Try to get from Redis cache first if available
        if let Some(redis) = &self.redis {
            if let Some(cached_data) = redis.get_cached(token_address) {
                return Ok(serde_json::from_str(&cached_data)?);
            }
        }

        // If not in cache, make API request
        let url = format!(
            "https://deep-index.moralis.io/api/v2/erc20/metadata\
            ?chain=eth\
            &addresses={}",
            token_address
        );

        let response = ureq::get(&url)
            .set("X-API-Key", &self.api_key)
            .call()?;
        let data: serde_json::Value = serde_json::from_reader(response.into_reader())?;

        if let Some(token) = data.as_array().and_then(|arr| arr.first()) {
            let token_info: TokenInfo = serde_json::from_str(&token.to_string())?;
            
            // Cache the result for 1 hour if Redis is available
            if let Some(redis) = &self.redis {
                redis.set_cached(token_address, &token.to_string(), 3600);
            }

            return Ok(token_info);
        }

        Err("Failed to get token metadata".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: token_metadata <erc20_address>");
        std::process::exit(1);
    }

    let api_key = std::env::var("MORALIS_API_KEY")
        .expect("MORALIS_API_KEY must be set");

    // Create Redis client only if URL is configured
    let redis_client = std::env::var("REDIS_URL")
        .ok()
        .map(|url| RedisClient::new(url));

    let client = MoralisClient::new(api_key, redis_client);
    
    // Get the last argument as the token address, regardless of total argument count
    let token_address = args.last().unwrap();
    let token_info = client.get_token_info(token_address)?;
    
    println!("{}", serde_json::to_string_pretty(&token_info)?);
    Ok(())
}
