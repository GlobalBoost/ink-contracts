#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod counter {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Counter {
     // Storage Declaration
     value: i32,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn get_counter(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn increment_counter(&mut self) {
            self.value += 1;
        }

        #[ink(message)]
        pub fn decrement_counter(&mut self) {
            self.value -= 1;
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let counter = Counter::default();
            assert_eq!(counter.get_counter(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn increment_works() {
            let mut counter = Counter::new(1);
            assert_eq!(counter.get_counter(), 1);
            counter.increment_counter();
            assert_eq!(counter.get_counter(), 2);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn decrement_works() {
            let mut counter = Counter::new(1);
            assert_eq!(counter.get_counter(), 1);
            counter.decrement_counter();
            assert_eq!(counter.get_counter(), 0);
        }
    }

}
