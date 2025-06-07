// This example plays a season for Fenerbahçe and updates trophy count if they won
// Run this multiple times to go through all seasons from 2010-2025

use counter_program::{
    client::{get_tracker_address, play_season},
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
use borsh::BorshDeserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🟡🔵 Fenerbahçe Season Simulation");
    
    // Connect to Solana devnet cluster
    let rpc_client = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );
    
    // Generate keypairs
    let program_id: Pubkey = Pubkey::from_str("C5j3ikzXVjiRGEdg47dyGu8trNMaMxXYagGp2mSGTR4m").unwrap();
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
    
    // Check if tracker exists
    let account = match rpc_client.get_account(&tracker_pubkey) {
        Ok(account) => account,
        Err(_) => {
            println!("❌ Fenerbahçe tracker doesn't exist!");
            println!("   Run client_init.rs first to initialize the tracker");
            return Ok(());
        }
    };
    
    // Read current tracker data
    let tracker_data = FenerbahceTracker::try_from_slice(&account.data)?;
    
    if tracker_data.is_season_complete() {
        println!("🏁 All seasons completed!");
        println!("   Final Fenerbahçe trophy count: {}", tracker_data.total_trophies);
        println!("   Seasons covered: 2010-2011 to 2024-2025");
        return Ok(());
    }
    
    println!("📊 Current Status:");
    println!("   Current season: {}", tracker_data.get_season_string());
    println!("   Total trophies: {}", tracker_data.total_trophies);
    println!("   Seasons played: {}", tracker_data.seasons_played);
    
    // Play the current season
    let play_season_ix = play_season(
        &program_id,
        &tracker_pubkey,
    );
    
    // Create and send play season transaction
    let play_season_tx = Transaction::new_signed_with_payer(
        &[play_season_ix],
        Some(&payer.pubkey()),
        &[&payer],
        rpc_client.get_latest_blockhash()
            .expect("Failed to get latest blockhash"),
    );
    
    let signature = rpc_client.send_and_confirm_transaction(&play_season_tx)
        .expect("Failed to play season");
    println!("✅ Season played! Transaction: {}", signature);
    
    // Read updated tracker data
    let account_data = rpc_client.get_account_data(&tracker_pubkey)
        .expect("Failed to get account data");
    let updated_tracker = FenerbahceTracker::try_from_slice(&account_data)
        .expect("Failed to deserialize tracker data");
    
    println!("\n📈 Updated Status:");
    println!("   Total trophies: {}", updated_tracker.total_trophies);
    println!("   Seasons played: {}", updated_tracker.seasons_played);
    
    if updated_tracker.is_season_complete() {
        println!("\n🎉 ALL SEASONS COMPLETED!");
        println!("🏆 Final Fenerbahçe trophy count: {}", updated_tracker.total_trophies);
        println!("📅 Period covered: 2010-2011 to 2024-2025");
        println!("🎯 Championships won in this period: {}", updated_tracker.total_trophies - 17);
    } else {
        println!("   Next season: {}", updated_tracker.get_season_string());
        println!("\n   Run this command again to play the next season!");
    }
    
    Ok(())
}
