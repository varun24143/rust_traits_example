/*
In order to understand things further, let's create another module, which could be
a voting module, this will be very similar to BalancesModule

*/

use std::collections::HashMap;

type AccountId = u64;
type VoteIndex = u64;

pub struct VotingModule {
    votes: HashMap<(AccountId, VoteIndex), bool>,
}

impl VotingModule {
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

// Now let's quickly do some testing on votingModule to see if it works or not