# Proof Splitter

Due to the gas limitations when verifying an original STARK proof, the proof generated by the Stone Prover should be split into three types: the main proof, Trace Merkle proof, and FRI proof. Typically, this includes one Main Proof accompanied by several Trace Merkle Proofs and FRI Proofs.

This process relies on the [fact registry](./facts.md), which functions as verification snapshots. These snapshots can be viewed as pre-verified subproofs. This allows the Main Proof to be verified based on earlier snapshots obtained from the Merkle Proofs and FRI Proofs, thereby circumventing the bottleneck caused by gas limitations in a single EVM transaction.

This section aims to clarify the roles of the various proof types, showing how the split proof works with references to the source code in [Cairo verifier contracts](https://github.com/starkware-libs/starkex-contracts), [Stone Prover](https://github.com/starkware-libs/stone-prover) and  [Stark EVM Adapter](https://github.com/zksecurity/stark-evm-adapter).

## Main proof

The Main proof is the primary proof that corresponds to the original proof generated by the Stone Prover. It encompasses essential proof data, including commitments for traces and FRI layers, along with trace values necessary for deriving the deep composition polynomial $p_0$, among other elements.

At a high level, it performs the following two key functions:
1. Deriving a deep composition polynomial $p_0$
2. Checking FRI layers for $p_0$

> This explanation reuses the notations from the [ethSTARK paper](https://eprint.iacr.org/2021/582.pdf).

The Main Proof contains the commitments [for](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/StarkVerifier.sol#L515) [the](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/StarkVerifier.sol#L526) [traces](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/StarkVerifier.sol#L536) and [FRI](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/StarkVerifier.sol#L555), as well as the trace values for deriving the polynomial $p_0$.

These trace values, also referred to as mask values, can be categorized into two types:

1. Trace values involved in expressing $C_j(x)$ in the following composition polynomial:


$$h(x) = \sum_{j=1}^{k} C_j(x) (\alpha_j x^{D-D_j^{-1}} + \beta_j)$$



2. Evaluations of the polynomials $h_i(x)$ decomposed from the $h(x)$: 


$$h(x) = \sum_{i=0}^{M_2-1} x^i h_i(x^{M_2})
$$

By invoking the [CpuConstraintPoly](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/cpu/CpuVerifier.sol#L353) contract to evaluate the $h(x)$ with the trace mask values, it checks the out-of-domain-sampling (oods) consistency among these provided trace values and the evaluations of decomposed composition polynomial.

In other word, to do the check at a random point $z$, they check:

$$\sum_{j=1}^{k} C_j(z) (\alpha_j z^{D-D_j^{-1}} + \beta_j) == \sum_{i=0}^{M_2-1} z^i h_i(z^{M_2})$$


After passed the OODS consistency check, it proceeds to prepare and verifie the FRI layers for $p_0​$.

By aggregating the trace values $y_\ell$ and evaluations $\hat{y}_i$ obtained from the OODS, it derives the deep composition polynomial $p_0$ (or quotient polynomial) through the [CpuOods](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/StarkVerifier.sol#L431) [contract](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/cpu/layout5/CpuOods.sol#L36).

$$p_0(x) = \sum_{\ell=0}^{M_1-1} \frac{\gamma_\ell \cdot (f_{\ell}(x) - y_\ell)}{x - z g^{s\ell}} + \sum_{i=0}^{M_2-1} \frac{\gamma_{M_1+i} \cdot (h_i(x) - \hat{y}_i)}{x - z^{M_2}}
$$

FRI plays the role as a PCS. Doing this means that the prover has to prove that quotient polynomial $p_0$ exists for OODS evaluations and is of degree $d(h) - 1$. During FRI the verifier will obtain different evaluations of $p_0$. For example if they want an evaluation of $p_0(3)$ the prover would have to produce merkle proofs for $f_\ell(3)$ for all $\ell$ and $h_i(3)$ for all $i$.

In the first FRI layer computation stage, the contract reads these induced values based on FRI queries and decommits them from the main proof. Three traces are required for the Cairo verifier: [execution trace](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/StarkVerifier.sol#L395), [interaction trace](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/StarkVerifier.sol#L405) and [composition trace](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/StarkVerifier.sol#L415). Each decommitment checks against a [trace commitment](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/StarkVerifier.sol#L400), which should have been verified and registered as [a fact](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/MerkleStatementVerifier.sol#L47) in the Merkle fact registry. 

These facts serve as verification [snapshots](./facts.html), which are crucial when it is needed to split a verification into multiple stages. This approach is necessary due to various reasons, such as circumventing gas cost limitations or conserving gas costs for transaction retries based on a verified snapshot.

Similarly, the process of checking the FRI layers depends on the FRI Merkle verification snapshots obtained earlier. It involves verifying whether the [fact is registered](https://github.com/starkware-libs/starkex-contracts/blob/f4ed79bb04b56d587618c24312e87d81e4efc56b/evm-verifier/solidity/contracts/Fri.sol#L93) with the FRI layer's commitment.

## Other split proofs
In order to provide the verfication snapshots for the Main Proof, it is essential to first complete both the Trace Merkle Proofs and the FRI Merkle Proofs. These preliminary steps are necessary to generate the verification snapshots required for the Main Proof.

The [MerkleStatementContract](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/MerkleStatementContract.sol#L15) and [FriStatementContract](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/FriStatementContract.sol#L23) are specifically designed for verifying the Trace Merkle Proofs and FRI Merkle Proofs, respectively. Upon successful verification, [these](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/FriStatementContract.sol#L85) [facts](https://github.com/starkware-libs/starkex-contracts/blob/f81ba5fdbd68516db50ea9679de9d0ac2f8049d8/evm-verifier/solidity/contracts/MerkleStatementContract.sol#L104) will be registered as verification snapshots.

## Proof annotations
With the original proof file obtained from the Stone Prover, it is necessary to run its CPU verifier to [generate the annotations](https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/main/verifier_main_helper.cc#L36-L45). This process is akin to operating an offline simulator, which verifies the proof and simulates the required proof data as annotations. These annotations are then utilized by verifiers in other locations, such as the EVM Cairo verifier.

The primary reason for generating these annotations, rather than extracting directly from the original proof, would be to facilitate the restructuring of the data in the original proof into a clearer format. This restructuring is particularly beneficial for other verification procedures, such as those employed by EVM Cairo verifiers.

[Here](https://github.com/zksecurity/stark-evm-adapter/blob/f4f2c88bd30c157423f67564d9ea3481b70c0a3c/tests/fixtures/annotated_proof.json#L2) is an example of an annotated proof. This example showcases how annotations are combined with the original proof into a single JSON file for further processing. The annotations themselves represent various data points, here are some of the examples:

```js
// execution trace commitment
"P->V[0:32]: /cpu air/STARK/Original/Commit on Trace: Commitment: Hash(0x3c8537043a0e5298ac50fd0c85a697b4f64ad84d000000000000000000000000)"
```

```js
// interaction trace commitment
"P->V[32:64]: /cpu air/STARK/Interaction/Commit on Trace: Commitment: Hash(0xf5a5807d04f92b370a2ca27ccafaf40f196a27ab000000000000000000000000)"
```

```js
// composition trace commitment
"P->V[64:96]: /cpu air/STARK/Out Of Domain Sampling/Commit on Trace: Commitment: Hash(0x2a0d752d3cf399e94ebc1cc8a425ce89d848b7d7000000000000000000000000)"
```

```js
// verifier challenges for interaction trace 
"V->P: /cpu air/STARK/Interaction: Interaction element #0: Field Element(0x29767aebd00e6750d36470c07b003624f52e08794f24c23c17fdbcf66e1593f)",
"V->P: /cpu air/STARK/Interaction: Interaction element #1: Field Element(0x5e8b64f90ebe7e15559196630cd2bb6bb95b0d9121ff82adf708a2e6637b142)..."
```

```js
// oods evaluation point
"V->P: /cpu air/STARK/Out Of Domain Sampling/OODS values: Evaluation point: Field Element(0x4f03c43ef1a1476cea9b31c4880f1808ed0dada6e2d650af829115186740856)"
```

```js
// commitments for FRI layers
"P->V[8768:8800]: /cpu air/STARK/FRI/Commitment/Layer 1: Commitment: Hash(0x994586a93d3f0397b588be7eb5ea55ecaec10145000000000000000000000000)",
"V->P: /cpu air/STARK/FRI/Commitment/Layer 2: Evaluation point: Field Element(0x69d6218dda1b690e11974c0d7bce4b5fb6584f5b188d2d8988436c9d7423fac)",
"P->V[8800:8832]: /cpu air/STARK/FRI/Commitment/Layer 2: Commitment: Hash(0x533a5b9d2896a3a50338eba17f3c660f6fc89995000000000000000000000000..."
```


## Open source library
The [Stark EVM Adapter](https://github.com/zksecurity/stark-evm-adapter) provides an API to facilitate the splitting of original proofs from the Stone Prover into these segmented proofs. This library is particularly useful for developers working with STARK proofs in EVM environments. Included in the repository is a demo script that illustrates the complete process of [splitting a Stone proof](https://github.com/zksecurity/stark-evm-adapter/blob/20cb1a83ddcbbd092f8aa6cf3382383b1c0e9814/examples/verify_stone_proof.rs#L68). This script demonstrates not only how to split the proof but also how to [submit the resultant proofs to the Cairo verifier contracts](https://github.com/zksecurity/stark-evm-adapter/blob/20cb1a83ddcbbd092f8aa6cf3382383b1c0e9814/examples/verify_stone_proof.rs#L71-L96).