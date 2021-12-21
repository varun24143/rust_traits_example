use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use crate::BalanceVoting; // this helps me import the trait that we just wrote 

pub struct BalanceModule<T: BalanceVoting> { // See the whole idea is that, I dont wanna define these
    // types again and again, its better if we define a single trait, which will have
    // all the associated types, I know I need. So here we will have T: Trait, which will
    // have a generic type, which must implement this 'Trait' trait. Ignore the naming for now
    // So before we go further, let me define this Trait trait, so lets actually go to main.rs
    // and define that trait which will have all those types
    balances: HashMap<T::AccountId, T::Balance>, 
}

// std::cmp::Eq + std::hash::Hash - So with this what we are trying to say is  that
// any accountId, which can be equal to other and also should be hashable coz we are using
// an entry for a HashMap
impl<T: BalanceVoting> BalanceModule<T> {
    
    pub fn new() -> Self { 
        Self { 
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) { 
        self.balances.insert(who, amount); 
    }

    pub fn balance(&self, who: T::AccountId) -> T::Balance {
        *self.balances.get(&who).unwrap_or(&T::Balance::zero())

    }

    pub fn transfer(&mut self, from: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> { 
        
        let zero = T::Balance::zero();
        let from_balance = self.balances.get(&from).ok_or("from user does not exist")?; 
        let to_balance = self.balances.get(&to).unwrap_or(&zero);
        let new_from_balance = from_balance.checked_sub(&amount).ok_or("user does not have enough funds")?; 
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

pub struct VotingModule<T: BalanceVoting> {
    votes: HashMap<(T::AccountId, T::VoteIndex), bool>,
}

impl<T: BalanceVoting> VotingModule<T> {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
        }
    }

    pub fn vote(&mut self, who: T::AccountId, index: T::VoteIndex, vote: bool) {
        self.votes.insert((who, index), vote);
    }

    pub fn get_vote(&self, who: T::AccountId, index: T::VoteIndex) -> bool {
        *self.votes.get(&(who, index)).unwrap_or(&false)
    }
}
