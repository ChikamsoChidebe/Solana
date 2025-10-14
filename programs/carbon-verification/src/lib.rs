use anchor_lang::prelude::*;

declare_id!("CarbVerify11111111111111111111111111111111");

#[program]
pub mod carbon_verification {
    use super::*;

    pub fn initialize_verifier(
        ctx: Context<InitializeVerifier>,
        verifier_name: String,
        certification_level: CertificationLevel,
        accreditation_body: String,
    ) -> Result<()> {
        require!(verifier_name.len() <= 64, ErrorCode::VerifierNameTooLong);
        require!(accreditation_body.len() <= 64, ErrorCode::AccreditationBodyTooLong);

        let verifier = &mut ctx.accounts.verifier;
        verifier.authority = ctx.accounts.authority.key();
        verifier.verifier_name = verifier_name.clone();
        verifier.certification_level = certification_level;
        verifier.accreditation_body = accreditation_body;
        verifier.is_active = true;
        verifier.total_projects_verified = 0;
        verifier.total_credits_verified = 0;
        verifier.created_at = Clock::get()?.unix_timestamp;
        verifier.bump = *ctx.bumps.get("verifier").unwrap();

        emit!(VerifierInitialized {
            verifier_id: verifier.key(),
            authority: verifier.authority,
            verifier_name,
            certification_level,
        });

        Ok(())
    }

    pub fn submit_verification_request(
        ctx: Context<SubmitVerificationRequest>,
        project_key: Pubkey,
        verification_type: VerificationType,
        documentation_uri: String,
        estimated_credits: u64,
    ) -> Result<()> {
        require!(documentation_uri.len() <= 200, ErrorCode::DocumentationUriTooLong);
        require!(estimated_credits > 0, ErrorCode::InvalidCreditAmount);

        let request = &mut ctx.accounts.verification_request;
        request.project = project_key;
        request.requester = ctx.accounts.requester.key();
        request.verifier = ctx.accounts.verifier.key();
        request.verification_type = verification_type;
        request.documentation_uri = documentation_uri.clone();
        request.estimated_credits = estimated_credits;
        request.status = VerificationStatus::Pending;
        request.submitted_at = Clock::get()?.unix_timestamp;
        request.bump = *ctx.bumps.get("verification_request").unwrap();

        emit!(VerificationRequestSubmitted {
            request_id: request.key(),
            project: project_key,
            requester: request.requester,
            verifier: request.verifier,
            verification_type,
        });

        Ok(())
    }

    pub fn conduct_verification(
        ctx: Context<ConductVerification>,
        verified_credits: u64,
        verification_notes: String,
        compliance_score: u8,
    ) -> Result<()> {
        require!(verification_notes.len() <= 500, ErrorCode::VerificationNotesTooLong);
        require!(compliance_score <= 100, ErrorCode::InvalidComplianceScore);

        let request = &mut ctx.accounts.verification_request;
        require!(request.status == VerificationStatus::Pending, ErrorCode::RequestNotPending);

        let verification = &mut ctx.accounts.verification_result;
        verification.request = request.key();
        verification.verifier = ctx.accounts.verifier.key();
        verification.project = request.project;
        verification.verified_credits = verified_credits;
        verification.verification_notes = verification_notes.clone();
        verification.compliance_score = compliance_score;
        verification.methodology_used = request.verification_type;
        verification.verified_at = Clock::get()?.unix_timestamp;
        verification.is_valid = true;
        verification.bump = *ctx.bumps.get("verification_result").unwrap();

        request.status = VerificationStatus::Completed;
        request.completed_at = Some(Clock::get()?.unix_timestamp);

        let verifier = &mut ctx.accounts.verifier;
        verifier.total_projects_verified += 1;
        verifier.total_credits_verified += verified_credits;

        emit!(VerificationCompleted {
            verification_id: verification.key(),
            request_id: request.key(),
            project: request.project,
            verifier: verification.verifier,
            verified_credits,
            compliance_score,
        });

        Ok(())
    }

    pub fn challenge_verification(
        ctx: Context<ChallengeVerification>,
        challenge_reason: String,
        evidence_uri: String,
    ) -> Result<()> {
        require!(challenge_reason.len() <= 500, ErrorCode::ChallengeReasonTooLong);
        require!(evidence_uri.len() <= 200, ErrorCode::EvidenceUriTooLong);

        let challenge = &mut ctx.accounts.challenge;
        challenge.verification = ctx.accounts.verification_result.key();
        challenge.challenger = ctx.accounts.challenger.key();
        challenge.challenge_reason = challenge_reason.clone();
        challenge.evidence_uri = evidence_uri;
        challenge.status = ChallengeStatus::Open;
        challenge.submitted_at = Clock::get()?.unix_timestamp;
        challenge.bump = *ctx.bumps.get("challenge").unwrap();

        let verification = &mut ctx.accounts.verification_result;
        verification.is_valid = false;

        emit!(VerificationChallenged {
            challenge_id: challenge.key(),
            verification_id: verification.key(),
            challenger: challenge.challenger,
            reason: challenge_reason,
        });

        Ok(())
    }

    pub fn resolve_challenge(
        ctx: Context<ResolveChallenge>,
        resolution: ChallengeResolution,
        resolution_notes: String,
    ) -> Result<()> {
        require!(resolution_notes.len() <= 500, ErrorCode::ResolutionNotesTooLong);

        let challenge = &mut ctx.accounts.challenge;
        require!(challenge.status == ChallengeStatus::Open, ErrorCode::ChallengeNotOpen);

        challenge.status = match resolution {
            ChallengeResolution::Upheld => ChallengeStatus::Upheld,
            ChallengeResolution::Rejected => ChallengeStatus::Rejected,
        };
        challenge.resolved_at = Some(Clock::get()?.unix_timestamp);
        challenge.resolution_notes = Some(resolution_notes.clone());

        let verification = &mut ctx.accounts.verification_result;
        verification.is_valid = matches!(resolution, ChallengeResolution::Rejected);

        emit!(ChallengeResolved {
            challenge_id: challenge.key(),
            verification_id: verification.key(),
            resolution,
            resolver: ctx.accounts.resolver.key(),
        });

        Ok(())
    }

    pub fn update_verifier_status(
        ctx: Context<UpdateVerifierStatus>,
        is_active: bool,
    ) -> Result<()> {
        let verifier = &mut ctx.accounts.verifier;
        let old_status = verifier.is_active;
        verifier.is_active = is_active;

        emit!(VerifierStatusUpdated {
            verifier_id: verifier.key(),
            old_status,
            new_status: is_active,
            updated_by: ctx.accounts.authority.key(),
        });

        Ok(())
    }

    pub fn create_verification_report(
        ctx: Context<CreateVerificationReport>,
        report_uri: String,
        methodology_details: String,
        sampling_approach: String,
    ) -> Result<()> {
        require!(report_uri.len() <= 200, ErrorCode::ReportUriTooLong);
        require!(methodology_details.len() <= 1000, ErrorCode::MethodologyDetailsTooLong);
        require!(sampling_approach.len() <= 500, ErrorCode::SamplingApproachTooLong);

        let report = &mut ctx.accounts.verification_report;
        report.verification = ctx.accounts.verification_result.key();
        report.verifier = ctx.accounts.verifier.key();
        report.report_uri = report_uri;
        report.methodology_details = methodology_details;
        report.sampling_approach = sampling_approach;
        report.created_at = Clock::get()?.unix_timestamp;
        report.bump = *ctx.bumps.get("verification_report").unwrap();

        emit!(VerificationReportCreated {
            report_id: report.key(),
            verification_id: report.verification,
            verifier: report.verifier,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVerifier<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Verifier::INIT_SPACE,
        seeds = [b"verifier", authority.key().as_ref()],
        bump
    )]
    pub verifier: Account<'info, Verifier>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitVerificationRequest<'info> {
    #[account(
        init,
        payer = requester,
        space = 8 + VerificationRequest::INIT_SPACE,
        seeds = [b"verification_request", project.as_ref(), requester.key().as_ref()],
        bump
    )]
    pub verification_request: Account<'info, VerificationRequest>,
    /// CHECK: This is the project pubkey being verified
    pub project: UncheckedAccount<'info>,
    pub verifier: Account<'info, Verifier>,
    #[account(mut)]
    pub requester: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConductVerification<'info> {
    #[account(mut)]
    pub verification_request: Account<'info, VerificationRequest>,
    #[account(
        init,
        payer = verifier_authority,
        space = 8 + VerificationResult::INIT_SPACE,
        seeds = [b"verification_result", verification_request.key().as_ref()],
        bump
    )]
    pub verification_result: Account<'info, VerificationResult>,
    #[account(mut, constraint = verifier.authority == verifier_authority.key())]
    pub verifier: Account<'info, Verifier>,
    #[account(mut)]
    pub verifier_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChallengeVerification<'info> {
    #[account(mut)]
    pub verification_result: Account<'info, VerificationResult>,
    #[account(
        init,
        payer = challenger,
        space = 8 + VerificationChallenge::INIT_SPACE,
        seeds = [b"challenge", verification_result.key().as_ref(), challenger.key().as_ref()],
        bump
    )]
    pub challenge: Account<'info, VerificationChallenge>,
    #[account(mut)]
    pub challenger: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveChallenge<'info> {
    #[account(mut)]
    pub challenge: Account<'info, VerificationChallenge>,
    #[account(mut)]
    pub verification_result: Account<'info, VerificationResult>,
    pub resolver: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateVerifierStatus<'info> {
    #[account(mut, constraint = verifier.authority == authority.key())]
    pub verifier: Account<'info, Verifier>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateVerificationReport<'info> {
    pub verification_result: Account<'info, VerificationResult>,
    #[account(
        init,
        payer = verifier_authority,
        space = 8 + VerificationReport::INIT_SPACE,
        seeds = [b"verification_report", verification_result.key().as_ref()],
        bump
    )]
    pub verification_report: Account<'info, VerificationReport>,
    #[account(constraint = verifier.authority == verifier_authority.key())]
    pub verifier: Account<'info, Verifier>,
    #[account(mut)]
    pub verifier_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Verifier {
    pub authority: Pubkey,
    #[max_len(64)]
    pub verifier_name: String,
    pub certification_level: CertificationLevel,
    #[max_len(64)]
    pub accreditation_body: String,
    pub is_active: bool,
    pub total_projects_verified: u64,
    pub total_credits_verified: u64,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VerificationRequest {
    pub project: Pubkey,
    pub requester: Pubkey,
    pub verifier: Pubkey,
    pub verification_type: VerificationType,
    #[max_len(200)]
    pub documentation_uri: String,
    pub estimated_credits: u64,
    pub status: VerificationStatus,
    pub submitted_at: i64,
    pub completed_at: Option<i64>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VerificationResult {
    pub request: Pubkey,
    pub verifier: Pubkey,
    pub project: Pubkey,
    pub verified_credits: u64,
    #[max_len(500)]
    pub verification_notes: String,
    pub compliance_score: u8,
    pub methodology_used: VerificationType,
    pub verified_at: i64,
    pub is_valid: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VerificationChallenge {
    pub verification: Pubkey,
    pub challenger: Pubkey,
    #[max_len(500)]
    pub challenge_reason: String,
    #[max_len(200)]
    pub evidence_uri: String,
    pub status: ChallengeStatus,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
    #[max_len(500)]
    pub resolution_notes: Option<String>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VerificationReport {
    pub verification: Pubkey,
    pub verifier: Pubkey,
    #[max_len(200)]
    pub report_uri: String,
    #[max_len(1000)]
    pub methodology_details: String,
    #[max_len(500)]
    pub sampling_approach: String,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum CertificationLevel {
    Basic,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum VerificationType {
    Initial,
    Periodic,
    PostImplementation,
    Surveillance,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum VerificationStatus {
    Pending,
    InProgress,
    Completed,
    Rejected,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ChallengeStatus {
    Open,
    Upheld,
    Rejected,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ChallengeResolution {
    Upheld,
    Rejected,
}

#[event]
pub struct VerifierInitialized {
    pub verifier_id: Pubkey,
    pub authority: Pubkey,
    pub verifier_name: String,
    pub certification_level: CertificationLevel,
}

#[event]
pub struct VerificationRequestSubmitted {
    pub request_id: Pubkey,
    pub project: Pubkey,
    pub requester: Pubkey,
    pub verifier: Pubkey,
    pub verification_type: VerificationType,
}

#[event]
pub struct VerificationCompleted {
    pub verification_id: Pubkey,
    pub request_id: Pubkey,
    pub project: Pubkey,
    pub verifier: Pubkey,
    pub verified_credits: u64,
    pub compliance_score: u8,
}

#[event]
pub struct VerificationChallenged {
    pub challenge_id: Pubkey,
    pub verification_id: Pubkey,
    pub challenger: Pubkey,
    pub reason: String,
}

#[event]
pub struct ChallengeResolved {
    pub challenge_id: Pubkey,
    pub verification_id: Pubkey,
    pub resolution: ChallengeResolution,
    pub resolver: Pubkey,
}

#[event]
pub struct VerifierStatusUpdated {
    pub verifier_id: Pubkey,
    pub old_status: bool,
    pub new_status: bool,
    pub updated_by: Pubkey,
}

#[event]
pub struct VerificationReportCreated {
    pub report_id: Pubkey,
    pub verification_id: Pubkey,
    pub verifier: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Verifier name too long")]
    VerifierNameTooLong,
    #[msg("Accreditation body name too long")]
    AccreditationBodyTooLong,
    #[msg("Documentation URI too long")]
    DocumentationUriTooLong,
    #[msg("Invalid credit amount")]
    InvalidCreditAmount,
    #[msg("Verification notes too long")]
    VerificationNotesTooLong,
    #[msg("Invalid compliance score")]
    InvalidComplianceScore,
    #[msg("Request not pending")]
    RequestNotPending,
    #[msg("Challenge reason too long")]
    ChallengeReasonTooLong,
    #[msg("Evidence URI too long")]
    EvidenceUriTooLong,
    #[msg("Resolution notes too long")]
    ResolutionNotesTooLong,
    #[msg("Challenge not open")]
    ChallengeNotOpen,
    #[msg("Report URI too long")]
    ReportUriTooLong,
    #[msg("Methodology details too long")]
    MethodologyDetailsTooLong,
    #[msg("Sampling approach too long")]
    SamplingApproachTooLong,
}