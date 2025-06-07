use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FenerbahceInstruction {
    /// Initialize Fenerbahçe tracker starting from 2010-2011 season
    /// 
    /// Accounts expected by this instruction:
    /// 0. `[writable]` Fenerbahçe tracker PDA account to be initialized
    /// 1. `[writable, signer]` Payer account
    /// 2. `[]` System program
    InitializeTracker, // variant 0
    
    /// Play a season and update trophy count if Fenerbahçe won
    /// 
    /// Accounts expected by this instruction:
    /// 0. `[writable]` Fenerbahçe tracker PDA account
    PlaySeason, // variant 1
}

impl FenerbahceInstruction {
    /// Unpacks a byte buffer into a FenerbahceInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, _rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Match instruction type
        match variant {
            0 => Ok(FenerbahceInstruction::InitializeTracker),
            1 => Ok(FenerbahceInstruction::PlaySeason),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Seed for the global Fenerbahçe tracker PDA
pub const FB_TRACKER_SEED: &[u8] = b"fenerbahce_tracker";

/// Find the global Fenerbahçe tracker PDA address
pub fn find_tracker_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[FB_TRACKER_SEED], program_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_initialize_tracker() {
        let instruction_data = vec![0]; // Variant 0

        let instruction = FenerbahceInstruction::unpack(&instruction_data).unwrap();
        
        match instruction {
            FenerbahceInstruction::InitializeTracker => {},
            _ => panic!("Expected InitializeTracker instruction"),
        }
    }

    #[test]
    fn test_unpack_play_season() {
        let instruction_data = vec![1]; // Variant 1

        let instruction = FenerbahceInstruction::unpack(&instruction_data).unwrap();
        
        match instruction {
            FenerbahceInstruction::PlaySeason => {},
            _ => panic!("Expected PlaySeason instruction"),
        }
    }

    #[test]
    fn test_find_tracker_pda() {
        let program_id = Pubkey::new_unique();
        let (pda, bump) = find_tracker_pda(&program_id);
        
        // Should return the same address each time for the same program
        assert_eq!((pda, bump), find_tracker_pda(&program_id));
    }
}
