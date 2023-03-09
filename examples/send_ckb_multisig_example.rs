use std::error::Error as StdErr;

use ckb_sdk::{
    tx_builder::{
        builder::CkbTransactionBuilder,
        send_transaction,
        transfer::{
            sign_mutisig_tx_with_str_keys, DefaultMultisigCapacityTransferBuilder,
            MultisigTransactionInfo,
        },
    },
    unlock::MultisigConfig,
    NetworkInfo,
};

fn main() -> Result<(), Box<dyn StdErr>> {
    let network_info = NetworkInfo::testnet();
    let multisig_config = MultisigConfig::new_with_hash_str(
        &[
            "0x7336b0ba900684cb3cb00f0d46d4f64c0994a562".to_string(),
            "0x5724c1e3925a5206944d753a6f3edaedf977d77f".to_string(),
        ],
        0,
        2,
    )?;
    // ckt1qpw9q60tppt7l3j7r09qcp7lxnp3vcanvgha8pmvsa3jplykxn32sqdunqvd3g2felqv6qer8pkydws8jg9qxlca0st5v
    println!(
        "from address: {}",
        multisig_config.to_address(network_info.network_type)
    );
    let mut builder =
        DefaultMultisigCapacityTransferBuilder::new(network_info.clone(), multisig_config.clone())
            .unwrap();
    builder.add_output_raw("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r", 50100000000u64)?;
    builder.add_unlocker_from_str(&[
        "0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c",
    ])?;
    let (tx, unsigned_group) = builder.build_unlocked().unwrap();
    println!("unsigned group len: {}", unsigned_group.len());

    let tx_info = MultisigTransactionInfo::new(tx, multisig_config.clone());
    // send the transaction to the second signer
    let tx_info_str = serde_json::to_string_pretty(&tx_info).unwrap();
    // the second signer parse the json content
    let tx_info: MultisigTransactionInfo = serde_json::from_str(&tx_info_str).unwrap();
    let tx = tx_info.get_transaction();
    let (tx, _) = sign_mutisig_tx_with_str_keys(
        &network_info.url,
        tx,
        &multisig_config,
        &["0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087"],
    )?;

    println!(
        "{}",
        serde_json::to_string_pretty(&ckb_jsonrpc_types::TransactionView::from(tx.clone()))
            .unwrap()
    );
    let tx_hash = send_transaction(tx, &network_info.url)?;
    // example tx hash: 6fc4d63c5be5b1101b9bca416da872402c3f3466f5e993f9bfb920eaca7798c6
    // example tx hash: f01c355bd3f69671b0d6f131dae97789384b7c1a4e08b3ce3acde3e5be0222f2
    println!("tx {} sent", tx_hash);
    Ok(())
}
