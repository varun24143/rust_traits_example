use std::{thread::AccessError, hash::Hash};
use std::cmp::Eq;

use num::{CheckedAdd, CheckedSub};

/*
So what we are going to learn today is pure Rust
rather than jumping directly into substrate magic.
This is done in order to make all of us get comfortable
with Rust first and then dive deeper into Substrate
from the next seminar.
This I believe will be very helpful. And then may be
we will take a look at substrate real quick just to
grasp a couple of things.

So basically we will start with creating some rust modules
and rust structs, which can act as kind of different pallets
So, let's do some live coding, where we might run
into some errors as well, we will learn how to debug them as well
So, instead of directly using this main function,
let's create a new module and name it as step1.rs
Sp, what are we going to do here is that we will creating
a balances module, we will create a simple storage item, which
will keep a track of all the balances of all the users and few functions
that will help us manipulate these balances.
*/


mod step1;
mod step2;
mod step3;
mod step4;
mod step5;

fn main() {
    println!("Hello, world!");
}
 

/*
Rust provides a beautiful feature of attributes
*/

#[test]
fn test_step_1() {
    // Basically we will create a new instance of the structure
    let mut balances = step1::BalanceModule::new();

    // after adding set_balance function
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    // now we need to check what is the balance of the user, so for that
    // we will write another function
    // Now let's query the storage
    assert!(balances.balance(1) == 100);
    assert!(balances.balance(2) == 200);
    assert!(balances.balance(3) == 0);

 
// Now we need to add some basic transfer functionality
// let's go back to step1.rs

    assert!(balances.transfer(1, 2, 50).is_ok());

}

#[test]
fn test_step_2() {
    // Basically we will create a new instance of the structure
    let mut balances = step2::BalanceModule::new();

    // after adding set_balance function
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    // now we need to check what is the balance of the user, so for that
    // we will write another function
    // Now let's query the storage
    assert!(balances.balance(1) == 100);
    assert!(balances.balance(2) == 200);
    assert!(balances.balance(3) == 0);

 
// Now we need to add some basic transfer functionality
// let's go back to step1.rs

    assert!(balances.transfer(1, 2, 50).is_ok());

}

#[test]
fn test_step_3() {
    let mut voting = step3::VotingModule::new();

    voting.vote(1, 0, true);
    voting.vote(2, 0, false);

    assert!(voting.get_vote(1, 0) == true);
    assert!(voting.get_vote(2, 0) == false);
}

/*
So, all the tests have been passed here, but if you noticed something in these, well
let us actually ty with a function first
 */ 

//  fn wont_work() {
//      let user_1 = 1;
//      let user_2 = 2;

//      let balances = step2::BalanceModule::new();
//      let voting = step3::VotingModule::new();

//      balances.set_balance(user_1, 100);
//      assert!(balances.balance(user_1) == 100);
    
//      voting.vote(user_1, 1, true);
//  }

 /*
 So, see now this throws an error of type mismatching, but this could be resolved
 if you do try_into and other things, but its always good and clean to make your
 types as generics which we would do further, so that a function is of any type, 
 so let's get onto that actually now. Lets create another step as step4, which will
 combine both BalancesModule and VotingModule
 */

 /*
 Test for step4
 */

 #[test]
 fn test_step_4() {
     type AccountId = u32;
     type Balance = u32;
     type VoteIndex = u32;

     let user_1: AccountId = 1;
     let user_2: AccountId = 2;

     let mut balances = step4::BalanceModule::<AccountId, Balance>::new();
     let mut voting = step4::VotingModule::<AccountId, VoteIndex>::new();

     balances.set_balance(user_1, 100);
     balances.set_balance(user_2, 200);

     voting.vote(user_1, 1, true);
     voting.vote(user_2, 2, true);

     assert!(balances.balance(user_1) == 100);
     assert!(voting.get_vote(user_1, 1) == true);
 }


 /*
 Now let's say that we tried to make all the functions and types to be generic, but
 we need to make a trait like the one we did while asking our accountId and balance types to have
Eq, hash, CheckedAdd, these all have the functions which are implemented for the types
Okay, lets not get overwhelmed and confused with this, let's understand the concept of a trait first
let's go to step5.rs and start actually
 */

use num::traits::{Zero};

 pub trait BalanceVoting {
     type AccountId: Eq + Hash;
     type Balance: Zero + CheckedAdd + CheckedSub + Copy;
     type VoteIndex: Eq + Hash;
 }
 // Here we will also define that what constraints are required for these types
 // Now this is the trait, which needs all the different kind of types needed to make
 // the whole runtime work, my runtime has 2 structs, and this trait defines all the
 // types that are needed for them, this has list of the types and the traits they
 // must satisfy. 
 /*
 Now I want to use this trait, instead of using these different definitions. So, the ideal
 world, I dont pass these all the different type definitions, I just pass here T
 */

#[test]
fn test_step_5() {

    struct Runtime; // which will hold the implmentation of this trait

    impl BalanceVoting for Runtime // So now I am going to implement these types in my runtime struct
    {
        type AccountId = u32;
        type Balance = u32;
        type VoteIndex = u32;
    }

    let user_1: <Runtime as BalanceVoting>::AccountId = 1;
    let user_2: <Runtime as BalanceVoting>::AccountId = 2;

    let mut balances = step5::BalanceModule::<Runtime>::new(); // Here T is the trait that we want
    // I am going to write this code and make it work in the next step. Umm, so lemme actually
    // create a dummy struct that will impl this trait. So now instead of passing this T here, I am going
    // to pass this struct, which implments this trait which defines all these types, now you can see the error
    // so in order to make this work we will have to update our code in step 5. Lets make that work now
    let mut voting = step5::VotingModule::<Runtime>::new();

    balances.set_balance(user_1, 100);
    balances.set_balance(user_2, 200);

    voting.vote(user_1, 1, true);
    voting.vote(user_2, 2, true);

    assert!(balances.balance(user_1) == 100);
    assert!(voting.get_vote(user_1, 1) == true);
}