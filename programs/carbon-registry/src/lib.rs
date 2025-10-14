use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, MintTo};

declare_id!("CarbRegistry1111111111111111111111111111111");

#[program]
pub mod carbon_registry {
    use super::*;

    pub fn initialize_registry(
        ctx: Context<InitializeRegistry>,
        registry_name: String,
        base_uri: String,
    ) -> Result<()> {
        require!(registry_name.len() <= 64, ErrorCode::RegistryNameTooLong);
        require!(base_uri.len() <= 200, ErrorCode::BaseUriTooLong);

        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.registry_name = registry_name.clone();
        registry.base_uri = base_uri;
        registry.total_credits_issued = 0;
        registry.total_credits_retired = 0;
        registry.total_projects = 0;
        registry.created_at = Clock::get()?.unix_timestamp;
        registry.bump = *ctx.bumps.get("registry").unwrap();

        emit!(RegistryInitialized {
            registry_id: registry.key(),
            authority: registry.authority,
            registry_name,
        });

        Ok(())
    }

    pub fn register_project(
        ctx: Context<RegisterProject>,
        project_id: String,
        vintage_year: u16,
        methodology: String,
        country_code: String,
        project_developer: Pubkey,
    ) -> Result<()> {
        require!(project_id.len() <= 32, ErrorCode::ProjectIdTooLong);
        require!(methodology.len() <= 100, ErrorCode::MethodologyTooLong);
        require!(country_code.len() <= 3, ErrorCode::CountryCodeTooLong);
        require!(vintage_year >= 2000 && vintage_year <= 2100, ErrorCode::InvalidVintageYear);

        let project_registry = &mut ctx.accounts.project_registry;
        project_registry.project_id = project_id.clone();
        project_registry.vintage_year = vintage_year;
        project_registry.methodology = methodology;
        project_registry.country_code = country_code;
        project_registry.project_developer = project_developer;
        project_registry.registry = ctx.accounts.registry.key();
        project_registry.total_issued = 0;
        project_registry.total_retired = 0;
        project_registry.status = ProjectRegistryStatus::Active;
        project_registry.registered_at = Clock::get()?.unix_timestamp;
        project_registry.bump = *ctx.bumps.get("project_registry").unwrap();

        let registry = &mut ctx.accounts.registry;
        registry.total_projects += 1;

        emit!(ProjectRegistered {
            project_registry_id: project_registry.key(),
            project_id,
            vintage_year,
            project_developer,
        });

        Ok(())
    }

    pub fn issue_credits(
        ctx: Context<IssueCredits>,
        serial_number_prefix: String,
        quantity: u64,
        issuance_date: i64,
    ) -> Result<()> {
        require!(serial_number_prefix.len() <= 20, ErrorCode::SerialNumberPrefixTooLong);
        require!(quantity > 0, ErrorCode::InvalidQuantity);
        require!(issuance_date <= Clock::get()?.unix_timestamp, ErrorCode::InvalidIssuanceDate);

        let issuance = &mut ctx.accounts.credit_issuance;
        issuance.project_registry = ctx.accounts.project_registry.key();
        issuance.serial_number_prefix = serial_number_prefix.clone();
        issuance.quantity = quantity;
        issuance.issuance_date = issuance_date;
        issuance.issued_to = ctx.accounts.recipient.key();
        issuance.status = IssuanceStatus::Active;
        issuance.created_at = Clock::get()?.unix_timestamp;
        issuance.bump = *ctx.bumps.get("credit_issuance").unwrap();

        // Mint tokens to represent carbon credits
        let mint_to_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.credit_mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.registry.to_account_info(),
            },
        );

        let registry_seeds = &[
            b"registry",
            ctx.accounts.registry.authority.as_ref(),
            &[ctx.accounts.registry.bump],
        ];
        let signer_seeds = &[&registry_seeds[..]];

        token::mint_to(mint_to_ctx.with_signer(signer_seeds), quantity)?;

        let project_registry = &mut ctx.accounts.project_registry;
        project_registry.total_issued += quantity;

        let registry = &mut ctx.accounts.registry;
        registry.total_credits_issued += quantity;

        emit!(CreditsIssued {
            issuance_id: issuance.key(),
            project_registry: project_registry.key(),
            serial_number_prefix,
            quantity,
            recipient: issuance.issued_to,
        });

        Ok(())
    }

    pub fn transfer_credits(
        ctx: Context<TransferCredits>,
        quantity: u64,
        transfer_reason: String,
    ) -> Result<()> {
        require!(quantity > 0, ErrorCode::InvalidQuantity);
        require!(transfer_reason.len() <= 200, ErrorCode::TransferReasonTooLong);

        let transfer_record = &mut ctx.accounts.transfer_record;
        transfer_record.from_owner = ctx.accounts.from_owner.key();
        transfer_record.to_owner = ctx.accounts.to_owner.key();
        transfer_record.project_registry = ctx.accounts.project_registry.key();
        transfer_record.quantity = quantity;
        transfer_record.transfer_reason = transfer_reason.clone();
        transfer_record.transferred_at = Clock::get()?.unix_timestamp;
        transfer_record.bump = *ctx.bumps.get("transfer_record").unwrap();

        // Transfer tokens
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.from_owner.to_account_info(),
            },
        );

        token::transfer(transfer_ctx, quantity)?;

        emit!(CreditsTransferred {
            transfer_id: transfer_record.key(),
            from_owner: transfer_record.from_owner,
            to_owner: transfer_record.to_owner,
            quantity,
            reason: transfer_reason,
        });

        Ok(())
    }

    pub fn retire_credits(
        ctx: Context<RetireCredits>,
        quantity: u64,
        retirement_reason: String,
        beneficiary: String,
    ) -> Result<()> {
        require!(quantity > 0, ErrorCode::InvalidQuantity);
        require!(retirement_reason.len() <= 200, ErrorCode::RetirementReasonTooLong);
        require!(beneficiary.len() <= 100, ErrorCode::BeneficiaryTooLong);

        let retirement = &mut ctx.accounts.credit_retirement;
        retirement.owner = ctx.accounts.owner.key();
        retirement.project_registry = ctx.accounts.project_registry.key();
        retirement.quantity = quantity;
        retirement.retirement_reason = retirement_reason.clone();
        retirement.beneficiary = beneficiary;
        retirement.retired_at = Clock::get()?.unix_timestamp;
        retirement.bump = *ctx.bumps.get("credit_retirement").unwrap();

        // Burn tokens to represent retirement
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: ctx.accounts.credit_mint.to_account_info(),
                from: ctx.accounts.owner_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );

        token::burn(burn_ctx, quantity)?;

        let project_registry = &mut ctx.accounts.project_registry;
        project_registry.total_retired += quantity;

        let registry = &mut ctx.accounts.registry;
        registry.total_credits_retired += quantity;

        emit!(CreditsRetired {
            retirement_id: retirement.key(),
            owner: retirement.owner,
            project_registry: project_registry.key(),
            quantity,
            reason: retirement_reason,
        });

        Ok(())
    }

    pub fn create_batch(
        ctx: Context<CreateBatch>,
        batch_id: String,
        vintage_start: i64,
        vintage_end: i64,
        monitoring_report_uri: String,
    ) -> Result<()> {
        require!(batch_id.len() <= 32, ErrorCode::BatchIdTooLong);
        require!(vintage_end >= vintage_start, ErrorCode::InvalidVintagePeriod);
        require!(monitoring_report_uri.len() <= 200, ErrorCode::MonitoringReportUriTooLong);

        let batch = &mut ctx.accounts.credit_batch;
        batch.batch_id = batch_id.clone();
        batch.project_registry = ctx.accounts.project_registry.key();
        batch.vintage_start = vintage_start;
        batch.vintage_end = vintage_end;
        batch.monitoring_report_uri = monitoring_report_uri;
        batch.total_credits = 0;
        batch.available_credits = 0;
        batch.status = BatchStatus::Pending;
        batch.created_at = Clock::get()?.unix_timestamp;
        batch.bump = *ctx.bumps.get("credit_batch").unwrap();

        emit!(BatchCreated {
            batch_id: batch.key(),
            project_registry: batch.project_registry,
            batch_identifier: batch_id,
            vintage_start,
            vintage_end,
        });

        Ok(())
    }

    pub fn update_project_status(
        ctx: Context<UpdateProjectStatus>,
        new_status: ProjectRegistryStatus,
        reason: String,
    ) -> Result<()> {
        require!(reason.len() <= 200, ErrorCode::ReasonTooLong);

        let project_registry = &mut ctx.accounts.project_registry;
        let old_status = project_registry.status;
        project_registry.status = new_status;

        emit!(ProjectStatusUpdated {
            project_registry_id: project_registry.key(),
            old_status,
            new_status,
            reason,
            updated_by: ctx.accounts.authority.key(),
        });

        Ok(())
    }

    pub fn add_project_metadata(
        ctx: Context<AddProjectMetadata>,
        metadata_type: MetadataType,
        metadata_uri: String,
        description: String,
    ) -> Result<()> {
        require!(metadata_uri.len() <= 200, ErrorCode::MetadataUriTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);

        let metadata = &mut ctx.accounts.project_metadata;
        metadata.project_registry = ctx.accounts.project_registry.key();
        metadata.metadata_type = metadata_type;
        metadata.metadata_uri = metadata_uri;
        metadata.description = description;
        metadata.added_at = Clock::get()?.unix_timestamp;
        metadata.bump = *ctx.bumps.get("project_metadata").unwrap();

        emit!(ProjectMetadataAdded {
            metadata_id: metadata.key(),
            project_registry: metadata.project_registry,
            metadata_type,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Registry::INIT_SPACE,
        seeds = [b"registry", authority.key().as_ref()],
        bump
    )]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(project_id: String)]
pub struct RegisterProject<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProjectRegistry::INIT_SPACE,
        seeds = [b"project_registry", registry.key().as_ref(), project_id.as_bytes()],
        bump
    )]
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut, constraint = registry.authority == authority.key())]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueCredits<'info> {
    #[account(mut)]
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(
        init,
        payer = authority,
        space = 8 + CreditIssuance::INIT_SPACE,
        seeds = [b"credit_issuance", project_registry.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub credit_issuance: Account<'info, CreditIssuance>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub credit_mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is the recipient of the credits
    pub recipient: UncheckedAccount<'info>,
    #[account(mut, constraint = registry.authority == authority.key())]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferCredits<'info> {
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(
        init,
        payer = from_owner,
        space = 8 + TransferRecord::INIT_SPACE,
        seeds = [b"transfer_record", project_registry.key().as_ref(), from_owner.key().as_ref()],
        bump
    )]
    pub transfer_record: Account<'info, TransferRecord>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_owner: Signer<'info>,
    /// CHECK: This is the recipient of the transfer
    pub to_owner: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RetireCredits<'info> {
    #[account(mut)]
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(
        init,
        payer = owner,
        space = 8 + CreditRetirement::INIT_SPACE,
        seeds = [b"credit_retirement", project_registry.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub credit_retirement: Account<'info, CreditRetirement>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub credit_mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(batch_id: String)]
pub struct CreateBatch<'info> {
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(
        init,
        payer = authority,
        space = 8 + CreditBatch::INIT_SPACE,
        seeds = [b"credit_batch", project_registry.key().as_ref(), batch_id.as_bytes()],
        bump
    )]
    pub credit_batch: Account<'info, CreditBatch>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProjectStatus<'info> {
    #[account(mut)]
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(constraint = registry.authority == authority.key())]
    pub registry: Account<'info, Registry>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddProjectMetadata<'info> {
    pub project_registry: Account<'info, ProjectRegistry>,
    #[account(
        init,
        payer = authority,
        space = 8 + ProjectMetadata::INIT_SPACE,
        seeds = [b"project_metadata", project_registry.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub project_metadata: Account<'info, ProjectMetadata>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Registry {
    pub authority: Pubkey,
    #[max_len(64)]
    pub registry_name: String,
    #[max_len(200)]
    pub base_uri: String,
    pub total_credits_issued: u64,
    pub total_credits_retired: u64,
    pub total_projects: u64,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ProjectRegistry {
    #[max_len(32)]
    pub project_id: String,
    pub vintage_year: u16,
    #[max_len(100)]
    pub methodology: String,
    #[max_len(3)]
    pub country_code: String,
    pub project_developer: Pubkey,
    pub registry: Pubkey,
    pub total_issued: u64,
    pub total_retired: u64,
    pub status: ProjectRegistryStatus,
    pub registered_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditIssuance {
    pub project_registry: Pubkey,
    #[max_len(20)]
    pub serial_number_prefix: String,
    pub quantity: u64,
    pub issuance_date: i64,
    pub issued_to: Pubkey,
    pub status: IssuanceStatus,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TransferRecord {
    pub from_owner: Pubkey,
    pub to_owner: Pubkey,
    pub project_registry: Pubkey,
    pub quantity: u64,
    #[max_len(200)]
    pub transfer_reason: String,
    pub transferred_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditRetirement {
    pub owner: Pubkey,
    pub project_registry: Pubkey,
    pub quantity: u64,
    #[max_len(200)]
    pub retirement_reason: String,
    #[max_len(100)]
    pub beneficiary: String,
    pub retired_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditBatch {
    #[max_len(32)]
    pub batch_id: String,
    pub project_registry: Pubkey,
    pub vintage_start: i64,
    pub vintage_end: i64,
    #[max_len(200)]
    pub monitoring_report_uri: String,
    pub total_credits: u64,
    pub available_credits: u64,
    pub status: BatchStatus,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ProjectMetadata {
    pub project_registry: Pubkey,
    pub metadata_type: MetadataType,
    #[max_len(200)]
    pub metadata_uri: String,
    #[max_len(500)]
    pub description: String,
    pub added_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ProjectRegistryStatus {
    Active,
    Suspended,
    Terminated,
    UnderReview,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum IssuanceStatus {
    Active,
    Cancelled,
    Transferred,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum BatchStatus {
    Pending,
    Approved,
    Issued,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum MetadataType {
    ProjectDocument,
    MonitoringReport,
    VerificationReport,
    Photo,
    Video,
    Other,
}

#[event]
pub struct RegistryInitialized {
    pub registry_id: Pubkey,
    pub authority: Pubkey,
    pub registry_name: String,
}

#[event]
pub struct ProjectRegistered {
    pub project_registry_id: Pubkey,
    pub project_id: String,
    pub vintage_year: u16,
    pub project_developer: Pubkey,
}

#[event]
pub struct CreditsIssued {
    pub issuance_id: Pubkey,
    pub project_registry: Pubkey,
    pub serial_number_prefix: String,
    pub quantity: u64,
    pub recipient: Pubkey,
}

#[event]
pub struct CreditsTransferred {
    pub transfer_id: Pubkey,
    pub from_owner: Pubkey,
    pub to_owner: Pubkey,
    pub quantity: u64,
    pub reason: String,
}

#[event]
pub struct CreditsRetired {
    pub retirement_id: Pubkey,
    pub owner: Pubkey,
    pub project_registry: Pubkey,
    pub quantity: u64,
    pub reason: String,
}

#[event]
pub struct BatchCreated {
    pub batch_id: Pubkey,
    pub project_registry: Pubkey,
    pub batch_identifier: String,
    pub vintage_start: i64,
    pub vintage_end: i64,
}

#[event]
pub struct ProjectStatusUpdated {
    pub project_registry_id: Pubkey,
    pub old_status: ProjectRegistryStatus,
    pub new_status: ProjectRegistryStatus,
    pub reason: String,
    pub updated_by: Pubkey,
}

#[event]
pub struct ProjectMetadataAdded {
    pub metadata_id: Pubkey,
    pub project_registry: Pubkey,
    pub metadata_type: MetadataType,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Registry name too long")]
    RegistryNameTooLong,
    #[msg("Base URI too long")]
    BaseUriTooLong,
    #[msg("Project ID too long")]
    ProjectIdTooLong,
    #[msg("Methodology too long")]
    MethodologyTooLong,
    #[msg("Country code too long")]
    CountryCodeTooLong,
    #[msg("Invalid vintage year")]
    InvalidVintageYear,
    #[msg("Serial number prefix too long")]
    SerialNumberPrefixTooLong,
    #[msg("Invalid quantity")]
    InvalidQuantity,
    #[msg("Invalid issuance date")]
    InvalidIssuanceDate,
    #[msg("Transfer reason too long")]
    TransferReasonTooLong,
    #[msg("Retirement reason too long")]
    RetirementReasonTooLong,
    #[msg("Beneficiary too long")]
    BeneficiaryTooLong,
    #[msg("Batch ID too long")]
    BatchIdTooLong,
    #[msg("Invalid vintage period")]
    InvalidVintagePeriod,
    #[msg("Monitoring report URI too long")]
    MonitoringReportUriTooLong,
    #[msg("Reason too long")]
    ReasonTooLong,
    #[msg("Metadata URI too long")]
    MetadataUriTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
}