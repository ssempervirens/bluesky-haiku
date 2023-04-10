use std::str::FromStr;

use adenosine::{app_bsky::Post, identifiers::Nsid};
use clap::builder::PossibleValuesParser;
use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json::json;

/// Bot that posts haikus to Bluesky
#[derive(Parser, Debug)]
struct Args {
    /// Username (example: haiku-bot.bsky.social)
    #[arg(short, long)]
    username: String,
    /// Password
    #[arg(short, long)]
    password: String,
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let mut client =
        adenosine::xrpc::XrpcClient::new("https:///bsky.social".to_owned(), None, None)?;
    client.auth_login(&args.username, &args.password)?;
    tracing::debug!(?client);

    let did = client.auth_did()?;

    let haikus = load_haikus("data/all_haiku.csv")?;

    loop {
        client.post(
            &Nsid::from_str("com.atproto.repo.createRecord")?,
            None,
            Some(json!({
                "repo": did,
                "collection": "app.bsky.feed.post",
                "record": {
                    "$type":"app.bsky.feed.post",
                    "text": haikus.choose(&mut thread_rng()),
                    "createdAt": adenosine::created_at_now(),
                }
            })),
        )?;

        std::thread::sleep(std::time::Duration::from_secs(60 * 15));
    }
}

fn load_haikus(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut haikus = Vec::new();
    for result in rdr.records() {
        let record = result?;
        haikus.push(format!(
            "{}\n{}\n{}",
            record.get(1).unwrap(),
            record.get(2).unwrap(),
            record.get(3).unwrap(),
        ));
    }
    Ok(haikus)
}
