# Starkex

The starkex contract live in https://github.com/starkware-libs/starkex-contracts
They implement the [SHARP (SHARed Prover)](https://starkware.co/resource/joining-forces-sharp/) verifier, which allows us to verify SHARP proofs on Ethereum.

> Note: A number of contracts refer to "GPS" which is the old name for SHARP (general-proving service).

Any application on Ethereum that wants to use Cairo can make use of this service to verify proofs.
The flow is split in two parts:

1. The proof is sent to the SHARP service, which verifies it and returns a fact.
2. The application can check that the fact has been verified.

We explain what "facts" are in [facts section](/facts.md).
