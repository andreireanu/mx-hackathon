PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/mx-hackathon/wallets/alice.pem"
WALLET_BOB="${PWD}/mx-hackathon/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqcgndu0vscec5qx8rx6fcu37784tgxfje7wpql5r9ep"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${MARTA_ADDRESS})"

SFT_ADDRESS="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
 

deploy() {
 erdpy contract deploy --chain="D" \
    --outfile="mx-hackathon/interaction/devnet.interaction.json" \
    --project=mx-hackathon \
    --pem="mx-hackathon/wallets/alice.pem" \
    --gas-limit=60000000 \
    --proxy=${PROXY} \
    --recall-nonce \
    --send \
    --metadata-payable
}
  

upgrade() {
 erdpy contract upgrade ${CONTRACT_ADDRESS} \
    --outfile="mx-hackathon/interaction/devnet.interaction.json" \
    --project=mx-hackathon \
    --pem="mx-hackathon/wallets/alice.pem" \
    --gas-limit=60000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --send \
    --metadata-payable
}

###########################

TKN_NAME="Salam"
TKN_NAME_HEX="$(echo -n ${TKN_NAME} | xxd -p -u | tr -d '\n')"

TKN_TICKER="SLM"
TKN_HEX="$(echo -n ${TKN_TICKER} | xxd -p -u | tr -d '\n')"

NR=1000
 

issueFungibleToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="issueFungibleToken" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER $NR 
} 

######## ISSUE TOKEN
 
TOKEN_NAME=0x54657374546f6b656e
TOKEN_NAME_HEX="$(echo -n ${TOKEN_NAME} | xxd -p -u | tr -d '\n')"

TOKEN_TICKER=0x545354
TOKEN_TICKER_HEX="$(echo -n ${TOKEN_TICKER} | xxd -p -u | tr -d '\n')"

DECIMALS=18

issueToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="issueToken" \
    --arguments $TOKEN_NAME $TOKEN_TICKER 
}

issueSft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="sft_issue" \
    --arguments $TOKEN_NAME $TOKEN_TICKER 
}

getUserNft() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --function="getUserNft" \
    --proxy=${PROXY} \
    --arguments ${ALICE_ADDRESS}
}

#######

ISS_TOKEN="TST-b23329"
ISS_TOKEN_HEX="$(echo -n ${ISS_TOKEN} | xxd -p -u | tr -d '\n')"

# setRole() {
#     erdpy --verbose tx new \
#     --send \
#     --proxy=${PROXY} \
#     --chain=${CHAIN_ID} \
#     --pem=${WALLET_ALICE} \
#     --recall-nonce \
#     --gas-limit=100000000 \
#     --receiver=${SFT_ADDRESS} \
#     --data="setSpecialRole@${ISS_TOKEN_HEX}@${ALICE_ADDRESS_HEX}@45534454526f6c654e4654437265617465"  
# }

setLocalRoles() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="setLocalRoles" \
    --arguments "str:"$ISS_TOKEN 
} 
 

#######

createNft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="createNft"
}

 

# 
################
 
 


###################################
## Issue SFT
 
SPECIAL_ROLES="canAddSpecialRoles"
SPECIAL_ROLES_HEX="$(echo -n ${SPECIAL_ROLES} | xxd -p -u | tr -d '\n')"

TRUE="true"
TRUE_HEX="$(echo -n ${TRUE} | xxd -p -u | tr -d '\n')"

issueSFT() {
    erdpy --verbose tx new \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=100000000 \
    --receiver=${SFT_ADDRESS} \
    --data="issueSemiFungible@${TOKEN_NAME_HEX}@${TOKEN_TICKER_HEX}@${SPECIAL_ROLES_HEX}@${TRUE_HEX}"  
}
 


ISSUED_TOKEN="HTKN-54137b"
ISSUED_TOKEN_HEX="$(echo -n ${ISSUED_TOKEN} | xxd -p -u | tr -d '\n')" 

QNT=1000
QNT_HEX="$(echo -n ${QNT} | xxd -p -u | tr -d '\n')" 

createSFT() {
    erdpy --verbose tx new \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=100000000 \
    --receiver=${ALICE_ADDRESS} \
    --data="ESDTNFTCreate@0${ISSUED_TOKEN}@${QNT_HEX}@${TOKEN_NAME_HEX}@1d4c@00@6d657461646174613a697066734349442f736f6e672e6a736f6e3b746167733a736f6e672c62656175746966756c2c6d75736963@55524c5f746f5f646563656e7472616c697a65645f73746f726167652f736f6e672e6d7033"  
} 