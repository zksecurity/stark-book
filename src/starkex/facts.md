# Fact Registry

The [Starkex contracts](https://github.com/starkware-libs/starkex-contracts/) implement the verifier side of the [SHARP service](https://starkware.co/resource/joining-forces-sharp/). 
In other words, when people create proofs using the [SHARP service](https://starkware.co/resource/joining-forces-sharp/), they can get them verified on Ethereum using the [Starkex contracts](https://github.com/starkware-libs/starkex-contracts/).

First, let's introduce what a fact is: a fact can be any computation that was computed by some logic in the smart contract. For example, a fact can be: "_we successfully verified a proof for this Cairo program and these public inputs_".
This example is actually the main fact that Starkex will register for the different applications making use of it.
But internally, other facts are used whenever a computation (like verifying a proof) is split in different transactions that need to produce a snapshot of what has been done so far and resume from other snapshots. This is explained in more details in the [Verifying Cairo proofs](./cairo.md) section.

A fact is represented (or authenticated) by a hash of its variables. As such, it is important that different applications or different contexts use "different" hash functions not to have collisions. This can be done by adding some domain separation string to the hash function, or as it is done by starkex by using different fact registries for different usecases.

## The Fact Registry

Let's introduce the smart contract in charge of these facts, [FactRegistry.sol](https://github.com/starkware-libs/starkex-contracts/blob/aecf37f2278b2df233edd13b686d0aa9462ada02/scalable-dex/contracts/src/components/FactRegistry.sol):

```js
contract FactRegistry is IQueryableFactRegistry {
    // Mapping: fact hash -> true.
    mapping(bytes32 => bool) private verifiedFact
```

As you can see, facts are just tracked via a hashmap. Registering a fact is such straightfoward:

```js
    function registerFact(bytes32 factHash) internal {
        // This function stores the fact hash in the mapping.
        verifiedFact[factHash] = true;
```

As well as checking if a fact has been registered:

```js
    function isValid(bytes32 fact) external view override returns (bool) {
        return _factCheck(fact);
    }
```

## Example of facts from external applications

An example of registering a fact can be seen, for example at the end of a proof verification. In [GpsStatementVerifier.sol:verifyProofAndRegister()](https://github.com/starkware-libs/starkex-contracts/blob/aecf37f2278b2df233edd13b686d0aa9462ada02/evm-verifier/solidity/contracts/gps/GpsStatementVerifier.sol#L71):

```js
    function verifyProofAndRegister(
        uint256[] calldata proofParams,
        uint256[] calldata proof,
        uint256[] calldata taskMetadata,
        uint256[] calldata cairoAuxInput,
        uint256 cairoVerifierId
    ) external {
        // TRUNCATED...

        registerGpsFacts(taskMetadata, publicMemoryPages, cairoAuxInput[OFFSET_OUTPUT_BEGIN_ADDR]);
    }
```

where `registerGpsFacts` is defined as:

```js
    function registerGpsFacts(
        uint256[] calldata taskMetadata,
        uint256[] memory publicMemoryPages,
        uint256 outputStartAddress
    ) internal {
        // TRUNCATED...

        // Register the fact for each task.
        for (task = 0; task < nTasks; task++) {
            // TRUNCATED...

            bytes32 fact = keccak256(abi.encode(programHash, programOutputFact));

            // TRUNCATED...
            registerFact(fact);

            // TRUNCATED...
        }

        // TRUNCATED...
    }
```

[Starknet](https://book.starknet.io/) is the main application making use of SHARP, and as such their smart contract uses the fact registry directly.

The main function of Starknet is [`updateState()`](https://github.com/mimoo/starknet-contracts/blob/main/contracts/Starknet.sol#L176), which updates the state based on proofs that have been verified:

```js
    function updateState(
        int256 sequenceNumber,
        uint256[] calldata programOutput,
        uint256 onchainDataHash,
        uint256 onchainDataSize
    ) external onlyOperator {
        // TRUNCATED...

        bytes32 sharpFact = keccak256(
            abi.encode(programHash(), stateTransitionFact)
        );
        require(
            IFactRegistry(verifier()).isValid(sharpFact),
            "NO_STATE_TRANSITION_PROOF"
        );

        // TRUNCATED...
```

## Example of checking if a fact internally

Another example we can look at is within a proof verification. As explained in [Verifying a Cairo proof](./cairo.md), a proof verification is split in multiple transactions.

For example, Merkle membership proofs are verified in segregated transactions, and then the fact that they were verified is used within another execution. The fact is first verified and then registered in `MerkleStatementContract:verifyMerkle()`:

```js
    function verifyMerkle(
        uint256[] memory merkleView,
        uint256[] memory initialMerkleQueue,
        uint256 height,
        uint256 expectedRoot
    ) public {
        // TRUNCATED...

        bytes32 resRoot = verifyMerkle(channelPtr, merkleQueuePtr, bytes32(expectedRoot), nQueries);
        bytes32 factHash;
        assembly {
            // Append the resulted root (should be the return value of verify) to dataToHashPtr.
            mstore(dataToHashPtr, resRoot)
            // Reset dataToHashPtr.
            dataToHashPtr := add(channelPtr, 0x20)
            factHash := keccak256(dataToHashPtr, add(mul(nQueries, 0x40), 0x20))
        }

        registerFact(factHash);
    }
```

and in `MerkleStatementVerifier:verifyMerkle()`:

```js
    function verifyMerkle(
        uint256, /*channelPtr*/
        uint256 queuePtr,
        bytes32 root,
        uint256 n
    ) internal view virtual override returns (bytes32) {
        // TRUNCATED...

        require(merkleStatementContract.isValid(statement), "INVALIDATED_MERKLE_STATEMENT");
        return root;
    }
```

where both use their own fact registry not to collide with other usecase (as pointed out at the beginning of this section).
