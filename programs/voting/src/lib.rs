use anchor_lang::prelude::*;

declare_id!("GYDa6fTek7nz1T5atPzgpHpRHMFhZaWryHfUvoRi1bne");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>, _poll_id: u64, name: String, description: String, start: u64, end:u64) -> Result<()> {
        let poll = & mut ctx.accounts.poll_account;
        poll.poll_description = description;
        poll.poll_name = name;
        poll.poll_start_time = start;
        poll.poll_end_time = end;
        Ok(())
    }

    pub fn init_candidate(ctx: Context<InitCandidate>, _poll_id: u64, name: String,) -> Result<()> {
        let candidate = & mut ctx.accounts.candidate_account;
        candidate.candidate_name = name;
        ctx.accounts.poll_account.poll_option_index += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let candidate_account = & mut ctx.accounts.candidate_account;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > (ctx.accounts.poll_account.poll_end_time as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

         if current_time < (ctx.accounts.poll_account.poll_start_time as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate_account.votes += 1;

        Ok(())
    }
}


//following lines of instructions tell that these are the accounts that you're going to interact with and how you're going to interact with it
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account (
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,
    pub system_program: Program<'info, System>
}

//candidate account
#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account (
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        //pdas
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account (
        mut,
        //pdas
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}


#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_start_time: u64,
    pub poll_end_time: u64,
    pub poll_option_index: u64
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub votes: u64
}

//error code macro
#[error_code] 
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}