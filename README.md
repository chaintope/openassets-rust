# openassets-rust [![openassets at crates.io](https://img.shields.io/crates/v/openassets.svg)](https://crates.io/crates/openassets) [![Build Status](https://travis-ci.org/chaintope/openassets-rust.svg?branch=master)](https://travis-ci.org/chaintope/openassets-rust)  [![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

The implementation of the [Open Assets Protocol](https://github.com/OpenAssets/open-assets-protocol) for Rust.

## Examples

bitcoin::TxOut supports marker output.

```rust
use bitcoin::{Script, TxOut};
use bitcoin::blockdata::script::Builder;
use bitcoin::util::misc::hex_bytes;
use hex::decode as hex_decode;
use openassets::marker_output::{TxOutExt, Payload};

let marker_output = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a244f4101000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d71").unwrap()).into_script()};

// judge marker output

marker_output.is_openassets_marker();

// get open assets payload

let payload: Payload = marker_output.get_oa_payload().unwrap();

// asset quantities
payload.quantities;
=> [127, 128, 12857]

// metadata
payload.metadata
=> "u=https://cpr.sm/5YgSU1Pg-q"

// encode payload
let metadata = "u=https://cpr.sm/5YgSU1Pg-q".to_string();
let payload = Payload { quantities: vec![100, 0, 123], metadata };
let serialized_marker: Vec<u8> = serialize(&payload);
```

Asset ID calculation.

```rust
use std::str::FromStr;
use bitcoin::blockdata::script::Builder;
use openassets::asset_id::AssetId;
use hex::decode as hex_decode;

let p2pkh = Builder::from(hex_decode("76a914010966776006953d5567439e5e39f86a0d273bee88ac").unwrap()).into_script();
let asset_id = AssetId::new(&p2pkh, bitcoin::network::constants::Network::Bitcoin);
asset_id.to_string();
=> "ALn3aK1fSuG27N96UGYB1kUYUpGKRhBuBC"

// load from string
AssetId::from_str("ALn3aK1fSuG27N96UGYB1kUYUpGKRhBuBC");
```

Open Assets Address

```rust
use std::str::FromStr;
use std::string::ToString;
use openassets::address::OAAddressConverter;

// convert btc address to open assets address
let addr = bitcoin::Address::from_str("1F2AQr6oqNtcJQ6p9SiCLQTrHuM9en44H8").unwrap();
addr.to_oa_address().unwrap().to_string();
=> "akQz3f1v9JrnJAeGBC4pNzGNRdWXKan4U6E"

// convert open assets address to btc address
let open_asset_addr = addr.to_oa_address().unwrap();
open_asset_addr.to_btc_addr().unwrap().to_string();
=> "1F2AQr6oqNtcJQ6p9SiCLQTrHuM9en44H8"
```

