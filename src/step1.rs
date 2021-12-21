use std::collections::HashMap;

pub struct BalanceModule {
    balances: HashMap<u32, u32>, // so this is the object that can basically store some data
}
/*
Now we need to implement some functions for it right

*/

impl BalanceModule {
    // first function is going to be an initializer
    pub fn new() -> Self { // This basically returns itself
        Self { // this is for creating an empty version of BalanceModule
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: u32, amount: u32) { // we created this mut object of self, coz we need to update this
        self.balances.insert(who, amount); // so now let's go and write the test to see if it works
    }

    pub fn balance(&self, who: u32) -> u32 {
        *self.balances.get(&who).unwrap_or(&0)
        // it may not be that every key has a value, right ?
        // so we do unwrap_or(), to get a default value, if there is no key
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: u32) -> Result<(), &'static str> { // It will return Ok with nothing inside of it, or may be an error string
        //let from_balance = self.balances.get(&from).unwrap_or(&0);
        let from_balance = self.balances.get(&from).ok_or("from user does not exist")?; // This ok_or() is returning the result, and I want to unwrap the value, 
        //so if I add this question mark here, it will take this result object, which either has an error, 
        // or Ok with a value in it. So I am sure, every one new to rust would be wondering what is this question mark operator here trying to do, this is basically a match
        // expression where you are either matching it to an Error or an Ok, this is equivalent to
        // let from_balance = match self.balances.get(&from).ok_or("from user does not exist") {
            // Err(e) => return Err(e.into()),
            // Ok(value) => value, // spits out the value and is given to from_balance
        // }
        //let to_balance = self.balances.get(&to).ok_or("to user does not exist")?;
        let to_balance = self.balances.get(&to).unwrap_or(&0);
        let new_from_balance = from_balance.checked_sub(amount).ok_or("user does not have enough funds")?; // using checked_sub to check for no overflowing and underflowing
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

/*
So, now let's do everything step by step by creating some framework
basically what I am going to do here is that I will go to main.rs
and test whatever I have written already

*/
/*
well all this is not that interesting, we need to add more functionalities
Now we will see how to move, add, transfer things, but before we do all this
we need to set some balances. SO we will create a backdoor that will allow us
to set any balances of any user
*/

/*
Now we have our BalancesModule ready, doing its work, but what is so irritating about this code,
it has like u32's everywhere, it is not generalized enough. Now we need something to iterate on this
let's make a copy of this and make it step2.rs
*/