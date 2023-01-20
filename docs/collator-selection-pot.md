# Collator Selection Reward Pot

Watr is configured to use collator selection. This is configured to store all fees in a "staking pot".

Every block, a portion of this pot is split between the collators. 

To activate the Pot account, ensure an existential deposit is sent to the following address:
`2wS2DpdLUh45fLicxABP8biegPz4vc6BnVA3RH9CWAi1bmky`

Once this account has an ED, the collators will start receiving a portion of the fees.  

## Sending ED 
It is recommended to send the Pot account an ED before any other transactions are made. This ensures the total issuance does not drop via burned fees.

1. Navigate to the Sudo page
2. Select `balances.transfer`
3. Paste the Pot account address `2wS2DpdLUh45fLicxABP8biegPz4vc6BnVA3RH9CWAi1bmky`
4. And choose a value great than the ED. For example, 1 Watr is more than sufficient.

Note: it is also possible to do this from a non-sudo account. But it is recommended to use Sudo as an assurance. Sudo does not pay fees, meaning no fees will be accidentally burned. 