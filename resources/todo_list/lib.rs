#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod todo_list {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    pub type TodoItemId = i32;

    // Item priorities enum
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Prioritise {
        HIGH,
        LOW,
        MEDIUM,
    }

    // item todo error
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum TodoError {
        ItemNotExists,
        NotAOwner,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct TodoItem {
        owner: AccountId,
        item_name: String,
        is_completed: bool,
        priority: Prioritise,
    }

    impl Default for TodoItem {
        fn default() -> Self {
            Self {
                owner: zero_address(),
                item_name: Default::default(),
                is_completed: false,
                priority: Prioritise::HIGH,
            }
        }
    }

    // Events for toto item
    #[ink(event)]
    pub struct ItemCreated {
        #[ink(topic)]
        item: TodoItem,
    }

    #[ink(event)]
    pub struct ItemUpdated {
        #[ink(topic)]
        item: TodoItem,
    }

    #[ink(storage)]
    pub struct TodoList {
        owner: AccountId,
        item: Mapping<TodoItemId, TodoItem>,
        item_id: i32,
    }

    impl TodoList {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            Self {
                owner,
                item: Mapping::default(),
                item_id: 1,
            }
        }

        #[ink(message)]
        pub fn create_todo(
            &mut self,
            item_name: String,
            priority: Prioritise,
        ) -> Result<(), TodoError> {
            let caller = self.env().caller();
            let item_id = self.get_item_id();

            let item = TodoItem {
                owner: caller,
                item_name,
                is_completed: false,
                priority,
            };

            self.item.insert(item_id, &item);
            self.env().emit_event(ItemCreated { item });
            Ok(())
        }

        #[ink(message)]
        pub fn update_item(&mut self, item_id: TodoItemId) -> Result<(), TodoError> {
            let item = self.item.get(item_id);
            let caller = self.env().caller();

            match item {
                Some(value) => {
                    // check wether the item belongs owner or not
                    if value.owner != caller {
                        return Err(TodoError::NotAOwner);
                    }

                    let item = TodoItem {
                        owner: caller,
                        item_name: value.item_name,
                        is_completed: true,
                        priority: value.priority,
                    };
                    self.item.insert(item_id, &item);
                    self.env().emit_event(ItemUpdated { item });
                }
                None => return Err(TodoError::ItemNotExists),
            }
            Ok(())
        }

        #[ink(message)]
        pub fn get_my_todo(&self, account: AccountId) -> Vec<TodoItem> {
            let mut item: Vec<TodoItem> = Vec::new();
            for _item in 0..self.item_id {
                match self.item.get(_item) {
                    Some(value) => {
                        if value.owner == account {
                            item.push(value);
                        }
                    }
                    None => (),
                }
            }
            item
        }

        #[ink(message)]
        pub fn get_all_todo(&self) -> i32 {
            let mut item: Vec<TodoItem> = Vec::new();
            for _item in 0..self.item_id {
                match self.item.get(_item) {
                    Some(value) => {
                        item.push(value);
                    }
                    None => (),
                }
            }
            item.len() as i32
        }

        // Item next Id
        pub fn get_item_id(&mut self) -> TodoItemId {
            let id = self.item_id;
            self.item_id += 1;
            id
        }
    }

    fn zero_address() -> AccountId {
        [0u8; 32].into()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }

        #[ink::test]
        fn register_works() {
            let accounts = default_accounts();
            set_next_caller(accounts.alice);

            let contract = TodoList::new();
            assert_eq!(contract.owner, accounts.alice);
        }

        #[ink::test]
        fn create_todo_works() {
            let accounts = default_accounts();
            set_next_caller(accounts.alice);
            let mut contract = TodoList::new();

            let name = "Item One".to_owned();
            contract
                .create_todo(name, Prioritise::HIGH)
                .unwrap_or_default();

            assert_eq!(contract.get_all_todo(), 1);

            set_next_caller(accounts.bob);

            let name = "Item two".to_owned();
            contract
                .create_todo(name, Prioritise::LOW)
                .unwrap_or_default();
            assert_eq!(contract.get_all_todo(), 2);
        }

        #[ink::test]
        pub fn update_item() {
            let accounts = default_accounts();
            set_next_caller(accounts.alice);
            let mut contract = TodoList::new();

            let name = "Item One".to_owned();
            contract
                .create_todo(name, Prioritise::HIGH)
                .unwrap_or_default();

            contract.update_item(1).unwrap_or_default();

            let new_contract = contract.item.get(1).unwrap_or_default();

            assert_eq!(new_contract.is_completed, true);
        }
    }
}
