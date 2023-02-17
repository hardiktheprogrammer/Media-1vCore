mod errors;
mod instructions;
mod interfaces;
mod structs;
use anchor_lang::prelude::*;
use instructions::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};

// const SEED: &str = "Template";

declare_id!("R9PatsTac3Y3UpC7ihYMMgzAQCe1tXnVvkSQ8DtLWUc");
