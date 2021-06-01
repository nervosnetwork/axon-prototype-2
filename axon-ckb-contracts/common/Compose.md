# prcedure of composing and populating cells

## 1 Global Config Cell
build merely empty global config cell and deploy it by type id.

leave all data field to blank except admin_public_key

set typescript to typeId

set lockscript to secp256k1 with args of admin_public_key_hash

## 2 Pass the typeId of Global Config Cell into all other cell's code

since the Global Config Cell has been deployed, the typeId must be determined.

though build.rs to hardcode typeId of Global Config Cell into other scripts

## 3 Deploy all other scripts by type id

now all other scripts have been deployed with their own typeIds

## 4 Update typeIds of scripts into Global Config Cell

## 5 Do business logic referring to their scripts by type id

## 6 Check Orthodox Business Logic

all script except code cell shall check if Global Config Cell of CellDeps[0] matches typeId hardcoded


the input[0] code cell checks Global Config Cell of CellDeps[0] matches typeId hardcoded

the input[0] code cell checks if all other cell's script matches typeId from Global Config Cell

code cell do business logic

