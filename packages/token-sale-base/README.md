# Token-Sale-Base Contract

Requirements:
1) MPC20 <-> MPC20 exchange. Deposit token A amount eq to X, receive token B amount eq to Y
2) Avaialble token allocation must be limited, but could be changed via onlyOwner access method
3) Would have been nice to support two token payout methods(both must immidiately transfer payout token to the sender after performing the exchange):
    3.1) Simple token transfer. Once the contract deployed it will assume that the owner also transferred the exact amount of tokens for payouts.
    When allocation is increased contract must use transfer_from function to increase its balance automaticaly.
    3.2) Contract will mint new tokens. For this case token-sale contract must be specified as a 'minter' inside MPC20 token. 
    Token-sale contract must have a method to set payout token address. This method should be only available in this mode.
4) The contract must allow to withdraw deposited tokens only to withdraw address. This address must be immutable(set via initialization). Only owner can trigger this function.
5) Whitelisting:
    5.1) We should have tiers. Each tier is an object that has info about: exchange price, sale limit, whitelisted addresses(must be done via merkle root), min purchase amount
    5.1.1) In case merkle_root for new tier is not specified then anyone can purchase tokens, otherwise users will need to provide merkle_proof to buy them.
    5.1.2) All this tier params must be modifiable: limit - ref point 2., exchange price, merkle_root - can be changed to add more addresses to the current tier, min purchase amount.
6) For the stats we should store the information about deposits and payouts. A two map(address => uint)'s that states how much each address deposited and received.
7) Unit tests should be implemented.


p.s.
for the ownable stuff use this lib - ../ownable-base
to work with merkle root stuff you can find methods here - ../utils/src/merkle (use utils::merkle::{validate_merkle_root, verify_merkle_proof})
