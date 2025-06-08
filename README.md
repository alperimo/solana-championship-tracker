# Learn Solana Blockchain Development

An educational Solana program that teaches core blockchain development concepts through an engaging Fenerbahçe Football Club championship tracker. This hands-on tutorial demonstrates Solana's key features including Program Derived Addresses (PDAs), account management, instruction processing, and client interaction - all while following Fenerbahçe's exciting journey from 2010-2025.

## Solana Blockchain Integration

This program leverages Solana's unique features to create a decentralized championship tracking experience:

### Program Derived Addresses (PDAs)
- **Deterministic Account Creation**: Uses seed `"fenerbahce_tracker"` to generate a unique, predictable account address
- **Global State**: The same tracker account is accessible to all users of the program
- **Rent-Exempt Storage**: Tracker data persists permanently on the blockchain
- **No Private Keys**: PDA accounts are controlled entirely by the program logic

### Account Structure
- **Tracker Account**: 11-byte data structure storing:
  - `total_trophies` (u64): Current championship count
  - `current_season` (u16): Season year (e.g., 2010 for 2010-2011)
  - `seasons_played` (u8): Number of seasons completed
- **System Program**: Used for account creation and rent payment
- **Payer Account**: User's wallet that pays for transaction fees and account creation

### Blockchain Benefits
- **Immutable History**: All championship tracking is recorded permanently
- **Transparent Logic**: Season simulation and trophy counting logic is publicly verifiable
- **Decentralized Access**: Anyone can interact with the tracker from any Solana-compatible wallet
- **Low Cost**: Minimal transaction fees for season simulation
- **Cross-Platform**: Works with any Solana RPC endpoint (localhost, devnet, mainnet)

## Features

- Track Fenerbahçe's championship count starting from 17 trophies (as of 2010)
- Interactive season simulation covering 15 seasons (2010-2024)
- Real historical league positions and points data
- Championship detection and trophy counting using blockchain state
- Season-by-season progression without spoiling future results
- Solana PDA-based persistent storage across multiple client sessions
- Proper account validation and error handling
- Modular code structure following Solana best practices

## Project Structure

```
src/
├── lib.rs          # Main library file with module declarations
├── state.rs        # Fenerbahçe tracker data structure and season data
├── instruction.rs  # Instruction definitions for tracker operations
├── processor.rs    # Season simulation and championship tracking logic
├── entrypoint.rs   # Program entrypoint
└── client.rs       # Client helper functions for tracker operations

examples/
├── client_init.rs      # Initialize the Fenerbahçe tracker
└── client_playseason.rs # Play through seasons interactively
```

## Program Information

- **Program Name**: Fenerbahçe Championship Tracker
- **Network**: Devnet/Localnet
- **Version**: 1.0.0

### Instructions

| Instruction | Discriminator | Description |
|-------------|--------------|-------------|
| `InitializeTracker` | 0 | Initialize Fenerbahçe tracker with 17 initial trophies |
| `PlaySeason` | 1 | Simulate a season and update trophy count |

## Building the Program

```bash
# Build the program
cargo build

# Build for deployment (BPF target)
cargo build-sbf

# Run tests
cargo test --lib
```

## Testing Locally

### Option 1: Using Local Test Validator (Recommended)

1. Start a local Solana test validator:
   ```bash
   solana-test-validator
   ```

2. Deploy the program (in another terminal):
   ```bash
   solana program deploy target/deploy/solana-championship-tracker.so
   ```

3. Note the program ID and update it in the examples.

4. Run the local test example:
   ```bash
   cargo run --example client_init
   # After initialization, play seasons:
   cargo run --example client_playseason
   ```

### Option 2: Using Solana Devnet

1. Configure Solana CLI for devnet:
   ```bash
   solana config set --url https://api.devnet.solana.com
   ```

2. Create a keypair and fund it:
   ```bash
   solana-keygen new --outfile ~/.config/solana/id.json
   solana airdrop 2
   ```

3. Deploy the program:
   ```bash
   solana program deploy target/deploy/solana-championship-tracker.so
   ```

4. Run the devnet example:
   ```bash
   cargo run --example client_init
   # After initialization, play seasons:
   cargo run --example client_playseason
   ```

## Program Instructions

### Initialize Tracker
Creates a new Fenerbahçe championship tracker starting from the 2010-2011 season with 17 initial trophies.

**Accounts:**
- `[writable, signer]` Tracker account (PDA)
- `[writable, signer]` Payer account
- `[]` System program

**Data:**
- `u8`: Instruction discriminator (0)

### Play Season
Simulates the next season in Fenerbahçe's history, checking league position and updating trophy count if they won the championship.

**Accounts:**
- `[writable]` Tracker account (PDA)

**Data:**
- `u8`: Instruction discriminator (1)

## Usage Examples

The project includes two main client examples designed to teach Solana development concepts while experiencing Fenerbahçe's championship journey:

### 1. Initialize Tracker (One Time Setup)
```bash
cargo run --example client_init
```
This educational example demonstrates:
- **PDA Creation**: How to create Program Derived Addresses for global state
- **Account Initialization**: Setting up persistent blockchain storage
- **System Program Interaction**: Using Solana's built-in system program

Initializes the Fenerbahçe championship tracker starting from the 2010-2011 season with 17 trophies. Only needs to be run once to set up the tracker.

### 2. Play Through Seasons (Interactive Experience)
```bash
cargo run --example client_playseason
```
This example teaches:
- **State Management**: Reading and updating blockchain account data
- **Instruction Processing**: How Solana programs handle different instruction types
- **Client-Program Communication**: Sending transactions and reading results

Plays through each season from 2010-2024, displaying:
- Season information (e.g., "2010-2011")
- League position and points earned
- Championship status and trophy updates
- Historical context and achievements

Run this multiple times to progress through all 15 seasons and witness Fenerbahçe's championship journey unfold!

### ⚠️ Important: Update Program ID After Deployment

**Before running the examples**, you must update the program ID in both client files with your deployed program address:

#### Step 1: Deploy Your Program
```bash
# Deploy the program and note the Program Id from output
solana program deploy target/deploy/solana-championship-tracker.so
```

#### Step 2: Get Your Program Address (Alternative)
```bash
# Or get your program address from the keypair
solana address -k target/deploy/solana-championship-tracker-keypair.json
```

#### Step 3: Update Client Files
Replace the program ID string in **both** example files:
- `examples/client_init.rs` (line ~11)
- `examples/client_playseason.rs` (line ~11)

Look for this line and replace with your actual program ID:
```rust
let program_id = Pubkey::from_str("YOUR_DEPLOYED_PROGRAM_ID_HERE").unwrap();
```

**Example:**
```rust
let program_id = Pubkey::from_str("9W8gF7dGJkLkNQ3PkRzVBsKqNjRc2TpM4XvYhF5E8aD1").unwrap();
```

## How It Works (Solana Development Concepts)

1. **PDA Initialization**: Creates a Program Derived Address using a deterministic seed, demonstrating Solana's account model
2. **State Management**: Shows how to serialize/deserialize data structures using Borsh for on-chain storage
3. **Instruction Processing**: Demonstrates how Solana programs handle different instruction types and validate account access
4. **Account Validation**: Teaches proper account ownership checks and PDA verification patterns
5. **Cross-Program Invocation**: Uses the System Program to create accounts, showing inter-program communication

## Key Benefits of This Approach (for Learning Solana)

- **Hands-On Learning**: Practice core Solana concepts through interactive blockchain transactions
- **Real Blockchain State**: Work with actual on-chain data persistence and account management
- **Production Patterns**: Learn industry-standard practices for PDA usage and account validation
- **Client Integration**: Understand how to build client applications that interact with Solana programs
- **Transaction Building**: Master the process of creating, signing, and sending Solana transactions
- **Account Model**: Gain deep understanding of Solana's unique account-based architecture

## Common Issues

### Airdrop Rate Limiting
If you encounter airdrop rate limiting on devnet:
- Wait a few minutes and try again
- Use `solana airdrop` CLI command
- Use a different RPC endpoint
- Switch to local test validator

### Program Not Found
If you get "Program not found" errors:
- Make sure the program is deployed
- Update the program ID in your client code
- Verify you're connecting to the correct cluster

## Dependencies

- `solana-program`: Core Solana program SDK
- `borsh`: Serialization library for Fenerbahçe tracker data
- `solana-client`: RPC client for interacting with Solana
- `solana-sdk`: Solana SDK for transactions and keypairs

## About Fenerbahçe

Fenerbahçe Spor Kulübü is one of Turkey's most successful and beloved football clubs, founded in 1907. The tracker covers their journey from 2010-2024, capturing their pursuit of championship glory in the Turkish Süper Lig.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

This educational project demonstrates Solana blockchain development concepts while celebrating Fenerbahçe's rich football history.
