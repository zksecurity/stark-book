# what is this
This is an ongoing project to implement a FRI prover and verifier in rust. 
The code aims to serve as a walkthrough of the FRI protocol.

# run test
`cargo test `

# plans
## Naive FRI
It is to create layers with folded polynomial and then sample the result. No commitment involved.

This is to demonstrate the natural structure of FRI, drawing the connection between the math and the idea of low degree testing.

From PCP(probabilistic checkable proof) perspective, the naive FRI can save verifying time by sampling resulted layers.

## FRI with commitment
*todo*

## Soundness calculation
*todo*