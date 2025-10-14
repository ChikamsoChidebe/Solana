use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("CarbMktpLace11111111111111111111111111111111");

#[program]
pub mod carbon_marketplace {
    use super::*;

    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        fee_percentage: u16,
        min_credit_amount: u64,
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_percentage = fee_percentage;
        marketplace.min_credit_amount = min_credit_amount;
        marketplace.total_credits_traded = 0;
        marketplace.total_volume = 0;
        marketplace.active_listings = 0;
        marketplace.verified_projects = 0;
        marketplace.bump = *ctx.bumps.get("marketplace").unwrap();
        
        emit!(MarketplaceInitialized {
            authority: marketplace.authority,
            fee_percentage,
            min_credit_amount,
        });
        
        Ok(())
    }

    pub fn create_carbon_project(
        ctx: Context<CreateCarbonProject>,
        project_id: String,
        project_name: String,
        project_type: ProjectType,
        location: String,
        estimated_credits: u64,
        verification_standard: VerificationStandard,
        metadata_uri: String,
    ) -> Result<()> {
        require!(project_id.len() <= 32, ErrorCode::ProjectIdTooLong);
        require!(project_name.len() <= 64, ErrorCode::ProjectNameTooLong);
        require!(location.len() <= 64, ErrorCode::LocationTooLong);
        require!(metadata_uri.len() <= 200, ErrorCode::MetadataUriTooLong);

        let project = &mut ctx.accounts.project;
        project.project_id = project_id.clone();
        project.project_name = project_name.clone();
        project.project_type = project_type;
        project.developer = ctx.accounts.developer.key();
        project.location = location;
        project.estimated_credits = estimated_credits;
        project.issued_credits = 0;
        project.retired_credits = 0;
        project.verification_standard = verification_standard;
        project.status = ProjectStatus::Pending;
        project.created_at = Clock::get()?.unix_timestamp;
        project.metadata_uri = metadata_uri;
        project.bump = *ctx.bumps.get("project").unwrap();

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.verified_projects += 1;

        emit!(CarbonProjectCreated {
            project_id,
            developer: project.developer,
            project_type,
            estimated_credits,
        });

        Ok(())
    }

    pub fn list_credits(
        ctx: Context<ListCredits>,
        amount: u64,
        price_per_credit: u64,
        expiry_time: i64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(price_per_credit > 0, ErrorCode::InvalidPrice);
        require!(expiry_time > Clock::get()?.unix_timestamp, ErrorCode::InvalidExpiryTime);

        let project = &ctx.accounts.project;
        require!(project.status == ProjectStatus::Verified, ErrorCode::ProjectNotVerified);
        
        let available_credits = project.issued_credits - project.retired_credits;
        require!(amount <= available_credits, ErrorCode::InsufficientCredits);

        let listing = &mut ctx.accounts.listing;
        listing.project = ctx.accounts.project.key();
        listing.seller = ctx.accounts.seller.key();
        listing.amount = amount;
        listing.price_per_credit = price_per_credit;
        listing.total_value = amount.checked_mul(price_per_credit).unwrap();
        listing.status = ListingStatus::Active;
        listing.created_at = Clock::get()?.unix_timestamp;
        listing.expiry_time = expiry_time;
        listing.bump = *ctx.bumps.get("listing").unwrap();

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.active_listings += 1;

        emit!(CreditsListed {
            listing_id: listing.key(),
            project: project.key(),
            seller: listing.seller,
            amount,
            price_per_credit,
        });

        Ok(())
    }

    pub fn purchase_credits(
        ctx: Context<PurchaseCredits>,
        amount: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.status == ListingStatus::Active, ErrorCode::ListingNotActive);
        require!(amount <= listing.amount, ErrorCode::InsufficientCreditsInListing);
        require!(Clock::get()?.unix_timestamp < listing.expiry_time, ErrorCode::ListingExpired);

        let total_cost = amount.checked_mul(listing.price_per_credit).unwrap();
        let marketplace = &ctx.accounts.marketplace;
        let fee_amount = total_cost.checked_mul(marketplace.fee_percentage as u64).unwrap() / 10000;
        let seller_amount = total_cost.checked_sub(fee_amount).unwrap();

        let transfer_to_seller = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_seller),
            seller_amount,
        )?;

        let transfer_fee = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.marketplace_fee_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_fee),
            fee_amount,
        )?;

        listing.amount = listing.amount.checked_sub(amount).unwrap();
        if listing.amount == 0 {
            listing.status = ListingStatus::Sold;
        }

        let purchase = &mut ctx.accounts.purchase;
        purchase.listing = listing.key();
        purchase.buyer = ctx.accounts.buyer.key();
        purchase.seller = listing.seller;
        purchase.amount = amount;
        purchase.price_per_credit = listing.price_per_credit;
        purchase.total_paid = total_cost;
        purchase.fee_paid = fee_amount;
        purchase.purchased_at = Clock::get()?.unix_timestamp;
        purchase.bump = *ctx.bumps.get("purchase").unwrap();

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_credits_traded += amount;
        marketplace.total_volume += total_cost;

        emit!(CreditsPurchased {
            purchase_id: purchase.key(),
            buyer: purchase.buyer,
            seller: purchase.seller,
            amount,
            total_cost,
        });

        Ok(())
    }

    pub fn retire_credits(
        ctx: Context<RetireCredits>,
        amount: u64,
        retirement_reason: String,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(retirement_reason.len() <= 200, ErrorCode::RetirementReasonTooLong);

        let retirement = &mut ctx.accounts.retirement;
        retirement.owner = ctx.accounts.owner.key();
        retirement.project = ctx.accounts.project.key();
        retirement.amount = amount;
        retirement.retirement_reason = retirement_reason.clone();
        retirement.retired_at = Clock::get()?.unix_timestamp;
        retirement.bump = *ctx.bumps.get("retirement").unwrap();

        let project = &mut ctx.accounts.project;
        project.retired_credits += amount;

        emit!(CreditsRetired {
            retirement_id: retirement.key(),
            owner: retirement.owner,
            project: project.key(),
            amount,
            reason: retirement_reason,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(project_id: String)]
pub struct CreateCarbonProject<'info> {
    #[account(
        init,
        payer = developer,
        space = 8 + CarbonProject::INIT_SPACE,
        seeds = [b"project", project_id.as_bytes()],
        bump
    )]
    pub project: Account<'info, CarbonProject>,
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub developer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListCredits<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + CreditListing::INIT_SPACE,
        seeds = [b"listing", project.key().as_ref(), seller.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, CreditListing>,
    #[account(mut)]
    pub project: Account<'info, CarbonProject>,
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseCredits<'info> {
    #[account(mut)]
    pub listing: Account<'info, CreditListing>,
    #[account(
        init,
        payer = buyer,
        space = 8 + CreditPurchase::INIT_SPACE,
        seeds = [b"purchase", listing.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub purchase: Account<'info, CreditPurchase>,
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub marketplace_fee_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RetireCredits<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + CreditRetirement::INIT_SPACE,
        seeds = [b"retirement", project.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub retirement: Account<'info, CreditRetirement>,
    #[account(mut)]
    pub project: Account<'info, CarbonProject>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    pub min_credit_amount: u64,
    pub total_credits_traded: u64,
    pub total_volume: u64,
    pub active_listings: u64,
    pub verified_projects: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CarbonProject {
    #[max_len(32)]
    pub project_id: String,
    #[max_len(64)]
    pub project_name: String,
    pub project_type: ProjectType,
    pub developer: Pubkey,
    #[max_len(64)]
    pub location: String,
    pub estimated_credits: u64,
    pub issued_credits: u64,
    pub retired_credits: u64,
    pub verification_standard: VerificationStandard,
    pub status: ProjectStatus,
    pub created_at: i64,
    pub verified_at: Option<i64>,
    #[max_len(200)]
    pub metadata_uri: String,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditListing {
    pub project: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub price_per_credit: u64,
    pub total_value: u64,
    pub status: ListingStatus,
    pub created_at: i64,
    pub expiry_time: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditPurchase {
    pub listing: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub price_per_credit: u64,
    pub total_paid: u64,
    pub fee_paid: u64,
    pub purchased_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditRetirement {
    pub owner: Pubkey,
    pub project: Pubkey,
    pub amount: u64,
    #[max_len(200)]
    pub retirement_reason: String,
    pub retired_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum VerificationStandard {
    VCS,
    CDM,
    GoldStandard,
    CAR,
    ACR,
    Plan,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ProjectStatus {
    Pending,
    Verified,
    Suspended,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

#[event]
pub struct MarketplaceInitialized {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    pub min_credit_amount: u64,
}

#[event]
pub struct CarbonProjectCreated {
    pub project_id: String,
    pub developer: Pubkey,
    pub project_type: ProjectType,
    pub estimated_credits: u64,
}

#[event]
pub struct CreditsListed {
    pub listing_id: Pubkey,
    pub project: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub price_per_credit: u64,
}

#[event]
pub struct CreditsPurchased {
    pub purchase_id: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub total_cost: u64,
}

#[event]
pub struct CreditsRetired {
    pub retirement_id: Pubkey,
    pub owner: Pubkey,
    pub project: Pubkey,
    pub amount: u64,
    pub reason: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Project ID too long")]
    ProjectIdTooLong,
    #[msg("Project name too long")]
    ProjectNameTooLong,
    #[msg("Location too long")]
    LocationTooLong,
    #[msg("Metadata URI too long")]
    MetadataUriTooLong,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid expiry time")]
    InvalidExpiryTime,
    #[msg("Project not verified")]
    ProjectNotVerified,
    #[msg("Insufficient credits")]
    InsufficientCredits,
    #[msg("Listing not active")]
    ListingNotActive,
    #[msg("Insufficient credits in listing")]
    InsufficientCreditsInListing,
    #[msg("Listing expired")]
    ListingExpired,
    #[msg("Retirement reason too long")]
    RetirementReasonTooLong,
}