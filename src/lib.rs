#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000;
const ROYALTIES: u32 = 10;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]

pub trait MxHackathon
// : multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self) {}

    ////////////////
    // Issue fungible token

    #[payable("EGLD")]
    #[endpoint(issueFungibleToken)]
    fn issue_fungible_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_fungible_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn issue_fungible_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        // TO DO
        // self.user_token(&caller).set(&token_identifier);
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.user_token(caller).set(token_identifier.unwrap_esdt());
            }
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
                // self.last_error_message().set(&message.err_msg);
            }
        }
    }

    ////////////////
    // Issue SFT
    #[payable("EGLD")]
    #[endpoint(sftIssue)]
    fn sft_issue(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        // require!(
        //     self.user_nft(&caller).is_empty(),
        //     "User already has an SFT issued"
        // );
        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().sft_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn sft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.user_nft(caller).set(&token_identifier);
            }
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
            }
        }
    }

    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, token_identifier: TokenIdentifier) {
        let caller = self.blockchain().get_caller();
        let user_nft_mapper = self.user_nft(&caller);
        // require!(
        //     !user_nft_mapper.is_empty(),
        //     "User does not have a token issued"
        // );
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &sc_address,
                &user_nft_mapper.get(),
                roles[..].iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }

    #[endpoint(createNft)]
    fn create_nft_with_attributes(&self) {
        let caller = self.blockchain().get_caller();
        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let user_nft_mapper = self.user_nft(&caller);
        require!(
            !user_nft_mapper.is_empty(),
            "User does not have a token issued"
        );
        let _ = self.send().esdt_nft_create_compact(
            &user_nft_mapper.get(),
            &BigUint::from(10u64),
            &attributes,
        );
    }

    ////////////////
    // Buy
    #[endpoint(buyNft)]
    #[payable("*")]
    fn buy_nft(&self, address: ManagedAddress, token_nft_nonce: u64) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        require!(
            self.user_token(&address).get() == payment.token_identifier,
            "Wrong token"
        );
        let user_nft_mapper = self.user_nft(&address);
        require!(!user_nft_mapper.is_empty(), "Empty");
        let user_nft_token = user_nft_mapper.get();
        self.send().esdt_local_burn(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
        self.send().direct_esdt(
            &caller,
            &user_nft_token,
            token_nft_nonce,
            &BigUint::from(1u64),
        );
    }

    ////////////////
    // Claim
    #[payable("EGLD")]
    #[endpoint(claim)]
    fn claim(&self, token: TokenIdentifier) {
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &token, 0, &BigUint::from(30u64));
    }

    #[endpoint(setUserPayment)]
    fn set_user_payment(&self, user: ManagedAddress, user_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let caller_token_mapper = self.user_token(&caller);
        require!(
            !caller_token_mapper.is_empty(),
            "User does not have a token issued"
        );
        require!(user_amount > 0, "Amount must be greater than 0");
        let token_id = caller_token_mapper.get();
        self.send().esdt_local_mint(&token_id, 0, &user_amount);
        let payment = EsdtTokenPayment::new(token_id, 0, user_amount);
        let user_payments_mapper = self.user_payments(&user);
        let mut user_payments = if user_payments_mapper.is_empty() {
            ManagedVec::new()
        } else {
            self.user_payments(&user).get()
        };
        user_payments.push(payment);
        user_payments_mapper.set(user_payments);
    }

    #[endpoint(claimUserPayments)]
    fn claim_user_payments(&self) {
        let caller = self.blockchain().get_caller();
        let user_payments_mapper = self.user_payments(&caller);
        if !user_payments_mapper.is_empty() {
            let user_payments = user_payments_mapper.get();
            self.send().direct_multi(&caller, &user_payments);
            user_payments_mapper.clear();
        }
    }

    #[view(getUserNft)]
    #[storage_mapper("userNft")]
    fn user_nft(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    // TO DO: SAVE TOKEN ID IN CALLBACK
    #[view(getUserToken)]
    #[storage_mapper("userToken")]
    fn user_token(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNftTokenToken)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(lastIssuedToken)]
    #[storage_mapper("lastIssuedToken")]
    fn last_issued_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("userPayments")]
    fn user_payments(
        &self,
        user: &ManagedAddress,
    ) -> SingleValueMapper<ManagedVec<EsdtTokenPayment<Self::Api>>>;
}

// fn issue(token_type: EsdtTokenType, issue_cost: BigUint, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer,initial_supply: BigUint, num_decimals: usize, opt_callback: Option<CallbackClosure>) -> !
