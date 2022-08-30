
| Title        |  Contributors  |
| :-------------|:--------------|
| Credits     | <Team: @giantconnect.com>          |


## Introduction

### Gifting - 

Any consumer who wants to purchase an offer, will have to mint the dct corresponding to that offer. The minting of a dct in mainnet `v1.0` is possible using either `USDC` or `Credit card` payment. The payment confirmation by the consumer for the given DCT is done by getting their `TransactionHash/ TransactionID` validated by the protocol. On successful validation, the DCT is minted and the consumer can go ahead and activate it.
Once the dct is minted, treasury mints an equivalent amount of data credits `(USD to credits (1:1))`. The minted credits are escrowed into protocol service fee `(5%)`, staking yield rewards `(5%)` and provider collateral `(90%)` accounts respectively. This is done in order to achieve total transparency, which will help the community gauge the amount of credits worth the value of USD for which they made the payment.
Once the DCT is expired, based on the rewards calculation the GIANT tokens are minted and provided to the provider, consumer and the staker. 


### gift

```
pub fn mint_external(
			origin: OriginFor<T>,
			offer_id: OfferId,
			recipient: AccountIdOf<T>,
			_memo: MintMemo,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(T::Admin::is_admin(&who), Error::<T>::NotPermitted);

			let treasury = T::GiantSudo::treasury_account();
			let treasury_balance = T::Credits::free_balance(&treasury);
			let dct_price_credits: CreditsOf<T> = dct_price_giant.saturated_into();
			// Ensure if treasury has enough credits for minting DCT else mint the remaining amount
			
			}
			ensure!(active, Error::<T>::OfferNotActive);
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let expiry_block_number =
				current_block_number + activation_period.into() + valid_from_activation.into();
			// retail_price in DCT will be represented in USD

			let dct_id = Self::mint_dct(treasury, recipient, dct_price_giant, dct_offer)?;
			let actual_expiry_block = Self::store_dct_expiry(expiry_block_number, dct_id)?;
		
			DCTMetadata::<T>::insert(dct_id, dct_metadata);
			Ok(())
		}
```

### gift_external flow

![gift_external](https://user-images.githubusercontent.com/11945179/187417834-61379447-5ba9-46f0-a7da-837657933fa6.jpg)




### Checks

There are several checks that should be done prior to minting of the dct and the credits:

1. `signed`: Checking that the identity trying to mint the DCT is the giant admin, and that the signing key used for minting contains a signing purpose and is not revoked
2. `active`: Checking if the offer is active and verified by the admin
