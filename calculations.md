| Title        |  Contributors  |
| :-------------|:--------------|
| Calculations     | <Team: @giantconnect.com>          |


## Introduction

### Rewards
#### Demand & Supply rewards

Consumers and providers are given tokens allocated from the initial distribution of the GIANT tokens. These tokens serve to encourage growth and ensure that consumers and providers both become stakeholders in the broader economy. 

These tokens are released based on the value of data consumers use, per GIANT token of data consumed, and decrease geometrically as the protocol becomes larger. Specifically, supply and demand rewards will be paid 0.3 and 0.4 GIANT tokens per GIANT token of data consumed for the first 2.5 million tokens worth of data consumed. After that the rewards will decrease by a factor of 2.5, then again for every five fold increase in cumulative consumption. 


<p align="center">
  <img 
    src="https://user-images.githubusercontent.com/11945179/182537982-07fa0ea7-7a40-47e4-bf46-42bf243efc52.jpg"
  >
</p>

Table mentioned below gives examples for the reward rates at the first six levels.

| Threshold (M)        |  Supply  | Threshold (M)  | Reward  |
| :-------------|:--------------| :-------------|:--------------|
| 2.50        |  30.00%  | 2.5  | 40.00%  |
| 12.50        |  12.00%  | 12.50  | 16.00%  |
| 62.50        |  4.80%  | 62.50  | 6.40%  |
| 312.50        |  1.92%  | 312.50  | 2.56%  |
| 1562.50        |  0.77%  | 1562.50  | 1.02%  |


## GIANT's idea

### Auto-stake and Unstake

These tokens are automatically staked with the provider pool as they are earned. 
All 100% of earned rewards for the providers aka supply rewards are staked in the pool unless requested for unstake. 
For consumers we maintain `amount_stake = 70%` and `amount_transfer = 30%` of demand rewards as auto-stake and wallet transfer respectively.
At the time of unstaking the user can request for unstake and it will stop generating yield for those tokens but the credit of tokens will only be active once the DCT is expired.

### Step-by-step overview
#### Important Terms


1. **TVO**
TVO is the Total Value Outstanding. This is equal to the sum total of all the active DCTs (in USD) subtracting the total number of rewards claimed and distributed along with no refunds.  
   

2. **SPP**
Stake in Provider Pool. This is equal to number of GIANT tokens currently available in the provider pool multiplied with the current GIANT_to_USD value in order to get the USD value of the pool. This total value is added with the USD value of any collateral held for the provider. 

```
SPP = SPP_GIANT * USDGIANT + ProviderCollateral 
```
 
3. **Escrow Values**
There are various escrow accounts where we hold the DCT yield tokens generated for a given pool along with the provider's collateral and the total stake in the pool for distribution of rewards and refunds.

The immediate price of DCT is known as P_DCT (offer price). So, for a given DCT value the provider is liable for rewards and revenue on the `90%` of the DCT value. 
`10%` of the DCT value is further divided into `5%` of the yield (rho_DCT) and `5%` of the protocol fee (nu_DCT).

Based on the above mentioned values, the immediate revenue that is available for the provider is calculated as per the equation below

<p align="center">
  <img 
    src="https://user-images.githubusercontent.com/11945179/182594057-d7c80374-f42b-42a9-bb73-d63453789f4b.png"
  >
</p>


where

<p align="center">
  <img 
    src="https://user-images.githubusercontent.com/11945179/182595960-0a8872b4-c2b9-4317-9455-06b0c78effb7.png"
  >
</p>


To calculate how to use the available collateral and pay any debt which a staker owes to the pool is mentioned below

<p align="center">
  <img 
    src="https://user-images.githubusercontent.com/11945179/182598058-c855fd6e-7eba-4098-94d3-ce2e5bf0f580.png"
  >
</p>

Based on all of the above mentioned calculations the total value outstanding (TVO) is updated when a new DCT is minted.

TVO is a novel approach that not only helps us calcuate the various factors for minting of a DCT but also helps in various calculations while unstaking GIANT tokens from a given pool.

### Fee Structure

Any fees as transactional fees is waived off from the user and incured by the protocol. We have defined the transaction and service fees for various operations in the table below

| Operations  |  Transaction + Service fees in GIANT  |
| :-------------|:--------------|
| mint   | 0 + 0  |
| activate   | 0 + 0  |
| expire   | 0 + 0  |
| submit SED   | 0 + 0  |
| request refund   | 0 + 1  |
| registerProviderIntent   | 0 + 10  |
| staking in any pool   | 0 + 0  |
| unstaking from any pool   | 0 + 1  |
| create offer   | 0 + 1  |
| registering Node intent   | 0 + 10  |
| nominating Nodes   | 0 + 0  |
| transferring DCT   | 0 + 1  |

Please also find the underlying calculations as well as detail on various other calculations [here](https://docs.google.com/document/d/1vn4cfXJ1CQNWbtpPhzaDymCYP3lB62htHINqcG2C_mg/edit?usp=sharing).
