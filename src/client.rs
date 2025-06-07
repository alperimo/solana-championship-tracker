use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use crate::instruction::find_tracker_pda;

/// Creates an instruction to initialize Fenerbahçe tracker
pub fn initialize_tracker(
    program_id: &Pubkey,
    tracker_account: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    // Serialize the instruction data
    let instruction_data = vec![0]; // Variant 0 for InitializeTracker

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*tracker_account, false), // Tracker account (writable, not signer - program will sign via invoke_signed)
            AccountMeta::new(*payer, true),            // Payer account (writable, signer)
            AccountMeta::new_readonly(solana_program::system_program::id(), false), // System program
        ],
        data: instruction_data,
    }
}

/// Creates an instruction to play a season
pub fn play_season(
    program_id: &Pubkey,
    tracker_account: &Pubkey,
) -> Instruction {
    let instruction_data = vec![1]; // Variant 1 for PlaySeason

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*tracker_account, false), // Tracker account (writable, not signer)
        ],
        data: instruction_data,
    }
}

/// Get the global Fenerbahçe tracker PDA address for this program
pub fn get_tracker_address(program_id: &Pubkey) -> Pubkey {
    let (tracker_pda, _) = find_tracker_pda(program_id);
    tracker_pda
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_initialize_tracker_instruction() {
        let program_id = Pubkey::new_unique();
        let tracker_account = Pubkey::new_unique();
        let payer = Pubkey::new_unique();

        let instruction = initialize_tracker(&program_id, &tracker_account, &payer);

        assert_eq!(instruction.program_id, program_id);
        assert_eq!(instruction.accounts.len(), 3);
        assert_eq!(instruction.accounts[0].pubkey, tracker_account);
        assert!(!instruction.accounts[0].is_signer); // Tracker account should NOT be marked as signer - program handles this
        assert_eq!(instruction.accounts[1].pubkey, payer);
        assert!(instruction.accounts[1].is_signer); // Payer should be signer
        assert_eq!(instruction.accounts[2].pubkey, solana_program::system_program::id());
        assert!(!instruction.accounts[2].is_signer); // System program should not be signer
        
        // Check instruction data
        assert_eq!(instruction.data, vec![0]);
    }

    #[test]
    fn test_play_season_instruction() {
        let program_id = Pubkey::new_unique();
        let tracker_account = Pubkey::new_unique();

        let instruction = play_season(&program_id, &tracker_account);

        assert_eq!(instruction.program_id, program_id);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(instruction.accounts[0].pubkey, tracker_account);
        assert!(!instruction.accounts[0].is_signer);
        assert_eq!(instruction.data, vec![1]);
    }

    #[test]
    fn test_get_tracker_address() {
        let program_id = Pubkey::new_unique();
        let tracker_address = get_tracker_address(&program_id);
        
        // Should return the same address each time for the same program
        assert_eq!(tracker_address, get_tracker_address(&program_id));
    }
}
