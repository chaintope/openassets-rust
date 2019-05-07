use bitcoin_hashes::{hash160, Hash};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use bitcoin::consensus::encode;
use bitcoin::util::base58;
use bitcoin::Script;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AssetId {

    pub hash: bitcoin_hashes::hash160::Hash,
    pub network: bitcoin::network::constants::Network

}

impl AssetId {

    pub fn new(script: &Script, network: bitcoin::network::constants::Network) -> AssetId{
        AssetId { hash: hash160::Hash::hash(&script.to_bytes()), network}
    }

}

impl Display for AssetId{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut prefixed = [0; 21];
        prefixed[0] = match self.network {
            bitcoin::network::constants::Network::Bitcoin => 0x17,
            bitcoin::network::constants::Network::Testnet | bitcoin::network::constants::Network::Regtest => 0x73
        };
        prefixed[1..].copy_from_slice(&self.hash[..]);
        base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
    }
}

impl FromStr for AssetId {
    type Err = encode::Error;

    fn from_str(s: &str) -> Result<AssetId, encode::Error> {
        let data = base58::from_check(s)?;
        let (network, hash) = match data[0] {
            0x17 => (
                bitcoin::network::constants::Network::Bitcoin,
                hash160::Hash::from_slice(&data[1..]).unwrap()
            ),
            0x73 => (
                bitcoin::network::constants::Network::Testnet,
                hash160::Hash::from_slice(&data[1..]).unwrap()
            ),
            x   => return Err(encode::Error::Base58(base58::Error::InvalidVersion(vec![x])))
        };
        Ok(AssetId { hash, network })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use bitcoin::blockdata::script::Builder;
    use openassets::asset_id::AssetId;
    use hex::decode as hex_decode;

    #[test]
    fn test_calculate_asset_id() {
        let p2pkh = Builder::from(hex_decode("76a914010966776006953d5567439e5e39f86a0d273bee88ac").unwrap()).into_script();
        let p2pkh_asset = AssetId::new(&p2pkh, bitcoin::network::constants::Network::Bitcoin);
        assert_eq!("ALn3aK1fSuG27N96UGYB1kUYUpGKRhBuBC", p2pkh_asset.to_string());
        assert_eq!(AssetId::from_str("ALn3aK1fSuG27N96UGYB1kUYUpGKRhBuBC").unwrap(), p2pkh_asset);

        let p2sh  = Builder::from(hex_decode("a914f9d499817e88ef7b10a88673296c6d6df2f4292d87").unwrap()).into_script();
        let testnet_asset = AssetId::new(&p2sh, bitcoin::network::constants::Network::Testnet);
        assert_eq!("oMb2yzA542yQgwn8XtmGefTzBv5NJ2nDjh", testnet_asset.to_string());
        assert_eq!(AssetId::from_str("oMb2yzA542yQgwn8XtmGefTzBv5NJ2nDjh").unwrap(), testnet_asset);
    }

}