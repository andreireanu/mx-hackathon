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
    fn init(&self) {
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]  
    fn issue_and_set_all_roles(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value();
        self.nft_token_mapper().issue_and_set_all_roles(
            EsdtTokenType::SemiFungible,
            issue_cost,
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    // #[payable("EGLD")]
    // #[endpoint(issueFungibleToken)]  
    // fn issue_fungible_token(
    //     &self,
    //     token_display_name: ManagedBuffer,
    //     token_ticker: ManagedBuffer,
    //     initial_supply: BigUint,
    // ) {
    //     let issue_cost = self.call_value().egld_value();
    //     let caller = self.blockchain().get_caller();

    //     self.send()
    //         .esdt_system_sc_proxy()
    //         .issue_fungible(
    //             issue_cost,
    //             &token_display_name,
    //             &token_ticker,
    //             &initial_supply,
    //             FungibleTokenProperties {
    //                 num_decimals: 0,
    //                 can_freeze: true,
    //                 can_wipe: true,
    //                 can_pause: true,
    //                 can_mint: true,
    //                 can_burn: true,
    //                 can_change_owner: true,
    //                 can_upgrade: true,
    //                 can_add_special_roles: true,
    //             },
    //         )
    //         .async_call()
    //         .with_callback(self.callbacks().esdt_issue_callback(&caller))
    //         .call_and_exit()
    // }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // self.last_issued_token().set(&token_identifier);
                // self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

                // self.last_error_message().set(&message.err_msg);
            },
        }
    }

    // #[only_owner]
    // #[payable("EGLD")]
    // #[endpoint(issueToken)]
    // fn issue_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
    //     let payment_amount = self.call_value().egld_value();
    //     self.send()
    //         .esdt_system_sc_proxy()
    //         .issue_semi_fungible(
    //             payment_amount,
    //             &token_name,
    //             &token_ticker,
    //             SemiFungibleTokenProperties {
    //                 can_freeze: true,
    //                 can_wipe: true,
    //                 can_pause: true,
    //                 can_transfer_create_role: true,
    //                 can_change_owner: false,
    //                 can_upgrade: false,
    //                 can_add_special_roles: true,
    //             },
    //         )
    //         .async_call()
    //         .with_callback(self.callbacks().issue_callback())
    //         .call_and_exit()
    // }

    // #[callback]
    // fn issue_callback(
    //     &self,
    //     #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    // ) {
    //     match result {
    //         ManagedAsyncCallResult::Ok(token_id) => {
    //             self.nft_token_id().set(&token_id.unwrap_esdt());
    //         },
    //         ManagedAsyncCallResult::Err(_) => {
    //             let caller = self.blockchain().get_owner_address();
    //             let returned = self.call_value().egld_or_single_esdt();
    //             if returned.token_identifier.is_egld() && returned.amount > 0 {
    //                 self.send()
    //                     .direct(&caller, &returned.token_identifier, 0, &returned.amount);
    //             }
    //         },
    //     }
    // }

    #[endpoint(createNft)]
    fn create_nft_with_attributes(
        &self
    )   {
        let caller = self.blockchain().get_caller();
        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let user_nft_mapper = self.user_nft(&caller);
        require!( !user_nft_mapper.is_empty(), "User does not have a token issued");
        let _ = self.send().esdt_nft_create_compact(
            &user_nft_mapper.get(),
            &BigUint::from(10u64),
            &attributes,
        );
        
        
        
        // let _ = self.nft_token_mapper().nft_create(
        //     BigUint::from(NFT_AMOUNT),
        //     &attributes,
        // );
        // let mut serialized_attributes = ManagedBuffer::new();
        // if let core::result::Result::Err(err) = attributes.creation_timestamp.top_encode(&mut serialized_attributes) {
        //     sc_panic!("Attributes encode error: {}", err.message_bytes());
        // }

        // let attributes = ExampleAttributes {
        //     creation_timestamp: self.blockchain().get_block_timestamp(),
        // };
        // let mut serialized_attributes = ManagedBuffer::new();
        // let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        // let attributes_hash = attributes_sha256.as_managed_buffer();
        // let uri: ManagedBuffer = "URL_to_decentralized_storage/song.mp3".into();
        // let uris = ManagedVec::from_single_item(uri);
        // let nft_nonce = self.send().esdt_nft_create(
        //     &nft_token_id,
        //     &BigUint::from(NFT_AMOUNT),
        //     &name,
        //     &BigUint::from(ROYALTIES),
        //     attributes_hash,
        //     &attributes,
        //     &uris,
        // ); 
    }


    #[endpoint(setLocalRoles)]
    fn set_local_roles(
        &self,
        token_identifier: TokenIdentifier,
    ) {
        let caller = self.blockchain().get_caller();
        let user_nft_mapper = self.user_nft(&caller);
        require!( !user_nft_mapper.is_empty(), "User does not have a token issued");
        let sc_address = self.blockchain().get_sc_address();
        let roles = [EsdtLocalRole::NftCreate, EsdtLocalRole::NftAddQuantity, EsdtLocalRole::NftBurn];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, &user_nft_mapper.get(), roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    #[payable("EGLD")]
    #[endpoint(sftIssue)]
    fn sft_issue(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        require!(self.user_nft(&caller).is_empty(), "User already has an SFT issued");
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
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

            },
        }
    }

    #[view(getUserNft)]
    #[storage_mapper("userNft")]
    fn user_nft(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;   

    #[view(lastIssuedToken)]
    #[storage_mapper("lastIssuedToken")]
    fn last_issued_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("nftTokenId")]
    fn nft_token_mapper(&self) -> NonFungibleTokenMapper;
 

}


// fn issue(token_type: EsdtTokenType, issue_cost: BigUint, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer,initial_supply: BigUint, num_decimals: usize, opt_callback: Option<CallbackClosure>) -> !