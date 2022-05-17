use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use state::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vibe_programs {
    use super::*;

    // Initialize a Vibe
    pub fn init_vibe(
        ctx: Context<InitializeVibe>, 
        vibe_content: Vec<VibeContent>, 
        allowed_comments: bool
    ) -> Result<()> {
        instructions::init_vibe::handler(ctx, vibe_content, allowed_comments)
    }

}

#[derive(Accounts)]
pub struct Initialize {}
