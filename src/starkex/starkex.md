# Starkex

The starkex contracts are deployed at [0x6cB3EE90C50a38A0e4662bB7e7E6e40B91361BF6](https://etherscan.io/address/0x6cB3EE90C50a38A0e4662bB7e7E6e40B91361BF6#code) and sit behind the upgradable smart contract at [0x47312450b3ac8b5b8e247a6bb6d523e7605bdb60](https://etherscan.io/address/0x47312450b3ac8b5b8e247a6bb6d523e7605bdb60#readProxyContract).

Somewhat outdated implementations live on Github at [starkware-libs/starkex-contracts](https://github.com/starkware-libs/starkex-contracts).

They implement the [SHARP (SHARed Prover)](https://starkware.co/resource/joining-forces-sharp/) verifier, which allows us to verify SHARP proofs on Ethereum.

> Note: A number of contracts refer to "GPS" which is the old name for SHARP (general-proving service).

Any application on Ethereum that wants to use Cairo can make use of this service to verify proofs.
The flow is split in two parts:

1. The proof is sent to the SHARP service, which verifies it and returns a fact.
2. The application can check that the fact has been verified.

We explain what "facts" are in [facts section](/facts.md).
