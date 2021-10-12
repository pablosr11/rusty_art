use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const PATH: &str = "/Users/ps/repos/rusty_art/dd.json";
const TOP: f32 = 0.004;

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

impl Entry {
    // store these as vec of strings when parsing the json
    fn parsed_attributes<'a>(&'a self) -> Vec<&'a str> {
        return self.attributes.split(",").collect();
    }
}

fn main() {
    let mut nfts: Vec<Entry> = parse_json(PATH).unwrap();
    let threshold: u16 = (nfts.len() as f32 * TOP) as u16;
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

fn parse_json<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let nfts: Vec<Entry> = serde_json::from_reader(reader)?;
    Ok(nfts)
}

fn build_fmap(nfts: &Vec<Entry>) -> HashMap<&str, u16> {
    let mut freq_map: HashMap<&str, u16> = HashMap::new();
    for nft in nfts {
        for attribute in nft.parsed_attributes() {
            *freq_map.entry(attribute).or_insert(0) += 1;
        }
    }
    freq_map
}
