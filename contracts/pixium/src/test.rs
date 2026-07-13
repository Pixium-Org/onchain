#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Events as _, Ledger};
use soroban_sdk::Event as _;

fn set_timestamp(env: &Env, ts: u64) {
    env.ledger().with_mut(|li| li.timestamp = ts);
}

#[test]
fn get_pixel_returns_none_for_unpainted_cell() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.get_pixel(&0, &0), None);
}

#[test]
fn place_pixel_then_get_pixel_round_trips() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    client.place_pixel(&player, &5, &10, &3);

    // Check events right after the call that emits them
    let expected = PixelPlaced {
        owner: player.clone(),
        x: 5,
        y: 10,
        color: 3,
    };
    assert_eq!(
        env.events().all(),
        std::vec![expected.to_xdr(&env, &contract_id)],
    );

    let pixel = client.get_pixel(&5, &10).unwrap();
    assert_eq!(pixel.color, 3);
    assert_eq!(pixel.owner, player);
}

#[test]
fn place_pixel_by_different_player_overwrites_previous_value() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let first_player = Address::generate(&env);
    let second_player = Address::generate(&env);

    client.place_pixel(&first_player, &0, &0, &1);
    client.place_pixel(&second_player, &0, &0, &2);

    let pixel = client.get_pixel(&0, &0).unwrap();
    assert_eq!(pixel.color, 2);
    assert_eq!(pixel.owner, second_player);
}

#[test]
#[should_panic(expected = "pixel coordinates out of bounds")]
fn get_pixel_rejects_out_of_bounds_x() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.get_pixel(&CANVAS_WIDTH, &0);
}

#[test]
#[should_panic(expected = "pixel coordinates out of bounds")]
fn place_pixel_rejects_out_of_bounds_y() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    client.place_pixel(&player, &0, &CANVAS_HEIGHT, &1);
}

#[test]
#[should_panic(expected = "color is not in the palette")]
fn place_pixel_rejects_color_outside_palette() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    client.place_pixel(&player, &0, &0, &PALETTE_SIZE);
}

#[test]
#[should_panic(expected = "cooldown has not elapsed")]
fn place_pixel_enforces_cooldown_for_same_player() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    set_timestamp(&env, 1_000);
    client.place_pixel(&player, &0, &0, &1);

    set_timestamp(&env, 1_000 + COOLDOWN_SECONDS - 1);
    client.place_pixel(&player, &1, &1, &2);
}

#[test]
fn place_pixel_allows_placement_after_cooldown_elapses() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    set_timestamp(&env, 1_000);
    client.place_pixel(&player, &0, &0, &1);

    set_timestamp(&env, 1_000 + COOLDOWN_SECONDS);
    client.place_pixel(&player, &1, &1, &2);

    let pixel = client.get_pixel(&1, &1).unwrap();
    assert_eq!(pixel.color, 2);
    assert_eq!(pixel.owner, player);
}

#[test]
fn place_pixel_cooldown_is_tracked_per_player() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let first_player = Address::generate(&env);
    let second_player = Address::generate(&env);

    set_timestamp(&env, 1_000);
    client.place_pixel(&first_player, &0, &0, &1);
    // A different player is unaffected by first_player's cooldown.
    client.place_pixel(&second_player, &1, &1, &2);

    let pixel = client.get_pixel(&1, &1).unwrap();
    assert_eq!(pixel.color, 2);
    assert_eq!(pixel.owner, second_player);
}
