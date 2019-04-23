use bitcoin::blockdata::script::Instruction;
use bitcoin::TxOut;

pub const MARKER: u16 = 0x4f41;
pub const VERSION: u16 = 0x0100;

pub struct Payload{

    pub quantities: Vec<u64>,
    pub metadata: String

}

pub trait TxOutExt{

    fn get_op_return_data(&self) -> Vec<u8>;

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

}