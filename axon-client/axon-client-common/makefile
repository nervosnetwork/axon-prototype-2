generate_mol_2_ts:
#	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/common.mol > ./mol-intermediate/common.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/checker_bond.mol > ./mol-intermediate/cell/checker_bond.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/checker_info.mol > ./mol-intermediate/cell/checker_info.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/code.mol > ./mol-intermediate/cell/code.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/sidechain_bond.mol > ./mol-intermediate/cell/sidechain_bond.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/sidechain_config.mol > ./mol-intermediate/cell/sidechain_config.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/sidechain_fee.mol > ./mol-intermediate/cell/sidechain_fee.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/sidechain_registry.mol > ./mol-intermediate/cell/sidechain_registry.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/sidechain_state.mol > ./mol-intermediate/cell/sidechain_state.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/sudt_token.mol > ./mol-intermediate/cell/sudt_token.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/cell/task.mol > ./mol-intermediate/cell/task.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/witness/anyone_shutdown_sidechain.mol > ./mol-intermediate/witness/anyone_shutdown_sidechain.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/witness/collator_submit_tasks.mol > ./mol-intermediate/witness/collator_submit_tasks.json
	moleculec --language - --format json --schema-file ../../axon-ckb-contracts/common-raw/src/molecule/witness/collator_shutdown_sidechain.mol > ./mol-intermediate/witness/collator_shutdown_sidechain.json

	moleculec-es -inputFile ./mol-intermediate/cell/checker_bond.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/checker_bond.js
	moleculec-es -inputFile ./mol-intermediate/cell/checker_info.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/checker_info.js
	moleculec-es -inputFile ./mol-intermediate/cell/code.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/code.js
	moleculec-es -inputFile ./mol-intermediate/cell/sidechain_bond.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/sidechain_bond.js
	moleculec-es -inputFile ./mol-intermediate/cell/sidechain_config.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/sidechain_config.js
	moleculec-es -inputFile ./mol-intermediate/cell/sidechain_fee.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/sidechain_fee.js
	moleculec-es -inputFile ./mol-intermediate/cell/sidechain_registry.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/sidechain_registry.js
	moleculec-es -inputFile ./mol-intermediate/cell/sidechain_state.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/sidechain_state.js
	moleculec-es -inputFile ./mol-intermediate/cell/sudt_token.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/sudt_token.js
	moleculec-es -inputFile ./mol-intermediate/cell/task.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/task.js
	moleculec-es -inputFile ./mol-intermediate/witness/anyone_shutdown_sidechain.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/anyone_shutdown_sidechain.js
	moleculec-es -inputFile ./mol-intermediate/witness/collator_submit_tasks.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/collator_submit_tasks.js
	moleculec-es -inputFile ./mol-intermediate/witness/collator_shutdown_sidechain.json --generateTypeScriptDefinition  -hasBigInt  -outputFile ./src/modules/models/mol/collator_shutdown_sidechain.js

	@echo 'check if any witness left!!!!!'

prepare_mol_env:
	cargo install moleculec
