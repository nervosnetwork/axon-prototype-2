import 'dotenv/config'
import {HashType, QueryOptions} from '@ckb-lumos/base'
import {scriptCamelToSnake} from './tools'
import {logger} from './logger'
// @ts-ignore
import JSONbig from 'json-bigint'
// @ts-ignore
import { blake160, privateKeyToPublicKey, scriptToHash } from '@nervosnetwork/ckb-sdk-utils'

function log(msg: string) {
    logger.info(`workEnv: ${msg}`)
}

export const NODE_ENV: string = process.env.NODE_ENV ? process.env.NODE_ENV : 'production'

export const INDEXER_URL: string = process.env.INDEXER_URL!
export const INDEXER_MYSQL_URL = process.env.INDEXER_MYSQL_URL!
export const INDEXER_MYSQL_URL_PORT: number = parseInt(process.env.INDEXER_MYSQL_URL_PORT!)
export const INDEXER_MYSQL_USERNAME = process.env.INDEXER_MYSQL_USERNAME!
export const INDEXER_MYSQL_PASSWORD = process.env.INDEXER_MYSQL_PASSWORD!
export const INDEXER_MYSQL_DATABASE = process.env.INDEXER_MYSQL_DATABASE!

export const CKB_NODE_URL = process.env.CKB_NODE_URL!

export const BLOCK_MINER_FEE = process.env.BLOCK_MINER_FEE ? BigInt(process.env.BLOCK_MINER_FEE) : 100000n

export const SELF_PRIVATE_KEY = process.env.SELF_PRIVATE_KEY!
export const SELF_ADDRESS = `0x${blake160(privateKeyToPublicKey(SELF_PRIVATE_KEY), 'hex')}`
log(`Secp256k1 args: SELF_ADDRESS:${SELF_ADDRESS}`)


// type id code codehash
export const TYPE_ID_CODE_HASH = `0x00000000000000000000000000000000000000000000000000545950455f4944`

// sudt
export const SUDT_TX_HASH = '0xe12877ebd2c3c364dc46c5c992bcfaf4fee33fa13eebdf82c591fc9825aab769'
export const SUDT_TX_INDEX = '0x0'
export const SUDT_CODE_HASH = '0xc5e5dcf215925f7ef4dfaf5f4b4f105bc321c02776d6e7d52a1db3fcd9d011a4'
export const SUDT_HASH_TYPE: HashType = 'type'

// secp256k1
export const SECP256K1_TX_HASH = '0xf8de3bb47d055cdf460d93a2a6e1b05f7432f9777c8c474abf4eec1d4aee5d37'
export const SECP256K1_TX_INDEX = '0x0'
export const SECP256K1_CODE_HASH = '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8'
export const SECP256K1_HASH_TYPE: HashType = 'type'

// always success
export const ALWAYS_SUCCESS_TX_HASH = process.env.ALWAYS_SUCCESS_TX_HASH!
export const ALWAYS_SUCCESS_TX_INDEX = process.env.ALWAYS_SUCCESS_TX_INDEX!
export const ALWAYS_SUCCESS_CODE_HASH = process.env.ALWAYS_SUCCESS_CODE_HASH!
export const ALWAYS_SUCCESS_HASH_TYPE: HashType = process.env.ALWAYS_SUCCESS_HASH_TYPE === 'type' ? 'type' : 'data'

export const SECP256K1_ARG = process.env.SECP256K1_ARG!


// global config cell
/*
capacity: - 8 bytes
data:
    skip
type: - 65 bytes
    codehash: typeid code
    hashtype: type
    args: unique_id
lock: - 53 bytes
    codehash: secp256k1 code
    hashtype: type
    args: public key hash
 */
export const GLOBAL_CONFIG_TYPE_ARGS = process.env.GLOBAL_CONFIG_TYPE_ARGS!
export const GLOBAL_CONFIG_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: TYPE_ID_CODE_HASH,
    hashType: `type`,
    args: GLOBAL_CONFIG_TYPE_ARGS,
}

export const GLOBAL_CONFIG_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: SECP256K1_CODE_HASH,
    hashType: SECP256K1_HASH_TYPE,
    args: SECP256K1_ARG,
}

export const GLOBAL_CONFIG_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 32,
        script: scriptCamelToSnake(GLOBAL_CONFIG_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 20,
        script: scriptCamelToSnake(GLOBAL_CONFIG_LOCK_SCRIPT),
    },
}

/*
custom sudt for sidechain bond of collator

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    code: sudt_type_script
    hashtype: type
    args: custom_owner_lock_hash
lock: - 53 bytes
    codehash: secp256k1 code
    hashtype: type
    args: public key hash - 20 bytes
 */
export const SUDT_TYPE_ARGS = process.env.SUDT_TYPE_ARGS!
export const SUDT_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SUDT_CODE_HASH,
    hashType: SUDT_HASH_TYPE,
    args: SUDT_TYPE_ARGS,
}

export const SUDT_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: SECP256K1_CODE_HASH,
    hashType: SECP256K1_HASH_TYPE,
    args: SECP256K1_ARG,
}

export const SUDT_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 32,
        script: scriptCamelToSnake(SUDT_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 20,
        script: scriptCamelToSnake(SUDT_LOCK_SCRIPT),
    },
}
/*
muse

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    code: sudt_type_script
    hashtype: type
    args: muse_owner_lock_hash
lock: - 53 bytes
    codehash: secp256k1 code
    hashtype: type
    args: public key hash - 20 bytes
 */
export const MUSE_TYPE_ARGS = process.env.MUSE_TYPE_ARGS!
export const MUSE_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SUDT_CODE_HASH,
    hashType: SUDT_HASH_TYPE,
    args: MUSE_TYPE_ARGS,
}

export const MUSE_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: SECP256K1_CODE_HASH,
    hashType: SECP256K1_HASH_TYPE,
    args: SECP256K1_ARG,
}

export const MUSE_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 32,
        script: scriptCamelToSnake(MUSE_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 20,
        script: scriptCamelToSnake(MUSE_LOCK_SCRIPT),
    },
}
/*
checker bond

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    codehash: sudt_type_script
    hashtype: type
    args: muse_owner_lock_hash
lock: - 85 bytes
    codehash: typeid
    hashtype: type
    args: checker public key hash | chain id bitmap - 52 bytes
 */


//==========
export const CHECKER_BOND_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SUDT_CODE_HASH,
    hashType: SUDT_HASH_TYPE,
    args: MUSE_TYPE_ARGS,
}


export const CHECKER_BOND_LOCK_OUTPOINT_TX_HASH = process.env.CHECKER_BOND_LOCK_OUTPOINT_TX_HASH!
export const CHECKER_BOND_LOCK_OUTPOINT_INDEX = process.env.CHECKER_BOND_LOCK_OUTPOINT_INDEX!

export const CHECKER_BOND_LOCK_CODE_HASH = process.env.CHECKER_BOND_LOCK_CODE_HASH!
export const CHECKER_BOND_LOCK_HASH_TYPE: HashType = process.env.CHECKER_BOND_LOCK_HASH_TYPE === 'type' ? 'type' : 'data'
export const CHECKER_BOND_LOCK_ARGS = process.env.CHECKER_BOND_LOCK_ARGS!
export const CHECKER_BOND_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: CHECKER_BOND_LOCK_CODE_HASH,
    hashType: CHECKER_BOND_LOCK_HASH_TYPE,
    args: CHECKER_BOND_LOCK_ARGS,
}


export const CHECKER_BOND_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 32,
        script: scriptCamelToSnake(CHECKER_BOND_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 52,
        script: scriptCamelToSnake(CHECKER_BOND_LOCK_SCRIPT),
    },
}

/*
checker info

capacity: - 8 bytes
data: - 1+1+16+512+20+1=551
    pub chain_id:                u8,
    pub checker_id:              u8,
    pub unpaid_fee:              u128,
    pub rpc_url:                 [u8; 512],
    pub checker_public_key_hash: [u8; 20],
    pub mode:                    CheckerInfoCellMode,//u8
type: - 54 bytes
    codehash: typeid
    hashtype: type
    args: chain id | public key hash - 21 bytes
lock: - A.S. 33 bytes

 */
export const CHECKER_INFO_TYPE_OUTPOINT_TX_HASH = process.env.CHECKER_INFO_TYPE_OUTPOINT_TX_HASH!
export const CHECKER_INFO_TYPE_OUTPOINT_INDEX = process.env.CHECKER_INFO_TYPE_OUTPOINT_INDEX!

export const CHECKER_INFO_TYPE_CODE_HASH = process.env.CHECKER_INFO_TYPE_CODE_HASH!
export const CHECKER_INFO_TYPE_HASH_TYPE: HashType = process.env.CHECKER_INFO_TYPE_HASH_TYPE === 'type' ? 'type' : 'data'
export const CHECKER_INFO_TYPE_ARGS = process.env.CHECKER_INFO_TYPE_ARGS!
export const CHECKER_INFO_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: CHECKER_INFO_TYPE_CODE_HASH,
    hashType: CHECKER_INFO_TYPE_HASH_TYPE,
    args: CHECKER_INFO_TYPE_ARGS,
}

export const CHECKER_INFO_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: ALWAYS_SUCCESS_CODE_HASH,
    hashType: ALWAYS_SUCCESS_HASH_TYPE,
    args: ``,
}


export const CHECKER_INFO_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 21,
        script: scriptCamelToSnake(CHECKER_INFO_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 0,
        script: scriptCamelToSnake(CHECKER_INFO_LOCK_SCRIPT),
    },
}

/*
code

capacity: - 8 bytes
data: -

type: - 65 bytes
    codehash: typeid
    hashtype: type
    args: chain id | public key hash - 21 bytes
lock: - 65 bytes
    codehash: secp256k1
    hashtype: type
    args: owner public key hash - 20 bytes

 */
export const CODE_TYPE_OUTPOINT_TX_HASH = process.env.CODE_TYPE_OUTPOINT_TX_HASH!
export const CODE_TYPE_OUTPOINT_INDEX = process.env.CODE_TYPE_OUTPOINT_INDEX!

export const CODE_TYPE_CODE_HASH = process.env.CODE_TYPE_CODE_HASH!
export const CODE_TYPE_HASH_TYPE: HashType = process.env.CODE_TYPE_HASH_TYPE === 'type' ? 'type' : 'data'
export const CODE_TYPE_ARGS = process.env.CODE_TYPE_ARGS!
export const CODE_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: CODE_TYPE_CODE_HASH,
    hashType: CODE_TYPE_HASH_TYPE,
    args: CODE_TYPE_ARGS,
}

export const CODE_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: SECP256K1_CODE_HASH,
    hashType: SECP256K1_HASH_TYPE,
    args: SECP256K1_ARG,
}

export const CODE_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 21,
        script: scriptCamelToSnake(CODE_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 20,
        script: scriptCamelToSnake(CODE_LOCK_SCRIPT),
    },
}

/*
sidechain bond

capacity: - 8 bytes
data: - amount
type: - 65 bytes
    codehash: sudt type
    hashtype: type
    args: custom sudt admin
lock: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain_id | collator_public_key_hash | unlock_sidechain_height - 37 bytes

 */

export const SIDECHAIN_BOND_TYPE_ARGS = SUDT_TYPE_ARGS
export const SIDECHAIN_BOND_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SUDT_CODE_HASH,
    hashType: SUDT_HASH_TYPE,
    args: SIDECHAIN_BOND_TYPE_ARGS,
}


export const SIDECHAIN_BOND_LOCK_OUTPOINT_TX_HASH = process.env.SIDECHAIN_BOND_LOCK_OUTPOINT_TX_HASH!
export const SIDECHAIN_BOND_LOCK_OUTPOINT_INDEX = process.env.SIDECHAIN_BOND_LOCK_OUTPOINT_INDEX!

export const SIDECHAIN_BOND_LOCK_CODE_HASH = process.env.SIDECHAIN_BOND_LOCK_CODE_HASH!
export const SIDECHAIN_BOND_LOCK_HASH_TYPE: HashType = process.env.SIDECHAIN_BOND_LOCK_HASH_TYPE === 'type' ? 'type' : 'data'
export const SIDECHAIN_BOND_LOCK_ARGS = process.env.SIDECHAIN_BOND_LOCK_ARGS!
export const SIDECHAIN_BOND_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: SIDECHAIN_BOND_LOCK_CODE_HASH,
    hashType: SIDECHAIN_BOND_LOCK_HASH_TYPE,
    args: SIDECHAIN_BOND_LOCK_ARGS,
}

export const SIDECHAIN_BOND_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 32,
        script: scriptCamelToSnake(SIDECHAIN_BOND_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 37,
        script: scriptCamelToSnake(SIDECHAIN_BOND_LOCK_SCRIPT),
    },
}

/*
sidechain config

capacity: - 8 bytes
data: -

type: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain id
lock: - A.S. 33 bytes

 */

export const SIDECHAIN_CONFIG_TYPE_OUTPOINT_TX_HASH = process.env.SIDECHAIN_CONFIG_TYPE_OUTPOINT_TX_HASH!
export const SIDECHAIN_CONFIG_TYPE_OUTPOINT_INDEX = process.env.SIDECHAIN_CONFIG_TYPE_OUTPOINT_INDEX!

export const SIDECHAIN_CONFIG_TYPE_CODE_HASH = process.env.SIDECHAIN_CONFIG_TYPE_CODE_HASH!
export const SIDECHAIN_CONFIG_TYPE_HASH_TYPE: HashType = process.env.SIDECHAIN_CONFIG_TYPE_HASH_TYPE === 'type' ? 'type' : 'data'
export const SIDECHAIN_CONFIG_TYPE_ARGS = process.env.SIDECHAIN_CONFIG_TYPE_ARGS!
export const SIDECHAIN_CONFIG_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SIDECHAIN_CONFIG_TYPE_CODE_HASH,
    hashType: SIDECHAIN_CONFIG_TYPE_HASH_TYPE,
    args: SIDECHAIN_CONFIG_TYPE_ARGS,
}

export const SIDECHAIN_CONFIG_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: ALWAYS_SUCCESS_CODE_HASH,
    hashType: ALWAYS_SUCCESS_HASH_TYPE,
    args: ``,
}

export const SIDECHAIN_CONFIG_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 1,
        script: scriptCamelToSnake(SIDECHAIN_CONFIG_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 0,
        script: scriptCamelToSnake(SIDECHAIN_CONFIG_LOCK_SCRIPT),
    },
}



/*
sidechain fee

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    codehash: sudt_type_script
    hashtype: type
    args: muse_owner_lock_hash
lock:
    codehash: typeid
    hashtype: type
    args: chain id
 */


//==========
export const SIDECHAIN_FEE_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SUDT_CODE_HASH,
    hashType: SUDT_HASH_TYPE,
    args: MUSE_TYPE_ARGS,
}


export const SIDECHAIN_FEE_LOCK_OUTPOINT_TX_HASH = process.env.SIDECHAIN_FEE_LOCK_OUTPOINT_TX_HASH!
export const SIDECHAIN_FEE_LOCK_OUTPOINT_INDEX = process.env.SIDECHAIN_FEE_LOCK_OUTPOINT_INDEX!

export const SIDECHAIN_FEE_LOCK_CODE_HASH = process.env.SIDECHAIN_FEE_LOCK_CODE_HASH!
export const SIDECHAIN_FEE_LOCK_HASH_TYPE: HashType = process.env.SIDECHAIN_FEE_LOCK_HASH_TYPE === 'type' ? 'type' : 'data'
export const SIDECHAIN_FEE_LOCK_ARGS = process.env.SIDECHAIN_FEE_LOCK_ARGS! // should be 32 bytes
export const SIDECHAIN_FEE_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: SIDECHAIN_FEE_LOCK_CODE_HASH,
    hashType: SIDECHAIN_FEE_LOCK_HASH_TYPE,
    args: SIDECHAIN_FEE_LOCK_ARGS,
}


export const SIDECHAIN_FEE_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 32,
        script: scriptCamelToSnake(SIDECHAIN_FEE_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 1,
        script: scriptCamelToSnake(SIDECHAIN_FEE_LOCK_SCRIPT),
    },
}


/*
sidechain status

capacity: - 8 bytes
data: -

type: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain id
lock: - A.S. 33 bytes

 */

export const SIDECHAIN_STATE_TYPE_OUTPOINT_TX_HASH = process.env.SIDECHAIN_STATE_TYPE_OUTPOINT_TX_HASH!
export const SIDECHAIN_STATE_TYPE_OUTPOINT_INDEX = process.env.SIDECHAIN_STATE_TYPE_OUTPOINT_INDEX!

export const SIDECHAIN_STATE_TYPE_CODE_HASH = process.env.SIDECHAIN_STATE_TYPE_CODE_HASH!
export const SIDECHAIN_STATE_TYPE_HASH_TYPE: HashType = process.env.SIDECHAIN_STATE_TYPE_HASH_TYPE === 'type' ? 'type' : 'data'
export const SIDECHAIN_STATE_TYPE_ARGS = process.env.SIDECHAIN_STATE_TYPE_ARGS!
export const SIDECHAIN_STATE_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: SIDECHAIN_STATE_TYPE_CODE_HASH,
    hashType: SIDECHAIN_STATE_TYPE_HASH_TYPE,
    args: SIDECHAIN_STATE_TYPE_ARGS,
}

export const SIDECHAIN_STATE_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: ALWAYS_SUCCESS_CODE_HASH,
    hashType: ALWAYS_SUCCESS_HASH_TYPE,
    args: ``,
}

export const SIDECHAIN_STATE_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 1,
        script: scriptCamelToSnake(SIDECHAIN_STATE_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 0,
        script: scriptCamelToSnake(SIDECHAIN_STATE_LOCK_SCRIPT),
    },
}


/*
task

capacity: - 8 bytes
data: -

type: - 65 bytes
    codehash: typeid
    hashtype: type
    args: chain id | public key hash - 21 bytes
lock: - A.S. 33 bytes

 */
export const TASK_TYPE_OUTPOINT_TX_HASH = process.env.TASK_TYPE_OUTPOINT_TX_HASH!
export const TASK_TYPE_OUTPOINT_INDEX = process.env.TASK_TYPE_OUTPOINT_INDEX!

export const TASK_TYPE_CODE_HASH = process.env.TASK_TYPE_CODE_HASH!
export const TASK_TYPE_HASH_TYPE: HashType = process.env.TASK_TYPE_HASH_TYPE === 'type' ? 'type' : 'data'
export const TASK_TYPE_ARGS = process.env.TASK_TYPE_ARGS!
export const TASK_TYPE_SCRIPT: CKBComponents.Script = {
    codeHash: TASK_TYPE_CODE_HASH,
    hashType: TASK_TYPE_HASH_TYPE,
    args: TASK_TYPE_ARGS,
}


export const TASK_STATE_LOCK_SCRIPT: CKBComponents.Script = {
    codeHash: ALWAYS_SUCCESS_CODE_HASH,
    hashType: ALWAYS_SUCCESS_HASH_TYPE,
    args: ``,
}

export const TASK_QUERY_OPTION: QueryOptions = {
    type: {
        argsLen: 21,
        script: scriptCamelToSnake(TASK_TYPE_SCRIPT),
    },
    lock: {
        argsLen: 0,
        script: scriptCamelToSnake(TASK_STATE_LOCK_SCRIPT),
    },
}

export const INSTANCE_NAME: string = process.env.INSTANCE_NAME!
log(`INSTANCE_NAME:${INSTANCE_NAME}`)

log('GLOBAL_CONFIG_QUERY_OPTION: ' + JSONbig.stringify(GLOBAL_CONFIG_QUERY_OPTION, null, 2))
log('SUDT_QUERY_OPTION: ' + JSONbig.stringify(SUDT_QUERY_OPTION, null, 2))
log('MUSE_QUERY_OPTION: ' + JSONbig.stringify(MUSE_QUERY_OPTION, null, 2))
log('CHECKER_BOND_QUERY_OPTION: ' + JSONbig.stringify(CHECKER_BOND_QUERY_OPTION, null, 2))
log('CHECKER_INFO_QUERY_OPTION: ' + JSONbig.stringify(CHECKER_INFO_QUERY_OPTION, null, 2))
log('CODE_QUERY_OPTION: ' + JSONbig.stringify(CODE_QUERY_OPTION, null, 2))
log('SIDECHAIN_BOND_QUERY_OPTION: ' + JSONbig.stringify(SIDECHAIN_BOND_QUERY_OPTION, null, 2))
log('SIDECHAIN_CONFIG_QUERY_OPTION: ' + JSONbig.stringify(SIDECHAIN_CONFIG_QUERY_OPTION, null, 2))
log('SIDECHAIN_FEE_QUERY_OPTION: ' + JSONbig.stringify(SIDECHAIN_FEE_QUERY_OPTION, null, 2))
log('SIDECHAIN_STATE_QUERY_OPTION: ' + JSONbig.stringify(SIDECHAIN_STATE_QUERY_OPTION, null, 2))
log('TASK_QUERY_OPTION: ' + JSONbig.stringify(TASK_QUERY_OPTION, null, 2))

export const CELL_DEPS = [
    {
        outPoint: {
            txHash: SUDT_TX_HASH,
            index: SUDT_TX_INDEX,
        },
        depType: 'code' as CKBComponents.DepType,
    },
    {
        outPoint: {
            txHash: SECP256K1_TX_HASH,
            index: SECP256K1_TX_INDEX,
        },
        depType: 'code' as CKBComponents.DepType,
    },
    {
        outPoint: {
            txHash: ALWAYS_SUCCESS_TX_HASH,
            index: ALWAYS_SUCCESS_TX_INDEX,
        },
        depType: 'code' as CKBComponents.DepType,
    },
    {
        outPoint: {
            txHash: CHECKER_BOND_LOCK_OUTPOINT_TX_HASH,
            index: CHECKER_BOND_LOCK_OUTPOINT_INDEX,
        },
        depType: 'code' as CKBComponents.DepType,
    },
    {
        outPoint: {
            txHash: CHECKER_INFO_TYPE_OUTPOINT_TX_HASH,
            index: CHECKER_INFO_TYPE_OUTPOINT_INDEX,
        },
        depType: 'code' as CKBComponents.DepType,
    },
    {
        outPoint: {
            txHash: SECP256K1_TX_HASH,
            index: SECP256K1_TX_INDEX,
        },
        depType: 'depGroup' as CKBComponents.DepType,
    },
]
