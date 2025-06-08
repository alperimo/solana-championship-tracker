use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    instruction::{FenerbahceInstruction, FB_TRACKER_SEED, find_tracker_pda}, 
    state::{FenerbahceTracker, SeasonData}
};

// Program metadata
pub const PROGRAM_NAME: &str = "Fenerbahçe Championship Tracker";
pub const PROGRAM_VERSION: &str = "1.0.0";

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction: FenerbahceInstruction,
    ) -> ProgramResult {
        msg!("🟡🔵 Fenerbahçe Championship Tracker - Processing instruction");
        match instruction {
            FenerbahceInstruction::InitializeTracker => {
                msg!("🚀 Instruction: Initialize Fenerbahçe Tracker");
                Self::process_initialize_tracker(program_id, accounts)
            }
            FenerbahceInstruction::PlaySeason => {
                msg!("⚽ Instruction: Play Season");
                Self::process_play_season(program_id, accounts)
            }
        }
    }

    /// Initialize Fenerbahçe tracker starting from 2010-2011 season
    fn process_initialize_tracker(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("📍 Starting Fenerbahçe tracker initialization");
        
        let accounts_iter = &mut accounts.iter();

        let tracker_account = next_account_info(accounts_iter)?;
        let payer_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        msg!("🔍 Tracker PDA: {}", tracker_account.key);
        msg!("💰 Payer: {}", payer_account.key);

        // Verify that the tracker account is the correct PDA
        let (expected_tracker_pda, tracker_bump) = find_tracker_pda(program_id);
        if tracker_account.key != &expected_tracker_pda {
            msg!("Invalid tracker account: expected PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if account is already initialized
        if tracker_account.data_len() > 0 {
            msg!("Fenerbahçe tracker already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Size of our tracker account (u64 + u16 + u8)
        let account_space = 8 + 2 + 1; // total_trophies + current_season + seasons_played

        // Calculate minimum balance for rent exemption
        let rent = Rent::get()?;
        let required_lamports = rent.minimum_balance(account_space);

        // Create the tracker account using PDA
        invoke_signed(
            &system_instruction::create_account(
                payer_account.key,    // Account paying for the new account
                tracker_account.key,  // PDA account to be created
                required_lamports,    // Amount of lamports to transfer to the new account
                account_space as u64, // Size in bytes to allocate for the data field
                program_id,           // Set program owner to our program
            ),
            &[
                payer_account.clone(),
                tracker_account.clone(),
                system_program.clone(),
            ],
            &[&[FB_TRACKER_SEED, &[tracker_bump]]], // PDA signer seeds
        )?;

        // Create a new FenerbahceTracker with initial values
        let tracker_data = FenerbahceTracker::new();

        // Get a mutable reference to the tracker account's data
        let mut account_data = &mut tracker_account.data.borrow_mut()[..];

        // Serialize the FenerbahceTracker struct into the account's data
        tracker_data.serialize(&mut account_data)?;

        msg!("🟡🔵 Fenerbahçe tracker initialized!");
        msg!("Starting season: {}", tracker_data.get_season_string());
        msg!("Initial trophies: {}", tracker_data.total_trophies);

        Ok(())
    }

    /// Play a season and update trophy count if Fenerbahçe won
    fn process_play_season(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("⚽ Starting season simulation");
        
        let accounts_iter = &mut accounts.iter();
        let tracker_account = next_account_info(accounts_iter)?;

        msg!("🔍 Tracker PDA: {}", tracker_account.key);

        // Verify that the tracker account is the correct PDA
        let (expected_tracker_pda, _) = find_tracker_pda(program_id);
        if tracker_account.key != &expected_tracker_pda {
            msg!("❌ Invalid tracker account: expected PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify account ownership
        if tracker_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        // Mutable borrow the account data
        let mut data = tracker_account.data.borrow_mut();

        // Deserialize the account data into our FenerbahceTracker struct
        let mut tracker_data: FenerbahceTracker = FenerbahceTracker::try_from_slice(&data)?;

        // Check if all seasons are completed
        if tracker_data.is_season_complete() {
            msg!("🏁 All seasons completed! Final trophy count: {}", tracker_data.total_trophies);
            return Ok(());
        }

        // Get current season data
        let season_data = SeasonData::get_season_data(tracker_data.current_season)
            .ok_or(ProgramError::InvalidAccountData)?;

        // Log season information
        msg!("🏈 Playing season: {}", tracker_data.get_season_string());
        msg!("📊 League position: {} - {}", season_data.position, season_data.description);

        // If Fenerbahçe won the championship (position 1), increment trophy count
        if season_data.champion {
            tracker_data.total_trophies = tracker_data
                .total_trophies
                .checked_add(1)
                .ok_or(ProgramError::InvalidAccountData)?;
            
            msg!("Trophy count increased to: {}", tracker_data.total_trophies);
        } else {
            msg!("😞 No trophy this season. Total trophies: {}", tracker_data.total_trophies);
        }

        // Move to next season
        tracker_data.current_season += 1;
        tracker_data.seasons_played += 1;

        // Serialize the updated tracker data back into the account
        tracker_data.serialize(&mut &mut data[..])?;

        if tracker_data.is_season_complete() {
            msg!("🎉 All seasons completed!");
            msg!("📈 Final Fenerbahçe trophy count: {}", tracker_data.total_trophies);
            msg!("📅 Seasons covered: 2010-2011 to 2024-2025");
        } else {
            msg!("⏭️  Next season: {}", tracker_data.get_season_string());
        }

        Ok(())
    }
}