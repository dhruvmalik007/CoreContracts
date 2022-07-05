# MPC721 Contract

# Actions

## set_base_uri
Set base uri for the tokens.

Params:
```json
SetBaseUriMsg {
    "new_base_uri": "<uri>",
}
```

## mint
Mint a new token. Can only be executed by minter account.

Params:
```json
MintMsg {
    "token_id": 1,
    "to": "<address>",
    "token_uri": "<optional uri>",
}
```

## transfer
Transfer token to another account.

Params:
```json
TransferMsg {
    "to": "<address>",
    "token_id": 1,
}
```

## transfer_from
Only with approval extension. Transfer token from owner to spender.

Params:
```json
TransferFromMsg {
    "from": "<address>",
    "to": "<address>",
    "token_id": 1,
}
```

## approve
Allows spender to transfer token from the owner account.

Params:
```json
ApproveMsg {
    "spedner": "<address>",
    "token_id": 1,
}
```

## approve_for_all
Allows operator to transfer any owner tokens from his account.

Params:
```json
ApproveForAllMsg {
    "operator": "<address>",
}
```

## revoke
Remove approval.

Params:
```json
RevokeMsg {
    "spedner": "<address>",
    "token_id": 1,
}
```

## revoke_for_all
Remove operator.

Params:
```json
RevokeForAllMsg {
    "operator": "<address>",
}
```

## burn
Destroy your token forever.

Params:
```json
BurnMsg {
    "token_id": 1,
}
```
