use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 300 characters long maximum.")]
    ContentTooLong,
    #[msg("The provided comment should be 150 characters long maximum.")]
    CommentTooLong,
}