//#![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod event;
mod metadata;
mod storage_types;
mod internal_mint;
mod internal_burn;
//mod test;

pub use crate::token::contract::TokenClient;
pub use crate::token::contract::Token;
pub use crate::token::contract::TokenTrait;
pub use crate::token::internal_mint::internal_mint;
pub use crate::token::internal_burn::internal_burn;