use bitcoin::blockdata::script::Instruction;
use bitcoin::{TxOut, VarInt};
use bitcoin::consensus::{Decodable, Decoder, deserialize};
use bitcoin::consensus::encode::Error;

pub const MARKER: u16 = 0x4f41;
pub const VERSION: u16 = 0x0100;

pub struct Payload{

    pub quantities: Vec<u64>,
    pub metadata: String

}

impl<D: Decoder> Decodable<D> for Payload{
    fn consensus_decode(d: &mut D) -> Result<Payload, Error> {
        let marker: u16 = Decodable::consensus_decode(d)?;
        if marker != MARKER.to_be() {
            return Err(Error::ParseFailed("Invalid marker."));
        }

        let version: u16 = Decodable::consensus_decode(d)?;
        if version != VERSION.to_be() {
            return Err(Error::ParseFailed("Invalid version."));
        }


        let VarInt(count): VarInt = Decodable::consensus_decode(d)?;
        let mut quantities: Vec<u64> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let mut value: u64 = 0;
            let mut offset: u64 = 0;
            loop {
                let b: u8 = Decodable::consensus_decode(d)?;
                value |= ((b as u64) & 0x7f) << offset;
                if (b as u64) & 0x80 == 0 { break;}
                offset += 7;
            }
            quantities.push(value);
        }

        let payload = Payload { quantities, metadata: Decodable::consensus_decode(d)? };
        return Ok(payload);

    }
}

pub trait TxOutExt{

    fn get_op_return_data(&self) -> Vec<u8>;

    fn is_openassets_marker(&self) -> bool;

}

impl TxOutExt for TxOut{

    fn get_op_return_data(&self) -> Vec<u8> {
        if self.script_pubkey.is_op_return() {
            let mut script_iter = self.script_pubkey.iter(false);
            script_iter.next(); // OP_RETURN
            let item = script_iter.next();
            if item.is_some() {
                return match item.unwrap() {
                    Instruction::PushBytes(value) => value.to_vec(),
                    _ => vec![]
                };
            } else{
                return vec![];
            }
        } else {
            return vec![];
        }
    }

    fn is_openassets_marker(&self) -> bool {
        if self.script_pubkey.is_op_return() {
            let op_return_data: Vec<u8> = self.get_op_return_data();
            let payload: Result<Payload, _> = deserialize(&op_return_data);
            println!("isOk = {:?}", payload.is_ok());
            println!("is_err = {:?}", payload.is_err());
            return payload.is_ok();
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::{Script, TxOut};
    use bitcoin::blockdata::script::Builder;
    use bitcoin::util::misc::hex_bytes;
    use hex::decode as hex_decode;

    use openassets::marker_output::TxOutExt;

    #[test]
    fn test_op_return_data(){
        // op return data
        let script: Script = Builder::from(hex_decode("6a244f4101000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d71").unwrap()).into_script();
        let txout = TxOut {value: 0, script_pubkey: script};
        assert_eq!(txout.get_op_return_data(), hex_bytes("4f4101000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d71").unwrap());

        // no op return
        let script: Script = Builder::from(hex_decode("76a91446c2fbfbecc99a63148fa076de58cf29b0bcf0b088ac").unwrap()).into_script();
        let no_data = TxOut {value: 0, script_pubkey: script};
        assert_eq!(no_data.get_op_return_data().len(), 0);
    }

    #[test]
    fn test_is_openassets_marker(){
        // no op return
        let no_data = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("76a91446c2fbfbecc99a63148fa076de58cf29b0bcf0b088ac").unwrap()).into_script()};
        assert!(!no_data.is_openassets_marker());

        // valid marker
        let valid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a244f4101000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d71").unwrap()).into_script()};
        assert!(valid_marker.is_openassets_marker());

        // invalid marker
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f4201000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d71").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());

        // invalid version
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f4102000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d71").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());

        // can not parse varint
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f410100ff").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());

        // can not decode leb128 data(invalid format)
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f410100018f8f").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());

        // can not decode leb128 data(EOFError)
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f410100028f7f").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());

        // no metadata length
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f410100018f7f").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());

        // invalid metadata length
        let invalid_marker = TxOut {value: 0, script_pubkey: Builder::from(hex_decode("6a4f4101000364007b1b753d68747470733a2f2f6370722e736d2f35596753553150672d").unwrap()).into_script()};
        assert!(!invalid_marker.is_openassets_marker());
    }

}