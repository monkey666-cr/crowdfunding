use anchor_lang::prelude::entrypoint::ProgramResult;
use anchor_lang::prelude::*;

declare_id!("BrpDKtbu9Z6dHZteU1sQc8644QDZCgjNKU1KUNiGhrQQ");

#[program]
pub mod crowdfunding2 {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.target_amount = 0;
        campaign.manager = ctx.accounts.manager.key();
        Ok(())
    }

    pub fn withdraw(ctx: Context<WithDraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        if ctx.accounts.manager.key() != campaign.manager {
            return Err(ProgramError::IncorrectProgramId);
        }
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx
            .accounts
            .manager
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn donate(ctx: Context<WithDraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        // **ctx
        //     .accounts
        //     .manager
        //     .to_account_info()
        //     .try_borrow_mut_lamports()? -= amount;
        // **campaign.to_account_info().try_borrow_mut_lamports()? += amount;
        // campaign.target_amount += amount;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.manager.key(),
            &campaign.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.manager.to_account_info(),
                campaign.to_account_info(),
            ],
        )?;
        campaign.target_amount += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = manager, space = 8 + Campaign::INIT_SPACE)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub manager: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithDraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub manager: Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub manager: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    #[max_len(100)]
    pub name: String,
    #[max_len(100)]
    pub description: String,
    pub target_amount: u64,
    pub manager: Pubkey,
}
