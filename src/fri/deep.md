# Domain Extension for Eliminating Pretenders (DEEP)

* sounds like an upgrade to FRI in order to reduce the number of queries while keeping the same soundness
* but risc0 says it's not:

> Shortly after the FRI protocol was released, an alternative protocol called DEEP-FRI was released. Although DEEP-FRI was initially thought to have improved soundness relative to FRI, the Proximity Gaps for Reed-Solomon Codes paper shows that the original FRI protocol offers the same soundness results as DEEP-FRI at less computational complexity. The RISC Zero ZKP system uses the original FRI protocol.

from: https://dev.risczero.com/reference-docs/about-fri

* yet starkware seems to use it with their DEEP as well as OODS (out-of-domain sampling) stuff
* DEEP-FRI is described in section 5.2 of the DEEP-FRI paper
* DEEP-ALI is described later in that same paper. It's a STIK (which you can isntantiate as a STARK like ethSTARK) that makes use of DEEP-FRI
