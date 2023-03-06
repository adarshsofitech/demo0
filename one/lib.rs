#![cfg_attr(not(feature = "std"), no_std)]

pub use self::one::{One,OneRef};

#[ink::contract]
pub mod one {
    use two::TwoRef;
    #[ink(storage)]
    pub struct One {
        
        strength: u16,

    }
    

    impl One {
        #[ink(constructor)]
        pub fn new(
            s: u16
        ) -> Self {

            One{
                strength: s
            }
            
        }
        
        #[ink(message)]
        pub fn change(&mut self,l: u16) {
             self.strength = l;
        }
        #[ink(message)]
        pub fn show(&self) -> u16 {
             self.strength
        }

        #[ink(message)]
        pub fn change_two(&mut self,two: AccountId,l: u16) {
            let mut one_instance: TwoRef = ink::env::call::FromAccountId::from_account_id(two);
            one_instance.change(l);
        }

    }


}