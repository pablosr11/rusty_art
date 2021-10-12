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
    let threshold: u16 = (nfts.len() as f32 * TOP.round()) as u16;

    let frequency_map = build_fmap(&nfts);
    let rare_attributes = &frequency_map
        .into_iter()
        .filter(|entry| entry.1 < threshold)
        .collect::<Vec<(&str, u16)>>();

    println!(
        "{:?}\nforsale {} - threshold {}",
        rare_attributes,
        nfts.len(),
        threshold
    )
}

fn parse_json<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let nfts: Vec<Entry> = serde_json::from_reader(reader)?;
    Ok(nfts)
}

fn build_fmap(nfts: &Vec<Entry>) -> HashMap<&str, u16> {
    let mut freq_map: HashMap<&str, u16> = HashMap::new();
    for nft in nfts {
        for attribute in parse_attributes(&nft) {
            *freq_map.entry(attribute).or_insert(0) += 1;
        }
    }
    freq_map
}

fn parse_attributes(e: &Entry) -> Vec<&str> {
    return e.attributes.split(",").collect();
}
