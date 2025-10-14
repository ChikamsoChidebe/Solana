# CarbonChain API Reference

## Overview

CarbonChain provides three main smart contracts on Solana for carbon credit trading:

1. **Carbon Marketplace** - Core trading functionality
2. **Carbon Verification** - Third-party verification system  
3. **Carbon Registry** - Credit issuance and retirement

## Carbon Marketplace Program

### Program ID
```
CarbMktpLace11111111111111111111111111111111
```

### Instructions

#### `initialize_marketplace`
Initialize the global marketplace with fee structure and minimum credit amounts.

**Parameters:**
- `fee_percentage: u16` - Marketplace fee (basis points, e.g., 250 = 2.5%)
- `min_credit_amount: u64` - Minimum credits per transaction

**Accounts:**
- `marketplace` - PDA for marketplace state
- `authority` - Marketplace admin (signer)
- `system_program` - Solana system program

#### `create_carbon_project`
Register a new carbon offset project.

**Parameters:**
- `project_id: String` - Unique project identifier (max 32 chars)
- `project_name: String` - Human-readable name (max 64 chars)
- `project_type: ProjectType` - Type of carbon project
- `location: String` - Geographic location (max 64 chars)
- `estimated_credits: u64` - Expected credit generation
- `verification_standard: VerificationStandard` - Compliance standard
- `metadata_uri: String` - IPFS/Arweave URI for documentation (max 200 chars)

**Accounts:**
- `project` - PDA for project state
- `marketplace` - Global marketplace account
- `developer` - Project developer (signer)
- `system_program` - Solana system program

#### `list_credits`
List verified carbon credits for sale.

**Parameters:**
- `amount: u64` - Number of credits to list
- `price_per_credit: u64` - Price in lamports per credit
- `expiry_time: i64` - Unix timestamp when listing expires

**Accounts:**
- `listing` - PDA for listing state
- `project` - Project account being listed
- `marketplace` - Global marketplace account
- `seller` - Credit owner (signer)
- `system_program` - Solana system program

#### `purchase_credits`
Purchase carbon credits from an active listing.

**Parameters:**
- `amount: u64` - Number of credits to purchase

**Accounts:**
- `listing` - Listing being purchased from
- `purchase` - PDA for purchase record
- `marketplace` - Global marketplace account
- `buyer` - Credit purchaser (signer)
- `buyer_token_account` - Buyer's payment token account
- `seller_token_account` - Seller's payment token account
- `marketplace_fee_account` - Marketplace fee collection account
- `token_program` - SPL Token program

#### `retire_credits`
Permanently retire carbon credits for offset claims.

**Parameters:**
- `amount: u64` - Number of credits to retire
- `retirement_reason: String` - Reason for retirement (max 200 chars)

**Accounts:**
- `retirement` - PDA for retirement record
- `project` - Project account
- `owner` - Credit owner (signer)
- `system_program` - Solana system program

### Data Structures

#### `ProjectType`
```rust
pub enum ProjectType {
    Forestry,
    RenewableEnergy,
    EnergyEfficiency,
    Methane,
    Transportation,
    Agriculture,
    WasteManagement,
    CarbonCapture,
}
```

#### `VerificationStandard`
```rust
pub enum VerificationStandard {
    VCS,      // Verified Carbon Standard
    CDM,      // Clean Development Mechanism
    GoldStandard,
    CAR,      // Climate Action Reserve
    ACR,      // American Carbon Registry
    Plan,     // Plan Vivo
}
```

#### `ProjectStatus`
```rust
pub enum ProjectStatus {
    Pending,
    Verified,
    Suspended,
    Cancelled,
}
```

## Carbon Verification Program

### Program ID
```
CarbVerify11111111111111111111111111111111
```

### Instructions

#### `initialize_verifier`
Register as a third-party verifier.

**Parameters:**
- `verifier_name: String` - Verifier organization name (max 64 chars)
- `certification_level: CertificationLevel` - Verifier certification level
- `accreditation_body: String` - Accrediting organization (max 64 chars)

#### `submit_verification_request`
Submit a project for verification.

**Parameters:**
- `project_key: Pubkey` - Project to be verified
- `verification_type: VerificationType` - Type of verification
- `documentation_uri: String` - Supporting documentation URI (max 200 chars)
- `estimated_credits: u64` - Expected credits to be verified

#### `conduct_verification`
Complete verification process (verifier only).

**Parameters:**
- `verified_credits: u64` - Actual verified credits
- `verification_notes: String` - Verification findings (max 500 chars)
- `compliance_score: u8` - Score 0-100

#### `challenge_verification`
Challenge a verification result.

**Parameters:**
- `challenge_reason: String` - Reason for challenge (max 500 chars)
- `evidence_uri: String` - Supporting evidence URI (max 200 chars)

## Carbon Registry Program

### Program ID
```
CarbRegistry1111111111111111111111111111111
```

### Instructions

#### `initialize_registry`
Initialize a carbon credit registry.

**Parameters:**
- `registry_name: String` - Registry name (max 64 chars)
- `base_uri: String` - Base URI for metadata (max 200 chars)

#### `register_project`
Register a project in the registry.

**Parameters:**
- `project_id: String` - Unique project ID (max 32 chars)
- `vintage_year: u16` - Credit vintage year
- `methodology: String` - Methodology used (max 100 chars)
- `country_code: String` - ISO country code (max 3 chars)
- `project_developer: Pubkey` - Developer public key

#### `issue_credits`
Issue tokenized carbon credits.

**Parameters:**
- `serial_number_prefix: String` - Credit serial prefix (max 20 chars)
- `quantity: u64` - Number of credits to issue
- `issuance_date: i64` - Unix timestamp of issuance

#### `transfer_credits`
Transfer credits between accounts.

**Parameters:**
- `quantity: u64` - Number of credits to transfer
- `transfer_reason: String` - Reason for transfer (max 200 chars)

#### `retire_credits`
Permanently retire credits.

**Parameters:**
- `quantity: u64` - Number of credits to retire
- `retirement_reason: String` - Retirement reason (max 200 chars)
- `beneficiary: String` - Beneficiary of retirement (max 100 chars)

## Events

### Marketplace Events

#### `MarketplaceInitialized`
```rust
pub struct MarketplaceInitialized {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    pub min_credit_amount: u64,
}
```

#### `CarbonProjectCreated`
```rust
pub struct CarbonProjectCreated {
    pub project_id: String,
    pub developer: Pubkey,
    pub project_type: ProjectType,
    pub estimated_credits: u64,
}
```

#### `CreditsPurchased`
```rust
pub struct CreditsPurchased {
    pub purchase_id: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub total_cost: u64,
}
```

### Verification Events

#### `VerificationCompleted`
```rust
pub struct VerificationCompleted {
    pub verification_id: Pubkey,
    pub request_id: Pubkey,
    pub project: Pubkey,
    pub verifier: Pubkey,
    pub verified_credits: u64,
    pub compliance_score: u8,
}
```

### Registry Events

#### `CreditsIssued`
```rust
pub struct CreditsIssued {
    pub issuance_id: Pubkey,
    pub project_registry: Pubkey,
    pub serial_number_prefix: String,
    pub quantity: u64,
    pub recipient: Pubkey,
}
```

## Error Codes

### Common Errors
- `ProjectIdTooLong` - Project ID exceeds 32 characters
- `InvalidAmount` - Amount must be greater than 0
- `InsufficientCredits` - Not enough credits available
- `ProjectNotVerified` - Project must be verified first
- `ListingExpired` - Listing has expired

## PDA Seeds

### Marketplace PDAs
- Marketplace: `["marketplace"]`
- Project: `["project", project_id]`
- Listing: `["listing", project_key, seller_key]`
- Purchase: `["purchase", listing_key, buyer_key]`
- Retirement: `["retirement", project_key, owner_key]`

### Verification PDAs
- Verifier: `["verifier", authority_key]`
- Verification Request: `["verification_request", project_key, requester_key]`
- Verification Result: `["verification_result", request_key]`
- Challenge: `["challenge", verification_key, challenger_key]`

### Registry PDAs
- Registry: `["registry", authority_key]`
- Project Registry: `["project_registry", registry_key, project_id]`
- Credit Issuance: `["credit_issuance", project_registry_key, authority_key]`
- Transfer Record: `["transfer_record", project_registry_key, from_owner_key]`
- Credit Retirement: `["credit_retirement", project_registry_key, owner_key]`

## Integration Examples

### JavaScript/TypeScript

```typescript
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider } from '@coral-xyz/anchor';

// Initialize connection
const connection = new Connection('https://api.devnet.solana.com');
const provider = new AnchorProvider(connection, wallet, {});

// Load program
const program = new Program(idl, PROGRAM_ID, provider);

// Create project
await program.methods
  .createCarbonProject(
    "FOREST-001",
    "Amazon Conservation",
    { forestry: {} },
    "Brazil",
    new BN(50000),
    { vcs: {} },
    "https://ipfs.io/metadata"
  )
  .accounts({
    project: projectPda,
    marketplace: marketplacePda,
    developer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Python

```python
from solana.rpc.api import Client
from anchorpy import Program, Provider

# Initialize client
client = Client("https://api.devnet.solana.com")
provider = Provider(client, wallet)

# Load program
program = Program.load(idl, PROGRAM_ID, provider)

# Purchase credits
await program.rpc.purchase_credits(
    1000,  # amount
    ctx=Context(
        accounts={
            "listing": listing_pda,
            "purchase": purchase_pda,
            "marketplace": marketplace_pda,
            "buyer": wallet.public_key,
            # ... other accounts
        }
    )
)
```

## Rate Limits

- Maximum 100 requests per minute per IP
- Maximum 1000 transactions per hour per wallet
- Bulk operations limited to 50 items per transaction

## Support

For technical support and integration assistance:
- Documentation: https://docs.carbonchain.io
- Discord: https://discord.gg/carbonchain
- GitHub: https://github.com/carbonchain/solana
- Email: developers@carbonchain.io