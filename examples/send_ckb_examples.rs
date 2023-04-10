use std::{error::Error as StdErr, str::FromStr};

use ckb_sdk::{
    constants::ONE_CKB,
    tx_builder::{builder::CkbTransactionBuilder, transfer::DefaultCapacityTransferBuilder},
    unlock::{get_unlock_handler, ContextFactory},
    Address, NetworkInfo,
};
use ckb_types::h256;

fn main() -> Result<(), Box<dyn StdErr>> {
    let network_info = NetworkInfo::testnet();
    let sender =  Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;
    let receiver = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;
    let mut builder = DefaultCapacityTransferBuilder::new_with_address(&network_info, sender)?;

    builder.add_output(&receiver, (501 * ONE_CKB).into());

    let mut tx_with_groups = builder.build().unwrap();

    let handler = get_unlock_handler(&network_info).unwrap();
    let private_key = h256!("0x6c9ed03816e3111e49384b8d180174ad08e29feb1393ea1b51cef1c505d4e36a");
    let sender_key = secp256k1::SecretKey::from_slice(private_key.as_bytes())
        .map_err(|err| format!("invalid sender secret key: {}", err))?;
    handler
        .unlock(
            &mut tx_with_groups,
            ContextFactory::make(vec![sender_key]).as_ref(),
        )
        .unwrap();

    let signed_tx_json = serde_json::to_string_pretty(&tx_with_groups)?;
    println!("signed tx_json: {}", signed_tx_json);
    Ok(())
}