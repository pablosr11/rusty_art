use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    attributes: String,
    skin: Option<String>,
    r#type: String,
    ranking: Option<u16>,
    buyer_add: Option<String>,
    blockhash: Option<String>,
    last_sold_price: Option<f32>,
}

fn main() {
    const PATH: &str = "/Users/ps/repos/rusty_art/full.json";
    const TOP: f32 = 0.005;

    let nfts: Vec<Entry> = parse_json(PATH).unwrap();
    let threshold: f32 = (nfts.len() as f32 * TOP).round();

    let mut freq_map: HashMap<&str, usize> = HashMap::new();
    for nft in &nfts {
        for attribute in parse_attributes(&nft) {
            *freq_map.entry(attribute).or_insert(0) += 1;
        }
    }
}

fn parse_json<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let nfts: Vec<Entry> = serde_json::from_reader(reader)?;
    Ok(nfts)
}

fn parse_attributes(e: &Entry) -> Vec<&str> {
    return e.attributes.split(",").collect();
}
