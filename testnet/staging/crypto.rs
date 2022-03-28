// This file was automatically generated
// This file was automatically generated on 2022-03-28 13:05:39.841709+00:00
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
    Exchange::HITBTC,
    Exchange::HUOBIPRO,
    Exchange::KRAKEN,
    Exchange::OKEX,
    Exchange::OSMOSIS,
];

#[derive(ToString, EnumString, EnumIter, PartialEq, Debug, Copy, Clone)]
enum Token {
    AAVE,
    ABYSS,
    ADA,
    AKRO,
    ALCX,
    ALGO,
    ALPHA,
    AMPL,
    ANC,
    ANT,
    ARPA,
    AST,
    ATOM,
    AUTO,
    AVAX,
    AXS,
    BAL,
    BAND,
    BAT,
    BCH,
    BEL,
    BETA,
    BLZ,
    BNB,
    BNT,
    BOBA,
    BOR,
    BSV,
    BTC,
    BTG,
    BTM,
    BTS,
    BTT,
    BTU,
    BUSD,
    BZRX,
    C98,
    CAKE,
    CELO,
    CKB,
    CND,
    COMP,
    CREAM,
    CRO,
    CRV,
    CUSD,
    CVC,
    DAI,
    DASH,
    DCR,
    DGB,
    DGX,
    DIA,
    DOGE,
    DOT,
    DPI,
    EGLD,
    ELF,
    ENJ,
    EOS,
    EQUAD,
    ETC,
    ETH,
    EURS,
    EWT,
    FET,
    FIL,
    FNX,
    FOR,
    FRAX,
    FTM,
    FTT,
    GDC,
    GEN,
    GHT,
    GNO,
    GRT,
    GVT,
    HBAR,
    HEGIC,
    HNT,
    HOT,
    HT,
    HYN,
    ICX,
    INDEX,
    INJ,
    IOST,
    IOTX,
    JOE,
    JST,
    KAI,
    KAVA,
    KDA,
    KEY,
    KMD,
    KNC,
    KP3R,
    KSM,
    LEND,
    LEO,
    LINA,
    LINK,
    LOOM,
    LRC,
    LSK,
    LTC,
    LUNA,
    MANA,
    MATIC,
    MCO,
    MET,
    MFG,
    MIM,
    MIOTA,
    MIR,
    MKR,
    MLN,
    MOVR,
    MTA,
    MTL,
    MVL,
    MYB,
    NEAR,
    NEO,
    NEXXO,
    NMR,
    NPXS,
    NXM,
    OCEAN,
    OGN,
    OHM,
    OKB,
    OMG,
    ONE,
    ONT,
    ORC,
    ORN,
    OSMO,
    OST,
    OXT,
    PAXG,
    PAY,
    PBTC,
    PERP,
    PICKLE,
    PLR,
    PLTC,
    PNK,
    PNT,
    POLY,
    POWR,
    QKC,
    QNT,
    QTUM,
    RAE,
    REN,
    RENBTC,
    REP,
    REQ,
    RLC,
    ROSE,
    RSR,
    RSV,
    RUNE,
    RVN,
    SAN,
    SAND,
    SC,
    SCRT,
    SFI,
    SHIB,
    SNT,
    SNX,
    SOL,
    SPELL,
    SPIKE,
    SPN,
    SRM,
    STMX,
    STORJ,
    STRK,
    STX,
    SUSD,
    SUSHI,
    SXP,
    THETA,
    TKN,
    TKX,
    TOMO,
    TRB,
    TRX,
    TRYB,
    TUSD,
    TWT,
    UBT,
    UMA,
    UNI,
    UOS,
    UPP,
    USDC,
    USDP,
    USDS,
    USDT,
    UST,
    VET,
    VIDT,
    WAN,
    WAVES,
    WBTC,
    WNXM,
    WRX,
    XEM,
    XHV,
    XLM,
    XMR,
    XRP,
    XTZ,
    XVS,
    XZC,
    YAM,
    YAMV2,
    YFI,
    YFII,
    ZB,
    ZEC,
    ZIL,
    ZRX,
}

#[derive(ToString, EnumString, EnumIter, EnumPropertyTrait, Debug, Copy, Clone, PartialEq)]
enum Exchange {
    #[strum(props(data_source_id = "275"))]
    BIBOX = 0,
    #[strum(props(data_source_id = "272"))]
    BINANCE = 1,
    #[strum(props(data_source_id = "268"))]
    BITFINEX = 2,
    #[strum(props(data_source_id = "277"))]
    BITTREX = 3,
    #[strum(props(data_source_id = "270"))]
    BRAVENEWCOIN = 4,
    #[strum(props(data_source_id = "278"))]
    COINBASEPRO = 5,
    #[strum(props(data_source_id = "276"))]
    COINGECKO = 6,
    #[strum(props(data_source_id = "274"))]
    COINMARKETCAP = 7,
    #[strum(props(data_source_id = "265"))]
    CRYPTOCOMPARE = 8,
    #[strum(props(data_source_id = "267"))]
    FTX = 9,
    #[strum(props(data_source_id = "273"))]
    HITBTC = 10,
    #[strum(props(data_source_id = "266"))]
    HUOBIPRO = 11,
    #[strum(props(data_source_id = "269"))]
    KRAKEN = 12,
    #[strum(props(data_source_id = "271"))]
    OKEX = 13,
    #[strum(props(data_source_id = "264"))]
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
            Token::AAVE => "010000111000000",
            Token::ABYSS => "000000111000000",
            Token::ADA => "010000111001100",
            Token::AKRO => "000000111000000",
            Token::ALCX => "100000111100000",
            Token::ALGO => "010000111001000",
            Token::ALPHA => "010000111000000",
            Token::AMPL => "000000111000000",
            Token::ANC => "000000111000000",
            Token::ANT => "010000111001000",
            Token::ARPA => "010000111000000",
            Token::AST => "000000111000000",
            Token::ATOM => "010001111001000",
            Token::AUTO => "010000110000000",
            Token::AVAX => "010001111001010",
            Token::AXS => "010000111001000",
            Token::BAL => "010000111001000",
            Token::BAND => "010010111001000",
            Token::BAT => "010100111001100",
            Token::BCH => "010000111001000",
            Token::BEL => "010000111000000",
            Token::BETA => "010000111000000",
            Token::BLZ => "010000111001000",
            Token::BNB => "010000111000000",
            Token::BNT => "010000111001000",
            Token::BOBA => "001000111100000",
            Token::BOR => "000000111000000",
            Token::BSV => "001000111001010",
            Token::BTC => "011111111011100",
            Token::BTG => "000000111000000",
            Token::BTM => "000000111000000",
            Token::BTS => "000000111000000",
            Token::BTT => "010000111001000",
            Token::BTU => "000000111000000",
            Token::BUSD => "010000111000000",
            Token::BZRX => "010000111000000",
            Token::C98 => "010000111000000",
            Token::CAKE => "000000111000000",
            Token::CELO => "010000111000000",
            Token::CKB => "000000111001000",
            Token::CND => "000000111000000",
            Token::COMP => "010101111000100",
            Token::CREAM => "000000111100000",
            Token::CRO => "000000111001000",
            Token::CRV => "010000111001000",
            Token::CUSD => "000000111000000",
            Token::CVC => "010000111001000",
            Token::DAI => "001001111000100",
            Token::DASH => "010000111001000",
            Token::DCR => "010000111001000",
            Token::DGB => "000000111000000",
            Token::DGX => "000000111000000",
            Token::DIA => "000000111000000",
            Token::DOGE => "010000111001000",
            Token::DOT => "010000111001000",
            Token::DPI => "000000111000000",
            Token::EGLD => "010000111000000",
            Token::ELF => "000000111001000",
            Token::ENJ => "010000111000000",
            Token::EOS => "011000111001000",
            Token::EQUAD => "000000111000000",
            Token::ETC => "010000110001000",
            Token::ETH => "011111111011100",
            Token::EURS => "000000111000000",
            Token::EWT => "000000111000000",
            Token::FET => "000000111000000",
            Token::FIL => "110001110011000",
            Token::FNX => "000000100000000",
            Token::FOR => "000000111000000",
            Token::FRAX => "000000111000000",
            Token::FTM => "010000111000000",
            Token::FTT => "010000111001000",
            Token::GDC => "000000110000000",
            Token::GEN => "000000110000000",
            Token::GHT => "000000100000000",
            Token::GNO => "000000111000000",
            Token::GRT => "010000111000000",
            Token::GVT => "000000111000000",
            Token::HBAR => "010000111000000",
            Token::HEGIC => "000000111000000",
            Token::HNT => "000000110000000",
            Token::HOT => "000000110000000",
            Token::HT => "000000111001000",
            Token::HYN => "000000101000000",
            Token::ICX => "010000111001000",
            Token::INDEX => "000000110000000",
            Token::INJ => "010000111001100",
            Token::IOST => "010000111001000",
            Token::IOTX => "000000111000000",
            Token::JOE => "000000111000000",
            Token::JST => "000000111000000",
            Token::KAI => "000000111000000",
            Token::KAVA => "010000111000000",
            Token::KDA => "000000111000000",
            Token::KEY => "010000111000000",
            Token::KMD => "000000111000000",
            Token::KNC => "010000000001000",
            Token::KP3R => "000000111000000",
            Token::KSM => "000000111001000",
            Token::LEND => "000000101000000",
            Token::LEO => "000000111000000",
            Token::LINA => "000000111000000",
            Token::LINK => "010101111001100",
            Token::LOOM => "000000111001000",
            Token::LRC => "010000111000000",
            Token::LSK => "010000111000000",
            Token::LTC => "011000111001000",
            Token::LUNA => "010000111001000",
            Token::MANA => "010000111001000",
            Token::MATIC => "010000111000000",
            Token::MCO => "000000011000000",
            Token::MET => "000000111000000",
            Token::MFG => "000000111000000",
            Token::MIM => "000000111000000",
            Token::MIOTA => "000000111000000",
            Token::MIR => "010000110000000",
            Token::MKR => "010000111001000",
            Token::MLN => "000000111001000",
            Token::MOVR => "000000111000100",
            Token::MTA => "000000111000000",
            Token::MTL => "010000111000000",
            Token::MVL => "000000111000000",
            Token::MYB => "000000111000000",
            Token::NEAR => "010000111011010",
            Token::NEO => "000000111000000",
            Token::NEXXO => "000000100000000",
            Token::NMR => "010000111000000",
            Token::NPXS => "000000111000000",
            Token::NXM => "000000110000000",
            Token::OCEAN => "010000111000000",
            Token::OGN => "000000111000000",
            Token::OHM => "000000111000000",
            Token::OKB => "000000111000000",
            Token::OMG => "010000111001000",
            Token::ONE => "010000111001000",
            Token::ONT => "010000111001000",
            Token::ORC => "000000110000000",
            Token::ORN => "000000110000000",
            Token::OSMO => "000000111000001",
            Token::OST => "000000111000000",
            Token::OXT => "000000100000000",
            Token::PAXG => "010000111000000",
            Token::PAY => "000000111000000",
            Token::PBTC => "000000110000000",
            Token::PERP => "000000111000000",
            Token::PICKLE => "000000111000000",
            Token::PLR => "000000111000000",
            Token::PLTC => "000000001000000",
            Token::PNK => "000000111000000",
            Token::PNT => "010000101000000",
            Token::POLY => "000000111000000",
            Token::POWR => "000000111000000",
            Token::QKC => "000000111000000",
            Token::QNT => "000000111000000",
            Token::QTUM => "010000111000000",
            Token::RAE => "000000110000000",
            Token::REN => "010000111001000",
            Token::RENBTC => "000000110000000",
            Token::REP => "010000111000000",
            Token::REQ => "000000111000000",
            Token::RLC => "010000111000000",
            Token::ROSE => "010000111000000",
            Token::RSR => "010000111001000",
            Token::RSV => "000000111000000",
            Token::RUNE => "000000111000000",
            Token::RVN => "010000100001000",
            Token::SAN => "000000111000000",
            Token::SAND => "010000111000010",
            Token::SC => "010000111000000",
            Token::SCRT => "000000111000000",
            Token::SFI => "000000111000000",
            Token::SHIB => "010001111001010",
            Token::SNT => "000000011001000",
            Token::SNX => "010000111001000",
            Token::SOL => "010000110000000",
            Token::SPELL => "010001111100000",
            Token::SPIKE => "000000110000000",
            Token::SPN => "000000101000000",
            Token::SRM => "010000111000000",
            Token::STMX => "000000111000000",
            Token::STORJ => "000000111000000",
            Token::STRK => "000000111000000",
            Token::STX => "010000110000000",
            Token::SUSD => "000000111000000",
            Token::SUSHI => "010000111001000",
            Token::SXP => "010000111000000",
            Token::THETA => "010000111001000",
            Token::TKN => "000000111000000",
            Token::TKX => "000000100000000",
            Token::TOMO => "010000111000000",
            Token::TRB => "010000111001000",
            Token::TRX => "010000111001000",
            Token::TRYB => "000000101000000",
            Token::TUSD => "010000111000000",
            Token::TWT => "010000111000000",
            Token::UBT => "000000111000000",
            Token::UMA => "010000111000000",
            Token::UNI => "010000111000000",
            Token::UOS => "000000110000000",
            Token::UPP => "000000111000000",
            Token::USDC => "010000111000000",
            Token::USDP => "010000111000000",
            Token::USDS => "000000101000000",
            Token::USDT => "000010111000000",
            Token::UST => "010001111100010",
            Token::VET => "010000111001000",
            Token::VIDT => "000000111000000",
            Token::WAN => "010000111000000",
            Token::WAVES => "010000111001000",
            Token::WBTC => "000000111000000",
            Token::WNXM => "010000101001000",
            Token::WRX => "000000111000000",
            Token::XEM => "000000111001000",
            Token::XHV => "000000100000000",
            Token::XLM => "010001111001100",
            Token::XMR => "010000111001000",
            Token::XRP => "011000111001000",
            Token::XTZ => "011100111001000",
            Token::XVS => "010000111000000",
            Token::XZC => "000000001000000",
            Token::YAM => "000000111000000",
            Token::YAMV2 => "000000111000000",
            Token::YFI => "010000111001000",
            Token::YFII => "010000111001000",
            Token::ZB => "000000101000000",
            Token::ZEC => "010000111001000",
            Token::ZIL => "010000111000000",
            Token::ZRX => "010000111001000",
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
        rates.push((median(symbol_pxs.get_mut(*&symbol).unwrap()) * (input.multiplier as f64)) as u64)
    }
    Output { rates }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
