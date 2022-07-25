# MPC20-Staking Contract

# Actions

## stake
Stake specified amount of tokens to earn rewards.

Pararms: 
```json
StakeMsg {
    amount: 10,
}
```

## unstake
Withdraw staked tokens.

Pararms: 
```json
UnstakeMsg {
    amount: 11,
}
```

## claim
Claim earned rewards.

Pararms: 
```json
ClaimMsg {
    amount: 10 | null
}
```

## compound
Compound earned rewards(e.g. stake them).
Only works when deposit token is reward token.

Pararms: 
```json
CompoundMsg {
    amount: 10 | null
}
```

## [MPC20 Base actions](https://github.com/partisiablockchainapplications/CoreContracts/blob/master/contracts/mpc20/README.md)