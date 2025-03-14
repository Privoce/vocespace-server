use std::env;

pub fn api_key() -> String {
    env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set")
}

pub fn api_secret() -> String {
    env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set")
}
