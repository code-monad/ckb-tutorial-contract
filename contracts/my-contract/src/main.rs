#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use ckb_std::ckb_types::prelude::Unpack;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

// First, let us define some error codes:
#[repr(i8)]
pub enum MyContractError {
    InvalidTransaction = -1, // if we faild in load_cell or other load statement, return this error
    InvalidFirstInput = -2,
    InvalidFirstOutput = -3,
    InvalidFirstGroupInput = -4,
    InvalidFirstContractOutput = -5,
}

impl From<MyContractError> for i8 {
    fn from(err: MyContractError) -> i8 {
        err as i8
    }
}

// we import the high level API for a cleaner code
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{
    load_cell,
    load_cell_type_hash,
    load_script_hash,
    QueryIter, // a iterator factory
};

pub fn program_entry() -> i8 {
    // first rule, we check the first input and first output
    // rule: capacity of first input must be greater than 100ckb,
    // and capacity of first output must be less than 100ckb

    let first_input = load_cell(0, Source::Input).expect("load input cell failed");
    let first_input_capacity: u64 = first_input.capacity().unpack(); // we need to unpack the value since this is a encoded field
    if first_input_capacity <= 100 * 10u64.pow(8) {
        return MyContractError::InvalidFirstInput.into();
    }

    let first_output = load_cell(0, Source::Output).expect("load output cell failed");
    let first_output_capacity: u64 = first_output.capacity().unpack();
    if first_output_capacity >= 100 * 10u64.pow(8) {
        return MyContractError::InvalidFirstOutput.into();
    }

    // second rule, we check the first group input and first group output
    let first_group_input = load_cell(0, Source::GroupInput)
        .expect("load group_input cell failed, maybe no cell refereced this contract?");
    let first_group_input_capacity: u64 = first_group_input.capacity().unpack();

    // verify if the first group input capacity is less than 100ckb
    if first_group_input_capacity >= 100 * 10u64.pow(8) {
        return MyContractError::InvalidFirstGroupInput.into();
    }

    // now let's find the first output cell that USE/CALL our contract

    let script_hash = load_script_hash().expect("load script hash failed");

    let query = QueryIter::new(load_cell_type_hash, Source::Output) // we use QueryIter to iterate all output cells, apply the load_cell_type_hash function to each cell
        .position(|cell_type_hash| cell_type_hash == Some(script_hash)); // find the first cell that has the same type hash as our contract

    // and this is equivalent to the following code:
    // for i in 0.. {
    //     let cell_type_hash =
    //         load_cell_type_hash(i, Source::Output).expect("load cell type hash failed");
    //     if cell_type_hash == Some(script_hash) {
    //         break;
    //     }
    // }

    // no output cell use this contract
    if query.is_none() {
        return MyContractError::InvalidTransaction.into();
    }

    let first_contract_output = load_cell(query.unwrap(), Source::Output)
        .expect("load group_output cell failed, maybe no cell refereced this contract?");

    let first_contract_output_capacity: u64 = first_contract_output.capacity().unpack();

    if first_contract_output_capacity <= 100 * 10u64.pow(8) {
        return MyContractError::InvalidFirstContractOutput.into();
    }

    // verify if the first group output capacity is greater than 100ckb

    0
}
