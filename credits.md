
| Title        |  Contributors  |
| :-------------|:--------------|
| Staking     | <Team: @giantconnect.com>          |


## Introduction

Any GIANT token holder may stake their GIANT token with a given provider and receive sGIANT tokens in return. sGIANT tokens earn a pro-rata share of stakeholder yield from future DCT sales, and also may be slashed for customer refunds of future DCT sales. This behaves like a decentralized mechanism of maintaining quality providers and offers based on the stake (weightage) in the staking pool.
For Ex, if the provider you believe is matching the standards of the ecosystem, is generating quality offers, and generating higher DCT sales, you will be rewarded by showing faith in the provider based on the stake you have.

Only provider pool tokens, i.e. sGIANT, may be staked with nodes. In this way provider pools and node pools share liquidity, share slashing risk, and mutually strengthen the protocol.


## Stake

A GIANT token holder selects a particular pool and stakes the tokens in it.

```
pub fn stake(
			origin: OriginFor<T>,
			pool_id: PoolId,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = T::Provider::is_provider_active(pool_id)?;

			let who = ensure_signed(origin)?;

			// Check for the minimum amount to stake
			ensure!(amount >= T::MinStake::get(), Error::<T>::AmountIsLessThanMinStake);

			let available_balance = Self::available_balance(who.clone(), pool_id)?;
			ensure!(amount <= available_balance, Error::<T>::AmountIsLessThanAvailableBalance);

```

## Unstake

At any time a stakeholder can request to unstake. If the stakeholder has staked in a provider pool only, the unstake request will be submitted to the unstake request queue for that provider pool and the tokens will be subtracted from the provider pool value when calculating the provider’s delayed revenue amount during the DCT minting process.
At this point the stakeholder will not earn any yield from future DCT sales. Only once the value of tokens in the provider pool staked before the date when the stake being requested to be withdrawn was staked is greater than the value of outstanding DCTs minted before the date when the stake requested to be withdrawn and all other unstake requests have been serviced will the tokens in question be unstaked. This will never be longer than the validity period of the longest DCT sold before the unstake request.


```
pub fn unstake(
			origin: OriginFor<T>,
			pool_id: PoolId,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = T::Provider::is_provider_active(pool_id)?;

			let who = ensure_signed(origin)?;

			ensure!(amount > T::MinUnstake::get(), Error::<T>::AmountIsLessThanMinUnStake);

			ensure!(
				StakingPool::<T>::contains_key(pool_id, who.clone()),
				Error::<T>::StakingAccountNotFound
			);

```


### Checks

There are several checks that should be done prior to the staking and unstaking of the tokens:

1. `signed`: Checking that the identity trying to stake the tokens is a valid GIANT identity, and that the signing key used for staking contains a signing purpose and is not revoked
2. `providerPool`: Checking if the selected pool is in active state or not
3. `minStake`: Checking if the signer's declared amount is greater than or equal to the minimum stake amount
4. `availableBalance`: Checking if the signer has enogh balance to stake
5. `active`: Checking if the provider is active and verified by the admin along with the pool status set to active
