use std::collections::HashMap;

// Now over here in these we will define some types, these are not types per se, but
// just the aliases of a primitive type

type Account = u16;
type Balance = u32;

// Now what we are going to do is, we are going to change the usage here

pub struct BalanceModule {
    balances: HashMap<Account, Balance>, // so this is the object that can basically store some data
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

    pub fn set_balance(&mut self, who: Account, amount: Balance) { // we created this mut object of self, coz we need to update this
        self.balances.insert(who, amount); // so now let's go and write the test to see if it works
    }

    pub fn balance(&self, who: Account) -> Balance {
        *self.balances.get(&who).unwrap_or(&0)
        // it may not be that every key has a value, right ?
        // so we do unwrap_or(), to get a default value, if there is no key
    }

    pub fn transfer(&mut self, from: Account, to: Account, amount: Balance) -> Result<(), &'static str> { // It will return Ok with nothing inside of it, or may be an error string
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
So, see this is definitely a step further, but this is still not adding flexibility
to our code, we have just hard coded our types, we need that flexibility to add our
types in our runtime. 
*/