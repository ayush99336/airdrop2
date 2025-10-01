#![allow(dead_code)]
#![allow(unused_imports)]

use bs58;
use solana_sdk::signature::{Keypair, Signer};
use std::io::{self, BufRead};
#[cfg(test)]
mod tests {
    use std::io::{self, BufRead};

    use solana_sdk::{self, signature::Keypair, signer::Signer};
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
    fn claim_airdrop() {}
    #[test]
    fn transfer_sol() {}
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
