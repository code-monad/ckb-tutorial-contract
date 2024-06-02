// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for examples
use crate::{verify_and_dump_failed_tx, Loader, TestEnv};
use ckb_testtool::builtin::ALWAYS_SUCCESS;
use ckb_testtool::{
    ckb_types::{
        bytes::Bytes,
        core::{ScriptHashType, TransactionBuilder},
        h256,
        packed::{CellOutput, *},
        prelude::*,
    },
    context::Context,
};
const MAX_CYCLES: u64 = 10_000_000;

fn create_loader() -> Loader {
    Loader::with_test_env(match cfg!(debug_assertions) {
        true => TestEnv::Debug,
        false => TestEnv::Release,
    })
}

#[test]
fn test_my_contract() {
    let loader = create_loader();
    let mut context = Context::default();

    // load our contract and deploy a simulated cell
    let my_contract_binary = loader.load_binary("my-contract");
    let my_contract_out_point = context.deploy_cell(my_contract_binary);

    // load dummy lock script that will always pass
    let always_success_output_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    let always_success_lock_script = context
        .build_script_with_hash_type(
            &always_success_output_point,
            ScriptHashType::Type,
            Default::default(),
        )
        .expect("always_success_script");

    // prepare cell_deps
    let cell_deps = CellDep::new_builder()
        .out_point(my_contract_out_point.clone())
        .build();

    // prepare a transaction that uses our contract

    let type_script_for_our_cell = context
        .build_script(&my_contract_out_point, Default::default())
        .pack(); // we need to encode it to packed format

    let first_input = context.create_cell(
        CellOutput::new_builder()
            .capacity(101_00000000u64.pack())
            .lock(always_success_lock_script.clone())
            .build(),
        Bytes::default(),
    );
    // second input will use our contract
    let second_input = context.create_cell(
        CellOutput::new_builder()
            .capacity(101_00000000u64.pack())
            .type_(type_script_for_our_cell.clone())
            .lock(always_success_lock_script.clone())
            .build(),
        Bytes::default(),
    );

    let first_output = CellOutput::new_builder()
        .capacity(100_00000000.pack()) // 99 ckb
        .lock(always_success_lock_script.clone())
        .build();

    // second output will use our contract
    let second_output = CellOutput::new_builder()
        .capacity(99_00000000.pack()) // 101 ckb
        .type_(type_script_for_our_cell.clone())
        .lock(always_success_lock_script.clone())
        .build();

    let input1 = CellInput::new_builder()
        .previous_output(first_input)
        .build();
    let input2 = CellInput::new_builder()
        .previous_output(second_input)
        .build();

    let tx = TransactionBuilder::default()
        .inputs([input1, input2])
        .outputs([first_output, second_output])
        .cell_dep(cell_deps)
        .build();

    let completed_tx = context.complete_tx(tx);

    context
        .verify_tx(&completed_tx, MAX_CYCLES)
        .expect_err("Contract verification failed!!!");
}
