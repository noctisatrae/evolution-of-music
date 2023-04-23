use anyhow;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client, Method, Url,
};
use serde::Deserialize;

mod data_structure;

static _APP_ID: &str = "174ca64a16024fe08a2f923ce6b57ac9";
static APP_SECRET: &str = "39e7d3f9f0154cdf91bb5cc76d3cfa72"; // HIDE THIS SHIT
                                                              // 37i9dQZEVXbLnolsZ8PSNw = ID of the daily chart in the UK
static APP_ENDPOINT: &str = "https://api.spotify.com/v1/playlists/37i9dQZEVXbLnolsZ8PSNw?market=GB";
static APP_USER_AGENT: &str = "reqwest";
static APP_CONTENT_POLICY: &str = "application/json";

// expires_in & token_type are deactivated because they're mandatory for the deserealization of the data but not useful as infos.
#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    access_token: String,
    #[allow(dead_code)]
    expires_in: u32,
    #[allow(dead_code)]
    token_type: String,
}

async fn get_auth_token() -> anyhow::Result<AccessTokenResponse> {
    let client = Client::new();

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(_APP_ID, Some(APP_SECRET))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;

    Ok(response)
}

//HashMap<String, Value> -> old version
async fn fetch_data() -> anyhow::Result<data_structure::Root> {
    let parsed_endpoint = Url::parse(APP_ENDPOINT)?;
    let bearer_token = format!("Bearer {}", get_auth_token().await?.access_token);
    let bearer_token_static = &bearer_token.as_str();

    let mut authorization_header = HeaderMap::new();
    authorization_header.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));
    authorization_header.insert(ACCEPT, HeaderValue::from_static(APP_CONTENT_POLICY));
    authorization_header.insert(AUTHORIZATION, HeaderValue::from_str(bearer_token_static)?);

    let top_tracks = reqwest::Client::new()
        .request(Method::GET, parsed_endpoint)
        .headers(authorization_header)
        .send()
        .await?
        .json::<data_structure::Root>()
        .await?;

    Ok(top_tracks)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dbg!(fetch_data().await?.tracks.items);
    Ok(())
}
