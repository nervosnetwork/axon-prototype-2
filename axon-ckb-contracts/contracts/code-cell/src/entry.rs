// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

use crate::{
    anyone_refresh_task::anyone_refresh_task, cell::*, checker_bond_withdraw::checker_bond_withdraw,
    checker_join_sidechain::checker_join_sidechain, checker_publish_challenge::checker_publish_challenge,
    checker_quit_sidechain::checker_quit_sidechain, checker_take_beneficiary::checker_take_beneficiary, checker_vote::checker_vote,
    collator_publish_task::collator_publish_task, collator_submit_faild_challenge::collator_submit_faild_challenge,
    collator_submit_success_challenge::collator_submit_success_challenge, collator_submit_tasks::collator_submit_tasks,
    collator_unlock_bond::collator_unlock_bond, error::Error,
};

use ckb_std::ckb_constants::Source;
use ckb_std::{ckb_types::prelude::*, high_level::load_witness_args};

use crate::pattern::is_admin_create_sidechain;
use common_raw::{
    cell::{code::CodeCellLockArgs, sidechain_config::SidechainConfigCellTypeArgs, sidechain_state::SidechainStateCellTypeArgs},
    pattern::Pattern,
    witness::{admin_create_sidechain::AdminCreateSidechainWitness, code_cell_witness::CodeCellTypeWitness},
    FromRaw,
};

const CODE_INPUT: CellOrigin = CellOrigin(0, Source::Input);

pub fn main() -> Result<(), Error> {
    /*
    the unlocker of code cell is the owner/signer of code cell
    thus code cell's lock script must be secp256k1
     */
    // of cause, the signer is correct
    let signer = CodeCellLockArgs::load(CODE_INPUT)?.lock_arg;

    let witness = load_witness_args(0, Source::GroupInput)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let raw_witness = witness.as_reader().raw_data();

    let witness = CodeCellTypeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    match witness.pattern() {
        /*
        CheckerBondWithdraw

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Bond Cell           ->         Muse Token Cell

         */
        Pattern::CheckerBondWithdraw => checker_bond_withdraw(signer),

        /*
        CheckerJoinSidechain,

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Null                        ->          Checker Info Cell

        */
        Pattern::CheckerJoinSidechain => checker_join_sidechain(raw_witness, signer),
        /*
        CheckerQuitSidechain

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Checker Info Cell           ->          Null
        */
        Pattern::CheckerQuitSidechain => checker_quit_sidechain(raw_witness, signer),

        /*
        CheckerVote,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          Null

        */
        Pattern::CheckerVote => checker_vote(raw_witness, signer),
        /*
        CheckerPublishChallenge,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          [Task Cell]

        */
        Pattern::CheckerPublishChallenge => checker_publish_challenge(raw_witness, signer),
        /*
        CheckerTakeBeneficiary,

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        Muse Token Cell             ->          Muse Token Cell

        */
        Pattern::CheckerTakeBeneficiary => checker_take_beneficiary(raw_witness, signer),

        /*
        AdminCreateSidechain,

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        CKB Cell                    ->          Sidechain Config Cell
        Null                        ->          Sidechain State Cell

        */
        Pattern::AdminCreateSidechain => {
            is_admin_create_sidechain()?;
            admin_create_sidechain(signer)
        }

        /*
        CollatorPublishTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        Sidechain State Cell        ->          Sidechain State Cell
        Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
        Null                        ->          [Task Cell]

        */
        Pattern::CollatorPublishTask => collator_publish_task(raw_witness, signer),
        Pattern::CollatorSubmitTasks => collator_submit_tasks(raw_witness, signer),

        /*
        CollatorSubmitChallenge,

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Sidechain State Cell        ->          Sidechain State Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        Muse Token Cell
        [Checker Info Cell]         ->          [Checker Info Cell]

        */
        Pattern::CollatorSubmitFaildChallenge => collator_submit_faild_challenge(raw_witness, signer),

        /*
        CollatorSubmitChallenge,

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        Sidechain Bond Cell
        [Checker Info Cell]         ->          [Checker Info Cell]

        */
        Pattern::CollatorSubmitSuccessChallenge => collator_submit_success_challenge(raw_witness),

        /*
        RefreshTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        [Task Cell]                 ->          [Task Cell]

        */
        Pattern::AnyoneRefreshTask => anyone_refresh_task(raw_witness),

        /*
        CollatorUnlockBond,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell
        Dep:    2 Sidechain State Cell

        Code Cell                   ->          Code Cell
        Sidechain Bond Cell         ->          Sudt Cell

        */
        Pattern::CollatorUnlockBond => collator_unlock_bond(raw_witness, signer),
    }
}

fn admin_create_sidechain(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    AdminCreateSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    CKB Cell                    ->          Sidechain Config Cell
    Null                        ->          Sidechain State Cell

    */
    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = AdminCreateSidechainWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let sidechain_config_output_type_args = SidechainConfigCellTypeArgs::load(CellOrigin(1, Source::Output))?;

    let _sidechain_state_output_type_args = SidechainStateCellTypeArgs::load(CellOrigin(2, Source::Output))?;

    if sidechain_config_output_type_args.chain_id != witness.chain_id {
        return Err(Error::Wrong);
    }

    //TODO check chain_id

    Ok(())
}
