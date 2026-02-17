use anchor_lang::prelude::*;

declare_id!("YourProgramIDHereReplaceThis");

#[program]
pub mod brown402 {
    use super::*;

    pub fn create_template(
        ctx: Context<CreateTemplate>,
        title: String,
        content: String,
    ) -> Result<()> {
        let template = &mut ctx.accounts.template;
        template.author = *ctx.accounts.author.key;
        template.title = title;
        template.content = content;
        Ok(())
    }
}

#[account]
pub struct Template {
    pub author: Pubkey,
    pub title: String,
    pub content: String,
}

#[derive(Accounts)]
pub struct CreateTemplate<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + 32 + 4 + 100 + 4 + 500
    )]
    pub template: Account<'info, Template>,

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>,
}
