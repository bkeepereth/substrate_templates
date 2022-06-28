#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod psp34template {
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::{
         metadata::*,
         burnable::*,
         mintable::*,
         enumerable::*,
    };
    use ink_storage::traits::SpreadAllocate;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, PSP34EnumerableStorage)]
    pub struct PSP34Template {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[PSP34EnumerableStorageField]
        e_metadata: PSP34EnumerableData,
    }

    impl PSP34 for PSP34Template {
        /*
         * 1 collection_id(&self) -> Id;
         * 2 balance_of(&self, owner: AccountId) -> u32;
         * 3 owner_of(&self, id: Id) -> Option<AccountId>;
         * 4 allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;
         * 5 approve();
         * 6 transfer();
         * 7 total_supply();
         */
    }

    impl PSP34Transfer for PSP34Template {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            Ok(())
        }

        /*
        _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
        
        } 
        */
    }

    impl PSP34Enumerable for PSP34Template {
        //owners_token_by_index()
        //token_by_index()
    }

    impl PSP34Internal for PSP34Template {}  
    impl PSP34Metadata for PSP34Template {}  // exposes get_attribute()
    impl PSP34Burnable for PSP34Template {}  // exposes burn()
    impl PSP34Mintable for PSP34Template {}  // exposes mint()

    impl PSP34Template {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, supply_cap: String) -> Self {
            let name_label = String::from("name").into_bytes();
            let symbol_label = String::from("symbol").into_bytes();
            let cap_label = String::from("supply_cap").into_bytes();

            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                _instance._set_attribute(Id::U8(1u8), name_label, name.into_bytes());
                _instance._set_attribute(Id::U8(2u8), symbol_label, symbol.into_bytes());
                _instance._set_attribute(Id::U8(3u8), cap_label, supply_cap.into_bytes());
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn init_works() {
            // Create new contract instance.
            let psp34 = PSP34Template::new(
                String::from("TestToken"), 
                String::from("TestSymbol"), 
                String::from("100")
            );
            assert_eq!(psp34.total_supply(), 0);
        }

        #[ink::test]
        fn mint_works() {
            let accounts = brush::test_utils::accounts();
            // Create a new contract instance.
            let mut psp34 = PSP34Template::new(
                String::from("TestToken"), 
                String::from("TestSymbol"), 
                String::from("100")
            );
            // Check existence of TokenId 1
            assert_eq!(psp34.owner_of(Id::U8(1u8)), None);
            assert_eq!(psp34.balance_of(accounts.alice), 0);
            // Alice mints a token
            assert!(psp34.mint(accounts.alice, Id::U8(1u8)).is_ok());
            // Check that Alice owns TokenId 1
            assert_eq!(psp34.balance_of(accounts.alice), 1);
        }

        #[ink::test]
        fn mint_existing_should_fail() {
            let accounts = brush::test_utils::accounts();
            // Create a new contract instance.
            let mut psp34 = PSP34Template::new(
                String::from("TestToken"), 
                String::from("TestSymbol"), 
                String::from("100")
            );
            // Alice mints a token.
            assert!(psp34.mint(accounts.alice, Id::U8(1u8)).is_ok());
            // Check that Alice owns TokenId 1
            assert_eq!(psp34.balance_of(accounts.alice), 1);
            assert_eq!(psp34.owner_of(Id::U8(1u8)), Some(accounts.alice));
            // Attempt to re-mint TokenId 1
            assert_eq!(psp34.mint(accounts.bob, Id::U8(1u8)), Err(PSP34Error::TokenExists));
        }

        #[ink::test]
        fn burn_works() {
            let accounts = brush::test_utils::accounts();
            // Create new contract instance.
            let mut psp34 = PSP34Template::new(
                String::from("TestToken"), 
                String::from("TestSymbol"), 
                String::from("100")
            );
            // Alice mints a token.
            assert!(psp34._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
            // Check that Alice owns TokenId 1
            assert_eq!(psp34.balance_of(accounts.alice), 1);
            assert_eq!(psp34.owner_of(Id::U8(1u8)), Some(accounts.alice));
            // Alice burns a token.
            assert!(psp34.burn(accounts.alice, Id::U8(1u8)).is_ok());
            // Check that TokenId 1 was burned.
            assert_eq!(psp34.balance_of(accounts.alice), 0);
            assert_eq!(psp34.owner_of(Id::U8(1u8)), None);
        }

        #[ink::test]
        fn burn_not_existing_should_fail() {
            let accounts = brush::test_utils::accounts();
            // Create a new contract instance.
            let mut psp34 = PSP34Template::new(
                String::from("TestToken"), 
                String::from("TestSymbol"), 
                String::from("100")
            );
            // Attempt to burn Token Id 4
            assert_eq!(psp34.burn(accounts.alice, Id::U8(4u8)), Err(PSP34Error::TokenNotExists));
        }

        #[ink::test]
        fn metadata_works() {
            let psp34 = PSP34Template::new(
                String::from("TestToken"),
                String::from("TestSymbol"),
                String::from("100"));

            assert_eq!(
                psp34.get_attribute(Id::U8(1u8), String::from("name").into_bytes()),
                Some(String::from("TestToken").into_bytes())
            );

            assert_eq!(
                psp34.get_attribute(Id::U8(2u8), String::from("symbol").into_bytes()),
                Some(String::from("TestSymbol").into_bytes())
            );

            assert_eq!(
                psp34.get_attribute(Id::U8(3u8), String::from("supply_cap").into_bytes()),
                Some(String::from("100").into_bytes())
            );
        }

        #[ink::test]
        fn enumerable_mint_works() {
            let accounts = brush::test_utils::accounts();
            // Create a new contract instance.
            let mut psp34 = PSP34Template::new(
                String::from("TestToken"),
                String::from("TestSymbol"),
                String::from("100")
            );
            // Alice mints a token.
            assert!(psp34._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
            // Check owners_token_by_index
            assert_eq!(psp34.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
            // Check token_by_index
            assert_eq!(psp34.token_by_index(0u128), Ok(Id::U8(1u8)));
        }

        #[ink::test]
        fn enumerable_should_fail() {
            let accounts = brush::test_utils::accounts();
            // Create a new contract instance.
            let psp34 = PSP34Template::new(
                String::from("TestToken"),
                String::from("TestSymbol"),
                String::from("100")
            );
            // Attempt to get Alice's tokens
            assert_eq!(
                psp34.owners_token_by_index(accounts.alice, 0u128),
                Err(PSP34Error::TokenNotExists),
            );
            // Check that TokenId 1 does not exist
            assert_eq!(psp34.token_by_index(0u128), Err(PSP34Error::TokenNotExists));
        }
        
        #[ink::test]
        fn enumerable_burn_works() {
            let accounts = brush::test_utils::accounts();
            // Create a new contract balance.
            let mut psp34 = PSP34Template::new(
                String::from("TestToken"),
                String::from("TestSymbol"),
                String::from("100")
            );
            // Alice mints a token. 
            assert!(psp34._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
            // Check Alice's tokens
            assert_eq!(psp34.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
            // Index 0 = TokenId 1
            assert_eq!(psp34.token_by_index(0u128), Ok(Id::U8(1u8)));
            // Burn TokenId 1
            assert!(psp34.burn(accounts.alice, Id::U8(1u8)).is_ok());
            // Check Alice's tokens
            assert_eq!(
                psp34.owners_token_by_index(accounts.alice, 0u128),
                Err(PSP34Error::TokenNotExists)
            );
            // Check that TokenId 1 does not exist
            assert_eq!(psp34.token_by_index(0u128), Err(PSP34Error::TokenNotExists));
        }
    }
}
