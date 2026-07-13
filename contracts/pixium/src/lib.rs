#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod types;
use types::{DataKey, Pixel, CANVAS_HEIGHT, CANVAS_WIDTH, COOLDOWN_SECONDS, PALETTE_SIZE};

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

    /// Places a pixel on the canvas as `player`.
    ///
    /// Requires `player`'s authorization, rejects out-of-bounds
    /// coordinates and out-of-palette colors, and enforces a cooldown
    /// between successive placements by the same player.
    pub fn place_pixel(env: Env, player: Address, x: u32, y: u32, color: u32) {
        player.require_auth();
        Self::require_in_bounds(x, y);
        Self::require_valid_color(color);
        Self::require_cooldown_elapsed(&env, &player);

        Self::write_pixel(&env, x, y, color, player.clone());
        Self::record_placement(&env, &player);
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

    fn require_valid_color(color: u32) {
        if color >= PALETTE_SIZE {
            panic!("color is not in the palette");
        }
    }

    fn require_cooldown_elapsed(env: &Env, player: &Address) {
        let key = DataKey::LastPlaced(player.clone());
        if let Some(last_placed) = env.storage().temporary().get::<_, u64>(&key) {
            let now = env.ledger().timestamp();
            if now - last_placed < COOLDOWN_SECONDS {
                panic!("cooldown has not elapsed since your last placement");
            }
        }
    }

    fn record_placement(env: &Env, player: &Address) {
        let key = DataKey::LastPlaced(player.clone());
        env.storage()
            .temporary()
            .set(&key, &env.ledger().timestamp());
    }

    /// Raw storage write, shared by `place_pixel`. Does not perform any
    /// authorization, bounds, or cooldown checks itself — callers are
    /// responsible for those.
    fn write_pixel(env: &Env, x: u32, y: u32, color: u32, owner: Address) {
        let key = DataKey::Pixel(x, y);
        env.storage()
            .persistent()
            .set(&key, &Pixel { color, owner });
    }
}

mod test;
