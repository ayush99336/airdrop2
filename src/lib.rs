#![allow(dead_code)]
#![allow(unused_imports)]
use bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::hash,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::io::{self, BufRead};
use std::str::FromStr;
const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55a
ca5c9c80a";
#[cfg(test)]
mod tests {

    use solana_system_interface::instruction::transfer;

    use super::*;
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("Youve generated a new solana wallet: {}", kp.pubkey());
        println!(
            "To save your wallet, copy and paste the following into a
JSON file:"
        );
        println!("{:?}", kp.to_bytes());
    }
    #[test]
    fn claim_airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect(
            "Couldn't find
wallet file",
        );
        // we'll establish a connection to Solana devnet using the const we
        // defined above
        let client = solana_client::rpc_client::RpcClient::new(RPC_URL);
        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Could not find wallet file");
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature Verified"),
            false => println!("Verification Failed"),
        }
        let to_pubkey = Pubkey::from_str("EAxj4hiybAqfMm3zvN3ozH7bX6D3h9XztbcsRjNTAoSD").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent Blockhash");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:
https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
    #[test]
    fn base58_to_wallet() {
        println!("Enter your Private key as base58 string:");
        let stdin = io::stdin();

        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your Wallet file format is : ");
        let wallet = bs58::decode(base58).into_vec().unwrap();

        println!("{:?}", wallet);
    }
    #[test]
    fn wallet_to_base58() {
        println!(
            "Input your private key as a JSON byte array (e.g.
[12,34,...]):"
        );
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
