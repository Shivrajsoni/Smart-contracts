use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("73ZZu7KAnMz9yXGM3AB65QtfAYT47H8oY3bbVeqXm8Aw");

#[program]
pub mod sol_anchor {
    use super::*;
    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        lottery.players = Vec::new();
        lottery.bump = *ctx.bumps.get("lottery").unwrap();
        lottery.start_time = Clock::get()?.unix_timestamp;
        lottery.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.lottery.key(),
            1000_000_000,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.lottery.to_account_info(),
            ],
        )?;
        // add player to the lottery
        lottery.players.push(ctx.accounts.player.key());
        msg!(
            "Player {}. entered to the lottery! ",
            ctx.accounts.player.key()
        );
    }

    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        require!(lottery.players.len() > 0, LotteryError::NoPlayers);

        // randomizing logic
        let clock = Clock::get();
        let seed = clock.unix_timestamp as u64 + clock.slot;
        let winner_index = (seed % lottery.players.len() as u64) as usize;
        let winner = lottery.players[winner_index];

        msg!("🎉 Winner is: {}", winner);

        let total_balance = ctx.accounts.lottery.to_account_info().lamports();
        **ctx
            .accounts
            .lottery
            .to_account_info()
            .try_borrow_mut_lamports()? -= total_balance;
        **ctx
            .accounts
            .winner
            .to_account_info()
            .try_borrow_mut_lamports()? += total_balance;
        lottery.players.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [b"lottery"],
        bump = lottery.bump,
    )]
    pub lottery: Account<'info, LotteryState>,
    pub system: Account<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut,address = lottery.key())]
    pub lottery: Account<'info, LotteryState>,

    #[account(mut, address = lottery.authority)] // <- Only admin can call
    pub authority: Signer<'info>, //admin
    #[account(mut)]
    pub winner: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instructions()]
pub struct InitializeLottery<'info> {
    #[account(
        init,
        seeds = [b"lottery"],
        bump,
        payer = signer,
        space = 8 + 4 + (32*10) + 1; // 10 players max, 1 byte bump
    )]
    pub lottery: Account<'info, LotteryState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LotteryState {
    pub players: Vec<Pubkey>,
    pub bump: u8,
    pub start_time: i64, // <-- add this
    pub authority: Pubkey,
}

#[error_code]
pub enum LotteryError {
    #[msg("No players in the lottery.")]
    NoPlayers,
}
