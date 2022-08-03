| Title        |  Contributors  |
| :-------------|:--------------|
| Calculations     | <Team: @giantconnect.com>          |


## Introduction

### Rewards
#### Demand & Supply rewards

Consumers and providers are given tokens allocated from the initial distribution of the GIANT tokens. These tokens serve to encourage growth and ensure that consumers and providers both become stakeholders in the broader economy. 

These tokens are released based on the value of data consumers use, per GIANT token of data consumed, and decrease geometrically as the protocol becomes larger. Specifically, supply and demand rewards will be paid 0.3 and 0.4 GIANT tokens per GIANT token of data consumed for the first 2.5 million tokens worth of data consumed. After that the rewards will decrease by a factor of 2.5, then again for every five fold increase in cumulative consumption. 

![rewardEquation](https://user-images.githubusercontent.com/11945179/182537982-07fa0ea7-7a40-47e4-bf46-42bf243efc52.jpg)


## GIANT's idea

### Auto-stake and Unstake

These tokens are automatically staked with the provider pool as they are earned. 
All 100% of earned rewards for the providers aka supply rewards are staked in the pool unless requested for unstake. 
For consumers we maintain `amount_stake = 70%` and `amount_transfer = 30%` of demand rewards as auto-stake and wallet transfer respectively.
At the time of unstaking the user can request for unstake and it will stop generating yield for those tokens but the credit of tokens will only be active once the DCT is expired.

### Step-by-step overview
#### Unstake GIANT


1. **TVO**
TVO is the Total Value Outstanding. This is equal to the sum total of all the active DCTs
   

2. **SPP**
   

3. **Provider debt**


4. **Escrow Values**



```

eq.1

```

```

eq.2

```
```

eq.3

```




### Operational costs

Any fees as gas fees that is currently been incured by the protocol. We can define that structure here. And the overall fee structure for the functionalities too.

## Sample calculations

### Simple example for one DCT minted, tokens staked and rewards generated

**Financial Parameters:**
1. DCT retail price = 10 GIANT,
2.  


**Valuation assumptions**
1. supply rewards (rs) = 30.00%,
2. demand rewards (rd) = 40.00%,
3. yield max = 0.5%


**Calculations**
