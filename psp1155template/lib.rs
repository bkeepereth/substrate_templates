#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod psp1155template {
    use brush::contracts::psp1155::*;
    use brush::contracts::psp1155::extensions::{
        mintable::*,
        burnable::*,
        metadata::*,
    };
    use ink_storage::traits::SpreadAllocate;
    use ink_lang::codegen::{Env, EmitEvent};

    #[ink(event)]
    pub struct TransferSingle {
        operator: AccountId,
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        amount: Balance,
    }

    #[ink(event)]
    pub struct TransferBatch {
        operator: AccountId,
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        ids_amounts: Vec<(Id, Balance)>,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP1155Storage, PSP1155MetadataStorage)]
    pub struct PSP1155Template {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
    }

    impl PSP1155Transfer for PSP1155Template {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
             Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            Ok(())
        }
    }

    impl PSP1155Internal for PSP1155Template {
        fn _emit_transfer_single_event(
            &self,
            _operator: AccountId,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
            _amount: Balance,
        ) {
            self.env().emit_event(TransferSingle {
                operator: _operator,
                from: _from,
                to: _to,
                id: _id,
                amount: _amount,
            });
        }

        fn _emit_transfer_batch_event(
            &self,
            _operator: AccountId,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _ids_amounts: Vec<(Id, Balance)>,
        ) {
            self.env().emit_event(TransferBatch {
                operator: _operator,
                from: _from,
                to: _to,
                ids_amounts: _ids_amounts,
            });
        }

        fn _emit_approval_for_all_event(
            &self,
            _owner: AccountId,
            _operator: AccountId,
            _approved: bool,
        ) {
            self.env().emit_event(ApprovalForAll {
                owner: _owner,
                operator: _operator,
                approved: _approved,
            });
        }

        fn _do_safe_transfer_check(
            &mut self,
            _operator: &AccountId,
            _from: &AccountId,
            _to: &AccountId,
            _ids_amounts: &Vec<(Id, Balance)>,
            _data: &Vec<u8>,
        ) -> Result<(), PSP1155Error> {
            Ok(())
        }
    }

    impl PSP1155 for PSP1155Template {
        /* balance_of()
         * balance_of_batch()
         * set_approval_for_all()
         * is_approved_for_all()
         * transfer_from()
         * batch_transfer_from()
         */
    }

    impl PSP1155Mintable for PSP1155Template { /* mint() */ }
    impl PSP1155Burnable for PSP1155Template { /* burn() */ }
    impl PSP1155Metadata for PSP1155Template { /* uri() */ }

    impl PSP1155Template {
        #[ink(constructor)]
        pub fn new(uri: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                _instance.metadata.uri = uri;
            })
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint_to(acc, vec![(id, amount)])
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use brush::test_utils::*;

        type Event = <PSP1155Template as ::ink_lang::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn balance_of_works() {
            let token_id_1 = [1; 32];
            let mint_amount = 5;
            let accounts = brush::test_utils::accounts();
            
            // Create a new contract instance
            let mut psp1155 = PSP1155Template::new(Some(String::from("testURI/")));
            // Check supply
            assert_eq!(psp1155.balance_of(accounts.alice, token_id_1), 0);
            // Alice mints some tokens
            assert!(psp1155.mint(accounts.alice, token_id_1, mint_amount).is_ok());

            let mut events_iter = ink_env::test::recorded_events();
            let emitted_event = events_iter.next().unwrap();
            assert_transfer_event(
                emitted_event,
                accounts.alice,
                None,
                Some(accounts.alice),
                token_id_1,
                mint_amount,
            );
            assert_eq!(ink_env::test::recorded_events().count(), 1);
        }

        #[ink::test]
        fn balance_of_batch_works() {
            let token_id_1 = [1; 32];
            let token_id_2 = [2; 32];
            let token_1_amount = 1;
            let token_2_amount = 20;
            let accounts = accounts();
            let accounts_ids = vec![(accounts.alice, token_id_1), (accounts.alice, token_id_2)];
            
            // Create a new contract instance
            let mut psp1155 = PSP1155Template::new(Some(String::from("testURI/")));
            // Check Alice's balance
            assert_eq!(psp1155.balance_of_batch(accounts_ids.clone()), vec![0, 0]);
            // Alice mints tokenId 1
            assert!(psp1155.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
            // Check Alice's balance
            assert_eq!(psp1155.balance_of_batch(accounts_ids.clone()), vec![token_1_amount, 0]);
            // Alice mints tokenId 2
            assert!(psp1155.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
            // Check Alice's balance
            assert_eq!(psp1155.balance_of_batch(accounts_ids.clone()), vec![token_1_amount, token_2_amount]);
            
            let mut events_iter = ink_env::test::recorded_events();
            let emitted_event = events_iter.next().unwrap();
            assert_transfer_event(
                emitted_event,
                accounts.alice,
                None,
                Some(accounts.alice),
                token_id_1,
                token_1_amount,
            );

            let emitted_event = events_iter.next().unwrap();
            assert_transfer_event(
                emitted_event,
                accounts.alice,
                None,
                Some(accounts.alice),
                token_id_2,
                token_2_amount,
            );
            assert_eq!(ink_env::test::recorded_events().count(), 2);
        }

        #[ink::test]
        fn mint_works() {
            let token_id_1 = [1; 32];
            let mint_amount = 22;
            let accounts = brush::test_utils::accounts();

            // Create a new contract instance
            let mut psp1155 = PSP1155Template::new(Some(String::from("testURI/")));
            // Alice mints some tokens
            assert!(psp1155.mint(accounts.alice, token_id_1, mint_amount).is_ok());
            // Check Alice's balance
            assert_eq!(psp1155.balance_of(accounts.alice, token_id_1), mint_amount);
        }

        #[ink::test]
        fn burn_works() {
            let token_id_1 = [1; 32];
            let mint_amount = 5;
            let burn_amount = 2;
            let accounts = brush::test_utils::accounts();

            // Create a new contract instance
            let mut psp1155 = PSP1155Template::new(Some(String::from("testURI/")));
            // Alice mints some tokens
            assert!(psp1155.mint(accounts.alice, token_id_1, mint_amount).is_ok());
            // Check Alice's balance
            assert_eq!(psp1155.balance_of(accounts.alice, token_id_1), mint_amount);
            // Alice burns some tokens
            assert!(psp1155.burn(accounts.alice, vec![(token_id_1, burn_amount)]).is_ok());
            // Check Alice's balance
            assert_eq!(psp1155.balance_of(accounts.alice, token_id_1), mint_amount - burn_amount);
        }

        #[ink::test]
        fn burn_insufficient_balance() {
            let token_id_1 = [1; 32];
            let burn_amount = 2;
            let accounts = brush::test_utils::accounts();
            
            // Create a new contract instance
            let mut psp1155 = PSP1155Template::new(Some(String::from("testURI/")));
            // Attempt to burn 2 tokens
            assert_eq!(
                psp1155.burn(accounts.alice, vec![(token_id_1, burn_amount)]),
                Err(PSP1155Error::InsufficientBalance),
            );
        }

        #[ink::test]
        fn metadata_works() {
            // Create a new contract instance
            let mut psp1155 = PSP1155Template::new(Some(String::from("testURI/")));
            // Validate URI
            assert_eq!(psp1155.uri([0; 32]), Some(String::from("testURI/")));
        }

        fn assert_transfer_event(
            event: ink_env::test::EmittedEvent,
            expected_operator: AccountId,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_id: Id,
            expected_amount: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered valid contract event data buffer");
            if let Event::TransferSingle(TransferSingle {
                operator,
                from,
                to,
                id,
                amount,
            }) = decoded_event {
                assert_eq!(operator, expected_operator, 
                           "encountered invalid TransferSingle.operator"
                );
                assert_eq!(from, expected_from, "encountered invalid TransferSingle.from");
                assert_eq!(to, expected_to, "encountered invalid TransferSingle.to");
                assert_eq!(id, expected_id, "encountered invalid TransferSingle.id");
                assert_eq!(amount, expected_amount, "encountered invalid TransferSingle.amount");
            } else {
                panic!("encountered unexpected event kind: expected a TransferSingle event");
            }
        }
    }
}
