use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const _COLLECTION: &str = "";
const TOP: f32 = 1.0;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Entry {
    id: u32,
    token_add: String,
    number: u32,
    currency: String,
    price: f32,
    link_img: String,
    for_sale: u8,
    programId: String,
    name: String,
    description: String,
    escrowAdd: String,
    seller_address: String,
    #[serde(deserialize_with = "transform_attributes")]
    attributes: Vec<String>,
    skin: Option<String>,
    r#type: String,
    ranking: Option<u16>,
    buyer_add: Option<String>,
    blockhash: Option<String>,
    last_sold_price: Option<f32>,
}

fn transform_attributes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.to_string()).collect::<Vec<String>>())
}

fn main() {
    let mut nfts: Vec<Entry> = _download_data(_COLLECTION);
    let frequency_map: HashMap<String, u16> = build_fmap(&nfts);

    for nft in &mut nfts {
        nft.ranking = calculate_ranking(nft, &frequency_map);
        if nft.ranking.unwrap() < 500 {
            println!(
                "{} has ranking {} and price {}",
                &nft.name,
                &nft.ranking.unwrap(),
                &nft.price
            );
        }
    }
    println!(
        "{:?}\n- threshold {}",
        &nfts.len(),
        threshold
    );
}

fn calculate_ranking(entry: &mut Entry, fmap: &HashMap<String, u16>) -> Option<u16> {
    let mut count = 0;
    for e in &entry.attributes {
        count += fmap.get(e).unwrap();
    }
    Some(count)
}

fn _parse_json<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let nfts: Vec<Entry> = serde_json::from_reader(reader)?;
    Ok(nfts)
}

fn build_fmap(nfts: &[Entry]) -> HashMap<String, u16> {
    let mut freq_map: HashMap<String, u16> = HashMap::new();
    for nft in nfts {
        for attribute in &nft.attributes {
            *freq_map.entry(attribute.to_string()).or_insert(0) += 1;
        }
    }
    freq_map
}

fn _download_data(collection: &str) -> Vec<Entry> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "{}{}",
        "https://qzlsklfacc.medianetwork.cloud/nft_for_sale?collection=", collection
    );
    client
        .get(url)
        .headers(_construct_headers())
        .send()
        .unwrap()
        .json()
        .unwrap()
}

fn _construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert("origin", HeaderValue::from_static("https://solanart.io"));
    headers.insert(
        "authority",
        HeaderValue::from_static("qzlsklfacc.medianetwork.cloud"),
    );
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            "\"Chromium\";v=\"94\", \"Google Chrome\";v=\"94\", \";Not A Brand\";v=\"99\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("macOS"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("cross-site"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-getch-dest", HeaderValue::from_static("empty"));
    headers.insert("referer", HeaderValue::from_static("https://solanart.io/"));
    headers.insert(
        "accept-language",
        HeaderValue::from_static("en-GB,en-US;q=0.9,en;q=0.8"),
    );
    headers.insert(
        "if-none-match",
        HeaderValue::from_static("W/\"110f20-2iRHziaYe8i1E4bCDaDV/5aiOVI\""),
    );
    headers
}
