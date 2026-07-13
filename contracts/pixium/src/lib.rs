#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod types;
use types::{DataKey, Pixel, CANVAS_HEIGHT, CANVAS_WIDTH};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Reads the current state of a pixel. Returns `None` if the pixel
    /// has never been painted.
    pub fn get_pixel(env: Env, x: u32, y: u32) -> Option<Pixel> {
        Self::require_in_bounds(x, y);
        env.storage().persistent().get(&DataKey::Pixel(x, y))
    }

    /// Writes a pixel's color and owner directly to storage.
    ///
    /// This is a raw storage setter only: it does not yet enforce
    /// authorization, cooldowns, or palette validity. Those rules land
    /// in a follow-up change alongside the public `place_pixel` entry
    /// point.
    pub fn set_pixel(env: Env, x: u32, y: u32, color: u32, owner: Address) {
        Self::require_in_bounds(x, y);
        let key = DataKey::Pixel(x, y);
        env.storage().persistent().set(&key, &Pixel { color, owner });
    }
}

// Internal helpers kept outside the #[contractimpl] block so they are not
// exposed as part of the contract's public interface.
impl Contract {
    fn require_in_bounds(x: u32, y: u32) {
        if x >= CANVAS_WIDTH || y >= CANVAS_HEIGHT {
            panic!("pixel coordinates out of bounds");
        }
    }
}

mod test;
