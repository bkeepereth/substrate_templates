#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc721 {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };

    use scale::{
        Decode,
        Encode,
    };

    pub type TokenId = u32;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Erc721 {
        name: String,
        symbol: String,
        /// Total token supply, set at init
        total_supply: u32,
        /// Mint count
        count: u32,
        /// Mapping from token to owner, 1-1
        token_owners: Mapping<TokenId, AccountId>,
        /// Mapping from owner to token count
        balances: Mapping<AccountId, u32>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        TokenExists,
        TokenNotFound,
        TokenSupplyLimit,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: TokenId,
    }

    impl Erc721 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, init_supply: u32) -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract, name, symbol, init_supply)
            })
        }

        fn new_init(&mut self, name: String, symbol: String, init_supply: u32) {
            self.name = name;
            self.symbol = symbol;
            self.total_supply = init_supply;
            self.count = 0;
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.to_string()
        }

        /// Returns the symbol of the token.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.to_string()
        }

        /// Returns the total supply of the token.
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// Returns the owner of the token.
        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.token_owners.get(&id)
        }

        /// Returns the number of tokens owned.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balance_of_or_zero(&owner)
        }

        /// Transfers the token from the caller to the given address.
        #[ink(message)]
        pub fn transfer(
            &mut self,
            destination: AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &destination, id)?;
            Ok(())
        }

        /// Creates a new token.
        #[ink(message)]
        pub fn mint(&mut self) -> Result<(), Error> {
            if self.count == self.total_supply {
                return Err(Error::TokenSupplyLimit)
            }

            let caller = self.env().caller();
            if caller == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            }

            self.count=self.count+1;
            let id = self.count;
            
            self.add_token_to(&caller, id)?;
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                id,
            });
            Ok(())
        }

        /// Destroys a token forever.
        #[ink(message)]
        pub fn burn(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            self.remove_token_from(&caller, id)?;
            self.env().emit_event(Transfer { 
                from: Some(caller),
                to: Some(AccountId::from([0x0; 32])),
                id,
            });
            Ok(())
        }

        /*
        /// WIP
        /// Returns a vector of TokenId's owned by an address.
        #[ink(message)]
        pub fn tokens_of_owner(&mut self, of: AccountId) -> Vec<AccountId> {
             // calls balanceOf
             let result: Vec<AccountId> = vec![];
             result
        }
        */

        /// Transfers token `id` `from` the sender to the `to` `AccountId`
        fn transfer_token_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            if !self.exists(id) {
                return Err(Error::TokenNotFound)
            }

            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id: id,
            });
            Ok(())
        }

        /// Removes token `id` from the owner.
        fn remove_token_from(
            &mut self,
            from: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let owner = self.token_owners.get(&id);
            if owner.is_none() {
                return Err(Error::TokenNotFound)
            }

            if owner != Some(*from) {
                return Err(Error::NotOwner)
            }

            let count = self.balances
                .get(&from)
                .map(|c| c - 1)
                .ok_or(Error::CannotFetchValue)?;
            self.balances.insert(&from, &count);
            self.token_owners.remove(&id);
            Ok(())
        }

        /// Adds the token `id` to the `to` AccountId.
        fn add_token_to(
            &mut self,
            to: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            if self.token_owners.get(&id).is_some() {
                return Err(Error::TokenExists)
            }

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)         
            }

            let count = self.balances
                .get(to)
                .map(|c| c + 1)
                .unwrap_or(1);
            self.balances.insert(to, &count);
            self.token_owners.insert(&id, to);
            Ok(())
        }

        /// Returns true if token `id` exists or false if it does not.
        fn exists(&self, id: TokenId) -> bool {
            self.token_owners.get(&id).is_some()
        }

        /// Returns the total number of tokens from an account.
        fn balance_of_or_zero(&self, of: &AccountId) -> u32 {
            self.balances.get(of).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn init_works() {
            // Create a new contract instance.
            let erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 50);
            // Check the token name
            assert_eq!(erc721.name(), "Test Token".to_string());
            // Check the token symbol
            assert_eq!(erc721.symbol(), "TEST".to_string());
            // Check token supply
            assert_eq!(erc721.total_supply(), 50);
        }

        #[ink::test]
        fn mint_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 50);
            // Check existence of TokenId = 1
            assert_eq!(erc721.owner_of(1), None);
            // Check Alice's balance
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Alice mints a token
            assert_eq!(erc721.mint(), Ok(()));
            // Check if Alice owns TokenId = 1
            assert_eq!(erc721.balance_of(accounts.alice), 1);
        }

        #[ink::test]
        fn mint_limit_works() {
            // Create a new contract instance.
            let mut erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 3);
            // Check existence of TokenId = 1, 2, 3
            assert_eq!(erc721.mint(), Ok(())); 
            assert_eq!(erc721.mint(), Ok(()));
            assert_eq!(erc721.mint(), Ok(()));
            assert_eq!(erc721.mint(), Err(Error::TokenSupplyLimit));
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 3);
            // Alice mints a token
            assert_eq!(erc721.mint(), Ok(()));
            // Check that Alice owns a token
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Check that Bob does not own a token
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // The first Transfer event (Alice's mint)
            assert_eq!(1, ink_env::test::recorded_events().count());
            // Alice transfers token to Bob
            assert_eq!(erc721.transfer(accounts.bob, 1), Ok(()));
            // The second Transfer event 
            assert_eq!(2, ink_env::test::recorded_events().count());
            // Bob owns token 1
            assert_eq!(erc721.balance_of(accounts.bob), 1);
        }

        #[ink::test]
        fn burn_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 50);
            // Alice mints a token
            assert_eq!(erc721.mint(), Ok(()));
            // Check if Alice owns TokenId = 1
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Burn TokenId = 1
            assert_eq!(erc721.burn(1), Ok(()));
            // Check existence of TokenId = 1
            assert_eq!(erc721.owner_of(1), None);
        }

        #[ink::test]
        fn burn_fails_token_not_found() {
            // Create a new contract instance.
            let mut erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 3);
            // Check if burn fails. 
            assert_eq!(erc721.burn(1), Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn burn_fails_not_owner() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new("Test Token".to_string(), "TEST".to_string(), 3);
            // Alice mints a token
            assert_eq!(erc721.mint(), Ok(()));
            // Try burning this token with a different account
            set_caller(accounts.eve);
            assert_eq!(erc721.burn(1), Err(Error::NotOwner));
        }

        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }
    }
}
