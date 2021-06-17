use super::Loader;
use crate::common::*;
use crate::secp256k1::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core, packed, packed::*, prelude::*};
use common_raw::cell::global_config::GlobalConfigCellData;

pub struct EnvironmentBuilder {
    pub context: Context,
    pub builder: core::TransactionBuilder,
}

pub struct AxonScripts {
    pub always_success_code:   OutPoint,
    pub always_success_script: Script,

    pub secp256k1_code:   OutPoint,
    pub secp256k1_script: Script,

    pub code_cell_code:   OutPoint,
    pub code_cell_script: Script,
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self {
            context: Context::default(),
            builder: core::TransactionBuilder::default(),
        }
    }
}

impl EnvironmentBuilder {
    pub fn create_input(&mut self, output: CellOutput, data: Bytes) -> CellInput {
        CellInput::new_builder()
            .previous_output(self.context.create_cell(output, data))
            .build()
    }

    pub fn create_dep(&mut self, output: CellOutput, data: Bytes) -> CellDep {
        CellDep::new_builder().out_point(self.context.create_cell(output, data)).build()
    }

    pub fn load_contract(mut self, name: &str) -> (Self, OutPoint) {
        let contract_bin: Bytes = Loader::default().load_binary(name);
        let out_point = self.context.deploy_cell(contract_bin);

        let self_ = self.cell_dep(CellDep::new_builder().out_point(out_point.clone()).build());

        (self_, out_point)
    }

    pub fn load_script(self, name: &str) -> (Self, OutPoint, Script) {
        let (mut self_, code) = self.load_contract(name);
        let script = self_.context.build_script(&code.clone(), Bytes::new()).expect("script");

        (self_, code, script)
    }

    pub fn bootstrap(self, lock_args: Vec<u8>) -> (Self, AxonScripts) {
        let mut global_config = GlobalConfigCellData::default();

        let (self_, code_cell_code, code_cell_script) = self.load_script("code-cell");

        global_config
            .code_cell_type_codehash
            .copy_from_slice(code_cell_script.as_reader().code_hash().raw_data());

        let (mut self_, always_success_code, always_success_script) = self_.load_script("always-success");
        let a_s_codehash = always_success_script.as_reader().code_hash().raw_data();

        global_config.checker_bond_cell_lock_codehash.copy_from_slice(a_s_codehash);
        global_config.checker_info_cell_type_codehash.copy_from_slice(a_s_codehash);
        global_config.sidechain_config_cell_type_codehash.copy_from_slice(a_s_codehash);

        let global_config_dep = self_.create_dep(
            new_type_cell_output(1000, &always_success_script, &always_success_script),
            global_config.serialize(),
        );

        let self_ = self_.cell_dep(global_config_dep);

        let (mut self_, secp256k1_code) = with_secp256k1_cell_deps(self_);
        let secp256k1_script = self_.context.build_script(&secp256k1_code, lock_args.into()).expect("script");

        let code_cell_input = self_.create_input(new_type_cell_output(1000, &secp256k1_script, &code_cell_script), Bytes::new());

        let self_ = self_.input(code_cell_input);

        (self_, AxonScripts {
            always_success_code,
            always_success_script,
            secp256k1_code,
            secp256k1_script,
            code_cell_code,
            code_cell_script,
        })
    }

    pub fn cell_dep(mut self, cell_dep: CellDep) -> Self {
        self.builder = self.builder.cell_dep(cell_dep);
        self
    }

    pub fn input(mut self, input: CellInput) -> Self {
        self.builder = self.builder.input(input);
        self
    }

    pub fn header_dep(mut self, header_dep: Byte32) -> Self {
        self.builder = self.builder.header_dep(header_dep);
        self
    }

    pub fn outputs<T>(mut self, outputs: T) -> Self
    where
        T: IntoIterator<Item = CellOutput>,
    {
        self.builder = self.builder.outputs(outputs);
        self
    }

    pub fn outputs_data<T>(mut self, outputs_data: T) -> Self
    where
        T: IntoIterator<Item = packed::Bytes>,
    {
        self.builder = self.builder.outputs_data(outputs_data);
        self
    }
}
