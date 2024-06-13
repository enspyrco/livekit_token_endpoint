use livekit_api::access_token;
use warp::Filter;
use serde::{Serialize, Deserialize};
use std::env;

#[tokio::main]
async fn main() {
    // Define the route
    let create_token_route = warp::path("create-token")
        .map(|| {
            let token = create_token().unwrap();
            warp::reply::json(&TokenResponse { token })
        });

    // Start the server
    warp::serve(create_token_route).run(([0, 0, 0, 0], 8080)).await;
}

// Token creation function
fn create_token() -> Result<String, access_token::AccessTokenError> {
   let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
   let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

   let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
      .with_identity("identity")
      .with_name("name")
      .with_grants(access_token::VideoGrants {
         room_join: true,
         room: "my-room".to_string(),
         ..Default::default()
      })
      .to_jwt();
   return token
}

// Response structure
#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}