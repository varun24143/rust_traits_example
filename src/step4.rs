use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub struct BalanceModule<AccountId, Balance> {
    balances: HashMap<AccountId, Balance>, 
}

// std::cmp::Eq + std::hash::Hash - So with this what we are trying to say is  that
// any accountId, which can be equal to other and also should be hashable coz we are using
// an entry for a HashMap
impl<AccountId: Eq + Hash, Balance: Zero + Eq + Hash + CheckedAdd + CheckedSub + Copy> BalanceModule<AccountId, Balance> {
    
    pub fn new() -> Self { 
        Self { 
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, amount: Balance) { 
        self.balances.insert(who, amount); 
    }

    pub fn balance(&self, who: AccountId) -> Balance {
        *self.balances.get(&who).unwrap_or(&Balance::zero())

    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> { 
        
        let zero = Balance::zero();
        let from_balance = self.balances.get(&from).ok_or("from user does not exist")?; 
        let to_balance = self.balances.get(&to).unwrap_or(&zero);
        let new_from_balance = from_balance.checked_sub(&amount).ok_or("user does not have enough funds")?; 
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

pub struct VotingModule<AccountId, VoteIndex> {
    votes: HashMap<(AccountId, VoteIndex), bool>,
}

impl<AccountId: Eq + Hash, VoteIndex: Eq + Hash> VotingModule<AccountId, VoteIndex> {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
        }
    }

    pub fn vote(&mut self, who: AccountId, index: VoteIndex, vote: bool) {
        self.votes.insert((who, index), vote);
    }

    pub fn get_vote(&self, who: AccountId, index: VoteIndex) -> bool {
        *self.votes.get(&(who, index)).unwrap_or(&false)
    }
}
