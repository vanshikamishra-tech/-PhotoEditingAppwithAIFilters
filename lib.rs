#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short, log};

const FILTER_COUNT: Symbol = symbol_short!("FILT_CT");

#[contracttype]
#[derive(Clone)]
pub struct PhotoFilter{
    pub photo_id: u64,
    pub user: String,
    pub filter_applied: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum PhotoBook {
    Filtered(u64),
}

#[contract]
pub struct PhotoEditorContract;

#[contractimpl]
impl PhotoEditorContract {
    pub fn apply_filter(env: Env, user: String, filter_applied: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&FILTER_COUNT).unwrap_or(0);
        count += 1;

        let photo_filter = PhotoFilter {
            photo_id: count,
            user,
            filter_applied,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&PhotoBook::Filtered(count), &photo_filter);
        env.storage().instance().set(&FILTER_COUNT, &count);

        log!(&env, "Filter '{}' applied to Photo ID {}", photo_filter.filter_applied, count);
        count
    }

    pub fn view_photo(env: Env, photo_id: u64) -> PhotoFilter {
        env.storage().instance().get(&PhotoBook::Filtered(photo_id)).unwrap_or(PhotoFilter {
            photo_id: 0,
            user: String::from_str(&env, "Not Found"),
            filter_applied: String::from_str(&env, "None"),
            timestamp: 0,
        })
    }

    pub fn total_photos_filtered(env: Env) -> u64 {
        env.storage().instance().get(&FILTER_COUNT).unwrap_or(0)
    }
}
