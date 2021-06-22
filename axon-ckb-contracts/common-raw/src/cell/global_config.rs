use crate::{check_args_len, FromRaw, Serialize};

const GLOBAL_CONFIG_DATA_LEN: usize = 284;

/**

    Global config cell only contains data

    Global Config Cell
    Data:
    Type:
        codehash: typeid                // A.S.
        hashtype: type                  // data
        args: unique_id                 // null
    Lock:
        codehash: secp256k1
        args: admin
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct GlobalConfigCellData {
    pub admin_lock_arg:          [u8; 20],
    /* this is the authenticated admin for
     * sidechain config cell */
    pub code_cell_type_codehash: [u8; 32],
    pub code_cell_type_hashtype: u8,

    pub sidechain_config_cell_type_codehash: [u8; 32],
    pub sidechain_config_cell_type_hashtype: u8,

    pub sidechain_state_cell_type_codehash: [u8; 32],
    pub sidechain_state_cell_type_hashtype: u8,

    pub checker_info_cell_type_codehash: [u8; 32],
    pub checker_info_cell_type_hashtype: u8,

    pub checker_bond_cell_lock_codehash: [u8; 32],
    pub checker_bond_cell_lock_hashtype: u8,

    pub task_cell_type_codehash: [u8; 32],
    pub task_cell_type_hashtype: u8,

    pub sidechain_fee_cell_lock_codehash: [u8; 32],
    pub sidechain_fee_cell_lock_hashtype: u8,

    pub sidechain_bond_cell_lock_codehash: [u8; 32],
    pub sidechain_bond_cell_lock_hashtype: u8,
}

impl FromRaw for GlobalConfigCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<GlobalConfigCellData> {
        check_args_len(cell_raw_data.len(), GLOBAL_CONFIG_DATA_LEN)?;

        let mut admin_lock_arg = [0u8; 20];
        admin_lock_arg.copy_from_slice(&cell_raw_data[0..20]);

        let mut code_cell_type_codehash = [0u8; 32];
        code_cell_type_codehash.copy_from_slice(&cell_raw_data[20..52]);
        let code_cell_type_hashtype = u8::from_raw(&cell_raw_data[52..53])?;

        let mut sidechain_config_cell_type_codehash = [0u8; 32];
        sidechain_config_cell_type_codehash.copy_from_slice(&cell_raw_data[53..85]);
        let sidechain_config_cell_type_hashtype = u8::from_raw(&cell_raw_data[85..86])?;

        let mut sidechain_state_cell_type_codehash = [0u8; 32];
        sidechain_state_cell_type_codehash.copy_from_slice(&cell_raw_data[86..118]);
        let sidechain_state_cell_type_hashtype = u8::from_raw(&cell_raw_data[118..119])?;

        let mut checker_info_cell_type_codehash = [0u8; 32];
        checker_info_cell_type_codehash.copy_from_slice(&cell_raw_data[119..151]);
        let checker_info_cell_type_hashtype = u8::from_raw(&cell_raw_data[151..152])?;

        let mut checker_bond_cell_lock_codehash = [0u8; 32];
        checker_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[152..184]);
        let checker_bond_cell_lock_hashtype = u8::from_raw(&cell_raw_data[184..185])?;

        let mut task_cell_type_codehash = [0u8; 32];
        task_cell_type_codehash.copy_from_slice(&cell_raw_data[185..217]);
        let task_cell_type_hashtype = u8::from_raw(&cell_raw_data[217..218])?;

        let mut sidechain_fee_cell_lock_codehash = [0u8; 32];
        sidechain_fee_cell_lock_codehash.copy_from_slice(&cell_raw_data[218..250]);
        let sidechain_fee_cell_lock_hashtype = u8::from_raw(&cell_raw_data[250..251])?;

        let mut sidechain_bond_cell_lock_codehash = [0u8; 32];
        sidechain_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[251..283]);
        let sidechain_bond_cell_lock_hashtype = u8::from_raw(&cell_raw_data[283..284])?;

        Some(GlobalConfigCellData {
            admin_lock_arg,
            code_cell_type_codehash,
            code_cell_type_hashtype,
            sidechain_config_cell_type_codehash,
            sidechain_config_cell_type_hashtype,
            sidechain_state_cell_type_codehash,
            sidechain_state_cell_type_hashtype,
            checker_info_cell_type_codehash,
            checker_info_cell_type_hashtype,
            checker_bond_cell_lock_codehash,
            checker_bond_cell_lock_hashtype,
            task_cell_type_codehash,
            task_cell_type_hashtype,
            sidechain_fee_cell_lock_codehash,
            sidechain_fee_cell_lock_hashtype,
            sidechain_bond_cell_lock_codehash,
            sidechain_bond_cell_lock_hashtype,
        })
    }
}

impl Serialize for GlobalConfigCellData {
    type RawType = [u8; GLOBAL_CONFIG_DATA_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; GLOBAL_CONFIG_DATA_LEN];

        buf[0..20].copy_from_slice(&self.admin_lock_arg);

        buf[20..52].copy_from_slice(&self.code_cell_type_codehash);

        buf[52..53].copy_from_slice(&self.code_cell_type_hashtype.serialize());

        buf[53..85].copy_from_slice(&self.sidechain_config_cell_type_codehash);
        buf[85..86].copy_from_slice(&self.sidechain_config_cell_type_hashtype.serialize());

        buf[86..118].copy_from_slice(&self.sidechain_state_cell_type_codehash);
        buf[118..119].copy_from_slice(&self.sidechain_state_cell_type_hashtype.serialize());

        buf[119..151].copy_from_slice(&self.checker_info_cell_type_codehash);
        buf[151..152].copy_from_slice(&self.checker_info_cell_type_hashtype.serialize());

        buf[152..184].copy_from_slice(&self.checker_bond_cell_lock_codehash);
        buf[184..185].copy_from_slice(&self.checker_bond_cell_lock_hashtype.serialize());

        buf[185..217].copy_from_slice(&self.task_cell_type_codehash);
        buf[217..218].copy_from_slice(&self.task_cell_type_hashtype.serialize());

        buf[218..250].copy_from_slice(&self.sidechain_fee_cell_lock_codehash);
        buf[250..251].copy_from_slice(&self.sidechain_fee_cell_lock_hashtype.serialize());

        buf[251..283].copy_from_slice(&self.sidechain_bond_cell_lock_codehash);
        buf[283..284].copy_from_slice(&self.sidechain_bond_cell_lock_hashtype.serialize());

        buf
    }
}
