# Starkex

The starkex contracts are:

* deployed at [0x6cB3EE90C50a38A0e4662bB7e7E6e40B91361BF6](https://etherscan.io/address/0x6cB3EE90C50a38A0e4662bB7e7E6e40B91361BF6#code) 
* and sit behind the upgradable smart contract at [0x47312450b3ac8b5b8e247a6bb6d523e7605bdb60](https://etherscan.io/address/0x47312450b3ac8b5b8e247a6bb6d523e7605bdb60#readProxyContract).

By looking at the [Constructor Arguments section of the implementation contract](https://etherscan.io/address/0x6cB3EE90C50a38A0e4662bB7e7E6e40B91361BF6#code) (which might be outdated at the moment of this writing, as the implementation contract is upgradable) one can see where the subcontracts are deployed:

* `bootloaderProgramContract`: [0x5d07afFAfc8721Ef3dEe4D11A2D1484CBf6A9dDf](https://etherscan.io/address/0x5d07afFAfc8721Ef3dEe4D11A2D1484CBf6A9dDf)
* `memoryPageFactRegistry_`: [0xFD14567eaf9ba941cB8c8a94eEC14831ca7fD1b4](https://etherscan.io/address/0xFD14567eaf9ba941cB8c8a94eEC14831ca7fD1b4)
* `cairoVerifierContracts`]: [0x217750c27bE9147f9e358D9FF26a8224F8aCC214](https://etherscan.io/address/0x217750c27bE9147f9e358D9FF26a8224F8aCC214),[0x630A97901Ac29590DF83f4A64B8D490D54caf239](https://etherscan.io/address/0x630A97901Ac29590DF83f4A64B8D490D54caf239),[0x8488e8f4e26eBa40faE229AB653d98E341cbE57B](https://etherscan.io/address/0x8488e8f4e26eBa40faE229AB653d98E341cbE57B),[0x9E614a417f8309575fC11b175A51599661f2Bd21](https://etherscan.io/address/0x9E614a417f8309575fC11b175A51599661f2Bd21),[0xC879aF7D5eD80e4676C203FD300E640C297F31e3](https://etherscan.io/address/0xC879aF7D5eD80e4676C203FD300E640C297F31e3),[0x78Af2BFB12Db15d35f7dE8DD77f29C299C78c590](https://etherscan.io/address/0x78Af2BFB12Db15d35f7dE8DD77f29C299C78c590),[0xe9664D230490d5A515ef7Ef30033d8075a8D0E24](https://etherscan.io/address/0xe9664D230490d5A515ef7Ef30033d8075a8D0E24),[0x03Fa911dfCa026D9C8Edb508851b390accF912e8](https://etherscan.io/address/0x03Fa911dfCa026D9C8Edb508851b390accF912e8)
* `hashedSupportedCairoVerifiers`: `3178097804922730583543126053422762895998573737925004508949311089390705597156`
* `simpleBootloaderProgramHash`: `2962621603719000361370283216422448934312521782617806945663080079725495842070`

> Note: The different verifier contracts correspond to the different layouts that are supported for Cairo (see the [Builtins and Layouts of Cairo](../cairo/builtins.md) section of the book).

Somewhat outdated implementations live on Github at [starkware-libs/starkex-contracts](https://github.com/starkware-libs/starkex-contracts).

They implement the [SHARP (SHARed Prover)](https://starkware.co/resource/joining-forces-sharp/) verifier, which allows us to verify SHARP proofs on Ethereum.

> Note: A number of contracts refer to "GPS" which is the old name for SHARP (general-proving service).

Any application on Ethereum that wants to use Cairo can make use of this service to verify proofs.
The flow is split in two parts:

1. The proof is sent to the SHARP service, which verifies it and returns a fact.
2. The application can check that the fact has been verified.

We explain what "facts" are in [facts section](/facts.md).
