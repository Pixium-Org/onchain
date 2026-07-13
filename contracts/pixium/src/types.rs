use soroban_sdk::{contracttype, Address};

/// Canvas dimensions. Matches the MVP spec (1000x1000 shared grid).
pub const CANVAS_WIDTH: u32 = 1000;
pub const CANVAS_HEIGHT: u32 = 1000;

/// Number of colors in the fixed MVP palette. `color` values passed to
/// `place_pixel` must be `< PALETTE_SIZE`. Color voting (v0.5) will make
/// the palette dynamic; until then it's a fixed-size constant.
pub const PALETTE_SIZE: u32 = 16;

/// Minimum time, in seconds, a player must wait between pixel placements.
pub const COOLDOWN_SECONDS: u64 = 60;

/// A single cell on the canvas: a color (index into the active palette)
/// and the address of the player who last painted it.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pixel {
    pub color: u32,
    pub owner: Address,
}

/// Storage keys used by the contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Maps (x, y) -> Pixel. One storage entry per painted pixel.
    Pixel(u32, u32),
    /// Maps a player's address -> the ledger timestamp (seconds) of their
    /// last successful pixel placement. Used to enforce the cooldown.
    LastPlaced(Address),
}
