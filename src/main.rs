use anyhow::{self, Ok};
use clap::Parser;
use data_structure::{AudioAnalysis, Cleaned, Root, AudioAnalysis2};
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client, Method, Url,
};
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use indicatif::{ProgressBar, ProgressStyle};

// JSON structure of a playlist -- in this case the UK top 50 chart
mod data_structure;

static _APP_ID: &str = "174ca64a16024fe08a2f923ce6b57ac9";
static APP_SECRET: &str = "39e7d3f9f0154cdf91bb5cc76d3cfa72"; // HIDE THIS SHIT
static APP_ENDPOINT_CHART: &str =
    "https://api.spotify.com/v1/playlists/37i9dQZEVXbLnolsZ8PSNw?market=GB";
// 37i9dQZEVXbLnolsZ8PSNw = ID of the daily chart in the UK
static APP_ENDPOINT_ANALYSIS: &str = "https://api.spotify.com/v1/audio-features/";
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

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value = "./snapshot/", value_name = "PATH")]
    directory: PathBuf,

    #[arg(short, long, default_value = "snapshot", value_name = "MODE")]
    mode: String,

    #[arg(long, default_value_t = get_date(), value_name = "DATE")]
    date: String,
}

fn get_date() -> String {
    let rough_date = chrono::Local::now().to_string();
    let date = String::from(rough_date.split_whitespace().collect::<Vec<&str>>()[0]);

    date
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

async fn fetch_data() -> anyhow::Result<data_structure::Root> {
    let parsed_endpoint = Url::parse(APP_ENDPOINT_CHART)?;
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

async fn fetch_analyis(id: String) -> anyhow::Result<AudioAnalysis2> {
    let endpoint = format!("{}{}", APP_ENDPOINT_ANALYSIS, id);

    let parsed_endpoint = Url::parse(&endpoint)?;
    let bearer_token = format!("Bearer {}", get_auth_token().await?.access_token);
    let bearer_token_static = &bearer_token.as_str();

    let mut authorization_header = HeaderMap::new();
    authorization_header.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));
    authorization_header.insert(ACCEPT, HeaderValue::from_static(APP_CONTENT_POLICY));
    authorization_header.insert(AUTHORIZATION, HeaderValue::from_str(bearer_token_static)?);

    let top_tracks: AudioAnalysis2 = reqwest::Client::new()
        .request(Method::GET, parsed_endpoint)
        .headers(authorization_header)
        .send()
        .await?
        .json::<AudioAnalysis2>()
        .await?;

    Ok(top_tracks)
}

async fn snapshot(cli_directory: PathBuf) -> anyhow::Result<()> {
    let date = get_date();

    let file_name = format!("spotify-ukchart-{}.json", date);
    let mut directory = PathBuf::new();
    directory.push(cli_directory);
    directory.push("uncleaned");
    directory.push(file_name);

    println!("Attempting to save data to {}", directory.display());

    let file = File::create(directory)?;
    let mut writer = BufWriter::new(file);
    let daily_data: data_structure::Root = fetch_data().await?;

    serde_json::to_writer(&mut writer, &daily_data)?;

    Ok(())
}

fn analysis() -> anyhow::Result<()> {
    Ok(())
}

async fn clean() -> anyhow::Result<()> {
    let date: Args = Parser::parse();

    let uncleaned_json_path = format!("./snapshot/uncleaned/spotify-ukchart-{}.json", date.date);
    let uncleaned_json_file = File::open(uncleaned_json_path)?;
    let uncleaned_json_reader = BufReader::new(uncleaned_json_file);

    let uncleaned_json: Root = serde_json::from_reader(uncleaned_json_reader)?;

    let cleaned_json_path = format!("./snapshot/cleaned/spotify-ukchart-{}.json", date.date);
    let cleaned_json_file = File::create(cleaned_json_path)?;

    let mut cleaned_json_vec: Vec<Cleaned> = vec![];

    let pb = ProgressBar::new(uncleaned_json.tracks.items.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {wide_msg:.cyan.bold} {spinner:.green} [{bar:40.cyan/blue}]")?);

    for data_structure::Item {
        track,
        added_at: _,
        added_by: _,
        is_local: _,
        primary_color: _,
        video_thumbnail: _,
    } in uncleaned_json.tracks.items
    {
        let fetched_audio_feature: AudioAnalysis2 = fetch_analyis(track.id.clone()).await?;

        let msg = format!("{}", &track.name);
        pb.set_message(msg);

        cleaned_json_vec.push(Cleaned {
            name: track.name,
            duration_ms: track.duration_ms,
            popularity: track.popularity,
            id: track.id,
            audio_feature: AudioAnalysis { 
                acousticness: fetched_audio_feature.acousticness, 
                danceability: fetched_audio_feature.danceability, 
                duration_ms: fetched_audio_feature.duration_ms, 
                energy: fetched_audio_feature.energy, 
                instrumentalness: fetched_audio_feature.instrumentalness, 
                key: fetched_audio_feature.key, 
                liveness: fetched_audio_feature.liveness, 
                loudness: fetched_audio_feature.loudness, 
                mode: fetched_audio_feature.mode, 
                speechiness: fetched_audio_feature.speechiness, 
                tempo: fetched_audio_feature.tempo, 
                time_signature: fetched_audio_feature.time_signature, 
                valence: fetched_audio_feature.valence 
            },
        });

        pb.inc(1);

    }

    serde_json::to_writer(cleaned_json_file, &cleaned_json_vec)?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: Args = Parser::parse();

    match cli.mode.as_str() {
        "snapshot" => {
            snapshot(cli.directory).await?;
        }
        "analysis" => {
            analysis()?;
        }
        "cleaning" => {
            clean().await?;
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown mode! The two available mode are: snapshot, analysis or cleaning"
            ));
        }
    }

    Ok(())
}

// TODO
// - DONE Cleaning data to only keep useful infos for PCA
// - DONE: Add a new argument to handle dates in CLI.
// - DONE: Analysis PCA:
//      - https://crates.io/crates/petal-decomposition
// - Research why it isn't working exactly as it should

/*
TODO {
    - DONE: CLI API with clap
    - DONE: Data gestion & storage
}
*/