# Verifying Cairo proofs

Verifying a SHARP Cairo proof on the Starkex contracts is done in a number of transactions:

* 3 [Verify Merkle](https://etherscan.io/address/0x5899efea757e0dbd6d114b3375c23d7540f65fa4) transactions to verify Merkle paths ([example](https://etherscan.io/tx/0x5ad19d4524e0d2f2281dd71b8b1030fca7131ce74b821b936f73df2cba9d65e5))
* 8 [Verify FRI](https://etherscan.io/address/0x3e6118da317f7a433031f03bb71ab870d87dd2dd) transactions to verify the FRI proofs ([example](https://etherscan.io/tx/0xccc7446d9e5e14892496ee3956f0e9579c2f56b8e70441623aa283c302130201))
* A bunch of [Register Continuous Memory Page](https://etherscan.io/address/0xfd14567eaf9ba941cb8c8a94eec14831ca7fd1b4) transactions ([example](https://etherscan.io/tx/0x6862ef5e0ce7599124e7c81625130990a102f483dde292d76b8d869b7d280ea7))
* A single Verify Proof and Register ([example](https://etherscan.io/tx/0x720571bcac39e6b973537d7dd2ba253072e6f82c634ce361de73769d026ce4a1))

The reason the verification of a proof is split in multiple transactions is because proofs are too large, and the verification cost too much gas, to fit in a single transaction. 

> Note: Proofs can be split using [stark-evm-adapter](https://github.com/zksecurity/stark-evm-adapter/)

## Layouts

Different layouts are deployed on SHARP:

https://github.com/starkware-libs/starkex-contracts/tree/aecf37f2278b2df233edd13b686d0aa9462ada02/evm-verifier/solidity/contracts/cpu
