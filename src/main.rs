use livekit_api::access_token;
use warp::Filter;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Deserialize, Serialize)]
struct QueryParams {
    name: String,
    identity: String,
}

#[tokio::main]
async fn main() {
    // Define the route
    let create_token_route = warp::path("create-token")
        .and(warp::query::<QueryParams>())
        .map(|p: QueryParams| {
            let token = create_token(&p.identity, &p.name, String::from("room") ).unwrap();
            warp::reply::json(&TokenResponse { token })
        });

    // Start the server
    warp::serve(create_token_route).run(([0, 0, 0, 0], 8080)).await;
}

// Token creation function
fn create_token(identity : &str, name : &str, room : String) -> Result<String, access_token::AccessTokenError> {
   let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
   let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

   let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
      .with_identity(identity)
      .with_name(name)
      .with_grants(access_token::VideoGrants {
         room_join: true,
         room: room.to_string(),
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