use bitcoin::network::constants::Network;
use bitcoin::util::address::Payload;
use bitcoin::consensus::encode;
use std::fmt::{self, Display, Formatter};
use bitcoin::util::base58;
use bitcoin::consensus::encode::Error::ParseFailed;

/// A Open Assets Address
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {

    pub network: Network,
    pub payload: Payload,

}

const NAMESPACE: u8 = 0x13;

impl Address {

    pub fn new(payload: Payload, network: bitcoin::network::constants::Network) -> Result<Self, encode::Error> {
        match payload {
            Payload::PubkeyHash(_) | Payload::ScriptHash(_) => {},
            _ => {return Err(ParseFailed("The Open Assets Address of the witness program does not defined."));}
        }
        Ok(Address { payload, network })
    }

    pub fn to_btc_addr(&self) -> Result<bitcoin::Address, encode::Error> {
        Ok(bitcoin::Address {network: self.network, payload: self.payload.clone()})
    }
}

impl Display for Address{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut prefixed = [0; 22];
        prefixed[0] = NAMESPACE;
        prefixed[1] = match self.network {
            bitcoin::network::constants::Network::Bitcoin => 0,
            bitcoin::network::constants::Network::Testnet | bitcoin::network::constants::Network::Regtest => 111
        };
        match self.payload {
            Payload::PubkeyHash(ref hash) => {
                prefixed[2..].copy_from_slice(&hash[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            },
            Payload::ScriptHash(ref hash) => {
                prefixed[2..].copy_from_slice(&hash[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            },
            Payload::WitnessProgram(_) => {
                fmt.write_str("The Open Assets Address of the witness program does not defined.")
            },
        }
    }
}

pub trait OAAddressConverter {

    fn to_oa_address(&self) -> Result<Address, encode::Error>;

}

impl OAAddressConverter for bitcoin::Address{
    fn to_oa_address(&self) -> Result<Address, encode::Error> {
        Address::new(self.payload.clone(), self.network)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::string::ToString;
    use openassets::address::OAAddressConverter;

    #[test]
    fn test_oa_address_converter(){
        let addr = bitcoin::Address::from_str("1F2AQr6oqNtcJQ6p9SiCLQTrHuM9en44H8").unwrap();
        assert_eq!("akQz3f1v9JrnJAeGBC4pNzGNRdWXKan4U6E", addr.to_oa_address().unwrap().to_string());
        assert_eq!(addr, addr.to_oa_address().unwrap().to_btc_addr().unwrap());


        let testnet_addr = bitcoin::Address::from_str("mkgW6hNYBctmqDtTTsTJrsf2Gh2NPtoCU4").unwrap();
        assert_eq!("bWvePLsBsf6nThU3pWVZVWjZbcJCYQxHCpE", testnet_addr.to_oa_address().unwrap().to_string());
        assert_eq!(testnet_addr, testnet_addr.to_oa_address().unwrap().to_btc_addr().unwrap());

        let segwit_addr = bitcoin::Address::from_str("bc1qvzvkjn4q3nszqxrv3nraga2r822xjty3ykvkuw").unwrap();
        assert!(segwit_addr.to_oa_address().is_err());
    }

}