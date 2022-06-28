#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod psp22template {
    use brush::contracts::psp22::*;
    use brush::contracts::psp22::extensions::{
        mintable::*,
        burnable::*,
        metadata::*,
    };
    use ink_storage::traits::SpreadAllocate;
    use ink_lang::codegen::{Env, EmitEvent};

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct PSP22Template {
        count: u32,
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22Transfer for PSP22Template {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
             Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            Ok(())
        }
    }

    impl PSP22Internal for PSP22Template {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance
        ) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        fn _emit_approval_event(
            &self,
            _owner: Option<AccountId>,
            _spender: Option<AccountId>,
            _amount: Balance,
        ) {
            self.env().emit_event(Approval {
                owner: _owner,
                operator: _operator,
                amount: _amount,
            });
        }

        fn _do_safe_transfer_check(
            &mut self,
            _from: &AccountId,
            _to: &AccountId,
            _value: &Balance,
            _data: &Vec<u8>,
        ) -> Result<(), PSP22Error> {}
    }

    impl PSP22 for PSP22Template {
        /* 
         * total_supply()
         * balance_of()
         * allowance()
         * transfer()
         * transfer_from()
         * approve()
         * increase_allowance()
         * decrease_allowance()
         *
         */
    }

    impl PSP22Mintable for PSP22Template { /* mint() */ }
    impl PSP22Burnable for PSP22Template { /* burn() */ }
   
    impl PSP22Metadata for PSP22Template { 
        /* 
         * token_name()
         * token_symbol()
         * token_decimals()
         *
         */ 
    }

    impl PSP22Template {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                _instance.metadata.name = name;
                _instance.metadata.symbol = symbol;
                _instance.metadata.decimals = decimal;
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use brush::test_utils::*;

        type Event = <PSP22Template as ::ink_lang::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn init_works() {
            // Create a new contract instance
            let psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18); 
            // Check metadata fields
            assert_eq!(psp22.token_name(), Some(String::from("TestToken")));
            assert_eq!(psp22.token_symbol(), Some(String::from("TEST")));
            assert_eq!(psp22.token_decimals(), 18);
        }

        #[ink::test]
        fn mint_to_zero_address_fails() {
            // Create a new contract instance
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let supply = 1000;
            // Attempt mint to zero address
            assert_eq!(
                psp22.mint(AccountId::from([0; 32]), supply),
                Err(PSP22Error::ZeroRecipientAddress)
            );
        }

        #[ink::test]
        fn total_supply_works() {
            // Create a new contract instance
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_mint = 1000;
            // Alice mints some tokens
            assert!(psp22.mint(accounts.alice, amount_to_mint).is_ok());
            // Bob mints some tokens
            assert!(psp22.mint(accounts.bob, amount_to_mint-324).is_ok());
            // Check for the emitted event
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(&emitted_events[0], None, Some(accounts.alice), 1000);
            assert_transfer_event(&emitted_events[1], None, Some(accounts.bob), 676);
            // 676 + 1000
            assert_eq!(psp22.total_supply(), 1676);
        }

        #[ink::test]
        fn balance_of_works() {
            // Create a new contract instance
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_mint = 1000;
            // Alice mints some tokens
            assert!(psp22.mint(accounts.alice, amount_to_mint).is_ok());
            // Check balance of Alice
            assert_eq!(psp22.balance_of(accounts.alice), 1000);
            // Bob mints some tokens
            assert!(psp22.mint(accounts.bob, amount_to_mint).is_ok());
            // Check balance of Bob
            assert_eq!(psp22.balance_of(accounts.bob), 1000);
        }

        #[ink::test]
        fn total_supply_increases_after_minting() {
            // Create a new contract instance
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();

            // Alice's balance before minting
            let account_balance = psp22.balance_of(accounts.alice);
            let amount_to_mint = 1000;
            assert!(psp22.mint(accounts.alice, amount_to_mint).is_ok());

            // Alice's balance after minting
            let new_account_balance = psp22.balance_of(accounts.alice);
            assert_eq!(new_account_balance, account_balance + amount_to_mint);
        }

        #[ink::test]
        fn should_emit_transfer_event_after_mint() {
            // Create a new contract instance
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_mint = 1000;
            // Alice mints some tokens
            assert!(psp22.mint(accounts.bob, amount_to_mint).is_ok());

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(accounts.bob),
                1000,
            );
            assert_eq!(emitted_events.len(), 1);
        }

        #[ink::test]
        fn burn_no_balance_fails() {
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_burn = 1000;
            // Burn should fail, empty wallet
            assert_eq!(
                psp22.burn(accounts.alice, amount_to_burn),
                Err(PSP22Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn should_emit_transfer_event_after_burn() {
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_mint = 1000;
            let amount_to_burn = 300;
            // Alice mints some tokens
            assert!(psp22.mint(accounts.alice, amount_to_mint).is_ok());
            // Check Alice's balance
            assert_eq!(psp22.balance_of(accounts.alice), 1000);
            // Alice burns 300 tokens
            assert!(psp22.burn(accounts.alice, amount_to_burn).is_ok());
            // Check Alice's balance
            assert_eq!(psp22.balance_of(accounts.alice), 700);
            
            // Count events
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Mint event 
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                amount_to_mint,
            );
            // Burn event
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                None,
                amount_to_burn,
            );
        }

        #[ink::test]
        fn total_supply_decreases_after_burning() {
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_mint = 1000;
            let amount_to_burn = 250;
            
            // Alice mints some tokens  
            assert!(psp22.mint(accounts.alice, amount_to_mint).is_ok());
            // Check total_supply
            assert_eq!(psp22.total_supply(), amount_to_mint);
            // Alice burns some tokens
            assert!(psp22.burn(accounts.alice, amount_to_burn).is_ok());
            // Check total_supply
            assert_eq!(psp22.total_supply(), amount_to_mint - amount_to_burn);
        }

        #[ink::test]
        fn burn_works() {
            let mut psp22 = PSP22Template::new(Some(String::from("TestToken")), Some(String::from("TEST")), 18);
            let accounts = brush::test_utils::accounts();
            let amount_to_mint = 1000;
            let amount_to_burn = 340;

            // Alice mints some tokens
            assert!(psp22.mint(accounts.alice, amount_to_mint).is_ok());
            // Check Alice's balance
            assert_eq!(psp22.balance_of(accounts.alice), amount_to_mint);
            // Alice burns some tokens
            assert!(psp22.burn(accounts.alice, amount_to_burn).is_ok());
            // Check Alice's balance
            assert_eq!(psp22.balance_of(accounts.alice), amount_to_mint - amount_to_burn);
        }

        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            let Event::Transfer(Transfer { from, to, value }) = decoded_event;
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(value, expected_value, "encountered invalid Transfer.value");

            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"PSP22Template::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"PSP22Template::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"PSP22Template::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"PSP22Template::Transfer::value",
                    value: &expected_value,
                }),
            ];

            for (n, (actual_topic, expected_topic)) in event.topics.iter().zip(expected_topics).enumerate() {
                assert_eq!(
                    &actual_topic[..],
                    expected_topic.as_ref(),
                    "encountered invalid topic as {}",
                    n
                );
            }
        }
    }
}
