// This file was automatically generated
// VERSION = 1

use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm::{execute_entry_point, ext, oei, prepare_entry_point};
use std::collections::hash_map::*;
use std::collections::HashMap;
use std::str::FromStr;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty as EnumPropertyTrait, EnumString, ToString};

#[derive(OBIDecode, OBISchema)]
struct Input {
    symbols: Vec<String>,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    rates: Vec<u64>,
}

const EXCHANGE_COUNT: u64 = 15;

const API_SOURCE: [Exchange; 15] = [
    Exchange::BIBOX,
    Exchange::BINANCE,
    Exchange::BITFINEX,
    Exchange::BITTREX,
    Exchange::BRAVENEWCOIN,
    Exchange::COINBASEPRO,
    Exchange::COINGECKO,
    Exchange::COINMARKETCAP,
    Exchange::CRYPTOCOMPARE,
    Exchange::FTX,
    Exchange::HUOBIPRO,
    Exchange::KRAKEN,
    Exchange::OKEX,
    Exchange::HITBTC,
    Exchange::OSMOSIS,
];

#[derive(ToString, EnumString, EnumIter, PartialEq, Debug, Copy, Clone)]
enum Token {
    BTC,
    ETH,
    USDT,
    INJ,
    BNB,
    LUNA,
    UST,
    ANC,
    ATOM,
    CRO,
    MIR,
    SCRT,
    STX,
    OSMO,
    MOVR,
    AVAX,
    SOL,
    FTM,
    NEAR,
    DOGE,
    DOT,
    ADA,
    COMP,
    HT,
    KSM,
    LINK,
    UNI,
    XRP,
    YFI,
    AAVE,
    ALCX,
    ALPHA,
    BAL,
    BCH,
    CAKE,
    CRV,
    EOS,
    HBAR,
    IOTX,
    LTC,
    MATIC,
    OHM,
    PERP,
    THETA,
    XTZ,
    FTT,
    ZIL,
    EGLD,
    HNT,
    KAI,
    KDA,
    ONE,
    TOMO,
    FIL,
}

#[derive(ToString, EnumString, EnumIter, EnumPropertyTrait, Debug, Copy, Clone, PartialEq)]
enum Exchange {
    #[strum(props(data_source_id = "231"))]
    BIBOX = 0,
    #[strum(props(data_source_id = "230"))]
    BINANCE = 1,
    #[strum(props(data_source_id = "226"))]
    BITFINEX = 2,
    #[strum(props(data_source_id = "227"))]
    BITTREX = 3,
    #[strum(props(data_source_id = "234"))]
    BRAVENEWCOIN = 4,
    #[strum(props(data_source_id = "235"))]
    COINBASEPRO = 5,
    #[strum(props(data_source_id = "251"))]
    COINGECKO = 6,
    #[strum(props(data_source_id = "236"))]
    COINMARKETCAP = 7,
    #[strum(props(data_source_id = "232"))]
    CRYPTOCOMPARE = 8,
    #[strum(props(data_source_id = "225"))]
    FTX = 9,
    #[strum(props(data_source_id = "224"))]
    HUOBIPRO = 10,
    #[strum(props(data_source_id = "228"))]
    KRAKEN = 11,
    #[strum(props(data_source_id = "229"))]
    OKEX = 12,
    #[strum(props(data_source_id = "252"))]
    HITBTC = 13,
    #[strum(props(data_source_id = "241"))]
    OSMOSIS = 14,
}

impl Exchange {
    fn from_u64(value: u64) -> Option<Exchange> {
        Exchange::iter().nth(value as usize)
    }
}

macro_rules! token_to_exchange_list {
    ($data:expr) => {
        match $data {
            Token::BTC => "011101111011000",
            Token::ETH => "011101111011000",
            Token::USDT => "001001111101000",
            Token::INJ => "010000111011000",
            Token::BNB => "010000111000000",
            Token::LUNA => "010000111010000",
            Token::UST => "000001111000100",
            Token::ANC => "000000111000000",
            Token::ATOM => "010001111010000",
            Token::CRO => "000000111010000",
            Token::MIR => "010000110000000",
            Token::SCRT => "000000111000000",
            Token::STX => "010000110000000",
            Token::OSMO => "000000111000001",
            Token::MOVR => "000000111001000",
            Token::AVAX => "010001111010100",
            Token::SOL => "010000110000000",
            Token::FTM => "010000111000000",
            Token::NEAR => "010000111010110",
            Token::DOGE => "010000111010000",
            Token::DOT => "010000111010000",
            Token::ADA => "010000111011000",
            Token::COMP => "010101111001000",
            Token::HT => "000000111010000",
            Token::KSM => "000000111010000",
            Token::LINK => "010101111011000",
            Token::UNI => "010000111000000",
            Token::XRP => "011000111010000",
            Token::YFI => "010000111010000",
            Token::AAVE => "010000111000000",
            Token::ALCX => "100000111100000",
            Token::ALPHA => "010000111000100",
            Token::BAL => "010000111010000",
            Token::BCH => "010000111010000",
            Token::CAKE => "000000111000000",
            Token::CRV => "010000111010000",
            Token::EOS => "011000111010000",
            Token::HBAR => "010000111000000",
            Token::IOTX => "000000111000000",
            Token::LTC => "011000111010000",
            Token::MATIC => "010000111000000",
            Token::OHM => "000000111000000",
            Token::PERP => "000000111000000",
            Token::THETA => "010000111010000",
            Token::XTZ => "011100111010000",
            Token::FTT => "010000111010000",
            Token::ZIL => "010000111000000",
            Token::EGLD => "010000111000000",
            Token::HNT => "010000111000000",
            Token::KAI => "000000111000000",
            Token::KDA => "000000111000000",
            Token::ONE => "010000111010000",
            Token::TOMO => "010000111000000",
            Token::FIL => "010001111010100",
        }
    };
}

fn get_ds_input(exchange_id: u64, symbols: Vec<Token>) -> String {
    let exchange = Exchange::from_u64(exchange_id).unwrap();
    if API_SOURCE.contains(&exchange) {
        format!(
            "{}",
            symbols
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        format!(
            "{} {}",
            exchange.to_string().to_ascii_lowercase(),
            symbols
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

fn get_ds_from_exchange(exchange_id: u64) -> i64 {
    let exchange = match Exchange::from_u64(exchange_id) {
        Some(data) => data,
        None => panic!("Unsupported Exchange ID"),
    };
    if API_SOURCE.contains(&exchange) {
        i64::from_str(exchange.get_str("data_source_id").unwrap()).unwrap()
    } else {
        3i64 // CCXT Data source id
    }
}

fn get_symbols_from_input(exchange_id: u64, input: String) -> Vec<String> {
    let exchange = Exchange::from_u64(exchange_id).unwrap();
    if API_SOURCE.contains(&exchange) {
        input.split(" ").map(|x| x.to_string()).collect()
    } else {
        let mut v: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
        v.drain(0..1);
        v
    }
}

// Get list of exchange that needs to be called along with the symbols to call
// given a list of input symbols
fn get_exchange_map(symbols: Vec<String>) -> HashMap<u64, Vec<Token>> {
    let mut exchange_map = HashMap::new();
    for symbol in symbols {
        let symbol_token = Token::from_str(symbol.as_str()).unwrap();
        let mut exchange_binary = token_to_exchange_list!(symbol_token).chars();
        for i in 0..(EXCHANGE_COUNT as usize) {
            if exchange_binary.next() == Some('1') {
                match exchange_map.entry(i as u64) {
                    Entry::Vacant(e) => {
                        e.insert(vec![symbol_token]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(symbol_token);
                    }
                }
            }
        }
    }
    exchange_map
}

fn median(arr: &mut Vec<f64>) -> f64 {
    let len_arr = arr.len() as f64;
    if len_arr > 0f64 {
        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = len_arr / 2f64;
        if len_arr as u64 % 2 == 0 {
            (arr[(mid - 1f64) as usize] + arr[mid as usize]) / 2f64
        } else {
            arr[mid as usize]
        }
    } else {
        0f64
    }
}

fn prepare_impl(input: Input) {
    let exchange_map = get_exchange_map(input.symbols);
    for (exchange_id, symbols) in exchange_map.iter() {
        oei::ask_external_data(
            *exchange_id as i64,
            get_ds_from_exchange(*exchange_id),
            get_ds_input(*exchange_id, symbols.to_vec()).as_bytes(),
        )
    }
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    // Get the required exchange and associated symbols to query
    let exchange_map = get_exchange_map((*input.symbols).to_vec());
    // store the median price of each token requested from an exchange
    let mut exchange_medians: Vec<Option<Vec<f64>>> = vec![Some(vec![]); EXCHANGE_COUNT as usize];
    for (exchange_id, _symbols) in exchange_map.iter() {
        // Get the data source calldata for a given external ID
        let raw_input = ext::load_input::<String>(*exchange_id as i64);
        let mut prices = vec![vec![]; exchange_map[exchange_id].len()];
        let inputs: Vec<String> = raw_input.collect();
        if inputs.len() == 0 {
            exchange_medians[*exchange_id as usize] = None;
            continue;
        }
        // for each validator response for the exchange,
        // split the response into individual prices
        for raw in inputs {
            let px_list: Vec<f64> = raw
                .split(",")
                .filter_map(|x| x.parse::<f64>().ok())
                .collect();
            // for each token price, add it to the list of validator responses
            // for that token and exchange
            for (idx, &px) in px_list.iter().enumerate() {
                prices[idx].push(px);
            }
        }
        let mut median_prices = vec![0f64; prices.len()];
        for (idx, price) in prices.iter().enumerate() {
            median_prices[idx] = median(&mut price.to_vec());
        }
        exchange_medians[*exchange_id as usize] = Some(median_prices);
    }

    let mut symbol_pxs = HashMap::new();
    for (exchange_id, symbols) in exchange_map.iter() {
        let exchange_median = exchange_medians[*exchange_id as usize].as_ref();
        if exchange_median.is_none() {
            continue;
        }
        let exchange_median = exchange_median.unwrap();
        let symbols_vec =
            get_symbols_from_input(*exchange_id, get_ds_input(*exchange_id, symbols.to_vec()));

        for (symbol_id, symbol) in symbols_vec.iter().enumerate() {
            match symbol_pxs.entry(symbol.clone()) {
                Entry::Vacant(e) => {
                    e.insert(vec![exchange_median[symbol_id]]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(exchange_median[symbol_id]);
                }
            }
        }
    }

    let mut rates = Vec::new();
    for symbol in input.symbols.iter() {
        rates.push(
            (median(symbol_pxs.get_mut(*&symbol).unwrap()) * (input.multiplier as f64)) as u64,
        )
    }
    Output { rates }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
