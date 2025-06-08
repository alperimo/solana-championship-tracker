// This is an example of how you would initialize the Fenerbahçe tracker
// Starting from 2010-2011 season with 17 trophies

use counter_program::{
    client::{get_tracker_address, initialize_tracker},
    state::FenerbahceTracker,
};
use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
     commitment_config::CommitmentConfig,
     signature::{Keypair, Signer},
     transaction::Transaction,
};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use borsh::BorshDeserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🟡🔵 Fenerbahçe Championship Tracker Initialization");
    
    // Connect to Solana devnet cluster
    let rpc_client = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );
    
    // Generate keypairs
    let program_id: Pubkey = Pubkey::from_str("FWTvPqvnxNMserrz39P33H1LfnfokHqkrM7k2TfoYn7d").unwrap();
    let payer = Keypair::new();
    let tracker_pubkey = get_tracker_address(&program_id);
    
    // Fund the payer account
    println!("💰 Requesting airdrop for payer account: {}", payer.pubkey());
    
    let airdrop_signature = rpc_client
        .request_airdrop(&payer.pubkey(), 1_000_000_000) // 1 SOL
        .expect("Failed to request airdrop");
    
    rpc_client.confirm_transaction(&airdrop_signature)
        .expect("Failed to confirm airdrop");
    println!("✅ Airdrop confirmed");
    
    // Wait a bit to ensure the account is funded
    thread::sleep(Duration::from_secs(2));
    
    // Check balance
    let balance = rpc_client.get_balance(&payer.pubkey())
        .expect("Failed to get balance");
    println!("Payer balance: {} lamports ({} SOL)", balance, balance as f64 / 1_000_000_000.0);
    
    if balance == 0 {
        return Err("Account has no balance - airdrop may not have completed".into());
    }
    
    // Check if tracker already exists
    match rpc_client.get_account(&tracker_pubkey) {
        Ok(account) => {
            let tracker_data = FenerbahceTracker::try_from_slice(&account.data)?;
            println!("⚠️  Fenerbahçe tracker already exists!");
            println!("   Current season: {}", tracker_data.get_season_string());
            println!("   Total trophies: {}", tracker_data.total_trophies);
            println!("   Seasons played: {}", tracker_data.seasons_played);
            println!("   Use client_playseason.rs to continue playing seasons");
            return Ok(());
        }
        Err(_) => {
            println!("🚀 Fenerbahçe tracker doesn't exist, initializing...");
        }
    }
    
    // Initialize Fenerbahçe tracker
    let initialize_ix = initialize_tracker(
        &program_id,
        &tracker_pubkey,  // Pass the PDA as tracker account
        &payer.pubkey(),
    );
    
    // Create and send initialize transaction
    let initialize_tx = Transaction::new_signed_with_payer(
        &[initialize_ix],
        Some(&payer.pubkey()),
        &[&payer],
        rpc_client.get_latest_blockhash()
            .expect("Failed to get latest blockhash"),
    );
    
    let signature = rpc_client.send_and_confirm_transaction(&initialize_tx)
        .expect("Failed to initialize Fenerbahçe tracker");
    println!("✅ Fenerbahçe tracker initialized! Transaction: {}", signature);
    
    // Read initial tracker data
    let account_data = rpc_client.get_account_data(&tracker_pubkey)
        .expect("Failed to get account data");
    let tracker_data = FenerbahceTracker::try_from_slice(&account_data)
        .expect("Failed to deserialize tracker data");
    
    println!("\n🏆 Fenerbahçe Championship Tracker Initialized!");
    println!("   🗓️  Starting season: {}", tracker_data.get_season_string());
    println!("   🏆 Initial trophies: {}", tracker_data.total_trophies);
    println!("   📊 Seasons to play: 2010-2011 to 2024-2025 (15 seasons)");
    println!("   🎯 Run client_playseason to find out the champions!");
    println!("   📍 Tracker PDA address: {}", tracker_pubkey);
    println!("\n   Run client_playseason.rs to start playing seasons!");
    
    Ok(())
}
