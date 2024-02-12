# Secret Recovery Protocol Using ICP .
So if you haven’t notice , there isn’t a forget password button in web3 yet, and if there is it’s usually centralised
and trusted provider giving that service . But what if that wasn’t the case ? What if we could have a
decentralised trustless wallet recovery system powered by smart contracts ? Well this is what i propose here 

## Current Solutions 
In 2021 , Near wallet did implement this , they used a centralised OTP email server to do that, plus the private
key was stored by them encrypted in their server. Soon enough a social engineering attack , comprosed the
email server and thousands of customer wallets were compromised . Ton wallet currently is doing the same
thing , and hence are not a non custodian wallet option. Then there are MPC wallets , MPC wallets could be
seen as partially custodial and non interoperable with normal wallet standards . Hence MPC wallets do require
a central party to be there

## How it works ?
We use DKIM signature verification to verify emails , and vet keys to store end to end encrypted seed phrases
. The Encrypted seed phrase can only be decrypted if the user sends a valid DKIM signature of this email .
Hence we can now have wallet recovery mechanism
## Flow Of Protocol
### User Registersing 
1. User clicks on register which shows 5 digit number that is going to expire in 5 minutes
2. Then an instruction is set out such that the user has to give a copy of the email which has those 5 digits
3. Then the user sends an email to the email he want to use as recovery.
4. Then submits the copy of the email from his recovery email
5. Then user enters the secret phrase
### User Retrieving Secret
1. User enters email
2. After checking if the email is registered , then a 5 digit number is show
3. The user sends the copy of a email which has those 5 digit number
4. The smart contract verifies the email using DKIM and return the phrase

## DEMO !
Click to view video
[![Demo Quick Video ](https://cdn.loom.com/sessions/thumbnails/de6b021b71e04e5ea4bc082b7ae832a5-with-play.gif)](https://www.loom.com/share/de6b021b71e04e5ea4bc082b7ae832a5)

## Link
https://poetic-figolla-c04a19.netlify.app/



