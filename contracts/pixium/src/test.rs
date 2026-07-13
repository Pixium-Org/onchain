#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;

#[test]
fn get_pixel_returns_none_for_unpainted_cell() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.get_pixel(&0, &0), None);
}

#[test]
fn set_pixel_then_get_pixel_round_trips() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    client.set_pixel(&5, &10, &3, &owner);

    let pixel = client.get_pixel(&5, &10).unwrap();
    assert_eq!(pixel.color, 3);
    assert_eq!(pixel.owner, owner);
}

#[test]
fn set_pixel_overwrites_previous_value() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let first_owner = Address::generate(&env);
    let second_owner = Address::generate(&env);

    client.set_pixel(&0, &0, &1, &first_owner);
    client.set_pixel(&0, &0, &2, &second_owner);

    let pixel = client.get_pixel(&0, &0).unwrap();
    assert_eq!(pixel.color, 2);
    assert_eq!(pixel.owner, second_owner);
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
fn set_pixel_rejects_out_of_bounds_y() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    client.set_pixel(&0, &CANVAS_HEIGHT, &1, &owner);
}
