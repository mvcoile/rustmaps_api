use anyhow::Result;
use clap::Parser;
use futures::future::join_all;
use reqwest::header::{self};
use rustmaps_api::*;
use tracing::{info, info_span};

const _EXAMPLE_MAP_ID: &str = "34a27cb6ef074ca392441b5de9f28f82";

/// A tool to request a batch of maps from the RustMaps API
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// API key for the RustMaps API
    #[arg(short = 'k', long, env = "RUSTMAPS_API_KEY")]
    api_key: Option<String>,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    // parse the command line arguments
    let args = Args::parse();

    info_span!("Setup");
    let client = reqwest::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, "application/json".parse().unwrap());
    if let Some(api_key) = args.api_key {
        headers.insert("x-api-key", api_key.parse().unwrap());
    }

    let span = info_span!("GET::Limits").entered();
    let lim_responses_fut: Vec<_> = (0..3)
        .map(|_| {
            client
                .get("https://api.rustmaps.com/v4/maps/limits")
                .headers(headers.clone())
                .send()
        })
        .collect();

    // Collect all the responses and filter out the errors
    let lim_responses: Vec<_> = join_all(lim_responses_fut)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    for lim_response in &lim_responses {
        info!("{}", lim_response.status());
    }
    let limit_fut: Vec<_> = lim_responses
        .into_iter()
        .map(|x| x.json::<Limits>())
        .collect();
    let limits: Vec<_> = join_all(limit_fut).await;
    for limit in &limits {
        if let Ok(limit) = limit {
            info!("{}", limit);
        }
    }
    span.exit();

    // let span = info_span!("GET::maps").entered();
    // let resp = client
    //     .get(format!("https://api.rustmaps.com/v4/maps/{EXAMPLE_MAP_ID}"))
    //     .headers(headers.clone())
    //     .send()
    //     .await?
    //     //.json::<HashMap<String, String>>()
    //     .text()
    //     .await?;

    // let resp: Value = serde_json::from_str(&resp)?;
    // let status = resp["meta"].as_object().unwrap()["statusCode"]
    //     .as_u64()
    //     .unwrap();
    // info!("map generation status: {status}");

    // match status {
    //     200 => {
    //         let data = resp["data"].as_object().unwrap();
    //         info!(
    //             "map generated: type: {}, id: {}, size: {}, seed: {}, url: {}, isCustomMap: {}, canDownload: {}, downloadUrl: {}",
    //             data["type"],
    //             data["id"],
    //             data["size"],
    //             data["seed"],
    //             data["url"],
    //             data["isCustomMap"],
    //             data["canDownload"],
    //             data["downloadUrl"],
    //         );
    //     }
    //     _ => {
    //         println!(
    //             "Status: {}",
    //             resp["meta"].as_object().unwrap()["errors"]
    //                 .as_array()
    //                 .unwrap()
    //                 .first()
    //                 .unwrap()
    //         );
    //     }
    // }
    // span.exit();

    // let default_data = data
    //     .iter()
    //     .find(|x| x["name"].as_str().unwrap().eq("Default"))
    //     .unwrap();
    // println!("default: {}", default_data["id"]);

    //println!("{resp:#?}");

    Ok(())
}
