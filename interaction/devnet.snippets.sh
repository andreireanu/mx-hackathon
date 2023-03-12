PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/mx-hackathon/wallets/alice.pem"
WALLET_BOB="${PWD}/mx-hackathon/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqsxd5zwqh0cf3azau89sm8r82dn8kx6ps7wpqm76wgy"
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
SWAP_ADDRESS="erd1qqqqqqqqqqqqqpgq4my4pgeuceewx4avaqxddpsetz9v7hff7wpqs5w869"

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
 
######## ISSUE FUNGIBLE TOKEN

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

######## ISSUE NFT TOKEN

sftIssue() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="sftIssue" \
    --arguments "str:"$TKN_NAME "str:"$TKN_TICKER  
} 
 

ISS_TOKEN="SLM-a81055"
ISS_TOKEN_HEX="$(echo -n ${ISS_TOKEN} | xxd -p -u | tr -d '\n')"

F_TOKEN="SLM-f73a62"
F_TOKEN_HEX="$(echo -n ${F_TOKEN} | xxd -p -u | tr -d '\n')"
 
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

######## CLAIM TOKEN

claim() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=70000000 \
    --proxy=${PROXY} \
    --function="claim" \
    --arguments "str:"$F_TOKEN 

} 
 

######## BUY TOKEN

BUY="buyNft"
BUY_HEX="$(echo -n ${BUY} | xxd -p -u | tr -d '\n')"
 
AMOUNT="0.1"
AMOUNT_HEX="$(echo -n ${AMOUNT} | xxd -p -u | tr -d '\n')"

AMOUNT_TOKENS=10
NFT_NONCE=1
F_TOKEN_HEX2=str:SLM-f73a62
BUY_HEX2=str:buyNft

buyNft() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=10000000 \
    --function="ESDTTransfer" \
    --arguments ${F_TOKEN_HEX2} ${AMOUNT_TOKENS} ${BUY_HEX2} ${ALICE_ADDRESS} ${NFT_NONCE}
}
 
buyNft2() {
    erdpy --verbose tx new \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=10000000 \
    --receiver=${CONTRACT_ADDRESS} \
    --data="ESDTTransfer@${F_TOKEN_HEX}@0a@${BUY_HEX}@${ALICE_ADDRESS_HEX}@$01" 
}
#########

getUserNft() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserNft"  
    }  

getUserToken() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getUserToken"  
    --data "str:"$WALLET_ALICE
    }  


 


























 