
| Title        |  Contributors  |
| :-------------|:--------------|
| Credits     | <Team: @giantconnect.com>          |


## Introduction

### Gifting - 

Any user who wants to gift an offer, will have to mint the dct corresponding to that offer. The minting of a dct in mainnet `v1.0` is possible using either `USDC` or `Credit card` payment. The payment confirmation by the buyer for the given DCT is done by getting their `TransactionHash/ TransactionID` validated by the protocol. 

On successful validation, the DCT is minted by the user, but a partial ownership is provided to the protocol gift account (behaves like a delivery service) which facilitates the gifting process. 
The actual ownership in order to get the rewards still belongs to the user who requested/ paid for the mint. This mint creates a unique URL to be shared with the consumer. The user can go ahead and share the gift URL with the consumer. At the activation of this gifted dct, the protocol gift account makes sure that it is transferred to the consumer and the corresponding rewards are provided to the original user.

Once the dct is activated, this mints an equivalent amount of data credits `(USD to credits (1:1))`. The minted credits are escrowed into protocol service fee `(5%)`, staking yield rewards `(5%)` and provider collateral `(90%)` accounts respectively. This is done in order to achieve total transparency, which will help the community gauge the amount of credits worth the value of USD for which they made the payment.
Once the DCT is expired, based on the rewards calculation the GIANT tokens are minted and provided to the provider, user (actual dct mint owner) and the staker. 


### gift

```
pub fn gift_external(
			origin: OriginFor<T>,
			offer_id: OfferId,
			recipient: AccountIdOf<T>,
			_memo: MintMemo,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(T::Admin::is_admin(&who), Error::<T>::NotPermitted);
			let dct_id: T::DctId = Self::mint_external_dct(offer_id, recipient.clone())?;
			Self::do_gift(recipient, dct_id)
		}
```

### gift_external flow

![gift_external](https://user-images.githubusercontent.com/11945179/187417834-61379447-5ba9-46f0-a7da-837657933fa6.jpg)




### Checks

There are several checks that should be done prior to minting of the dct and the credits:

1. `signed`: Checking that the identity trying to mint the DCT is the giant admin, and that the signing key used for minting contains a signing purpose and is not revoked
2. `active`: Checking if the offer is active and verified by the admin
