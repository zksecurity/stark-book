# Bootloader

The bootloader is a Cairo program that can prove the execution of multiple Cairo programs by simply running the programs inside the bootloader. Since the Cairo program to be proved can be any arbitrary program, it can also be a Cairo verifier program, which means that the bootloader also supports creating a proof that proves the validity of a bootloader proof. Building on this concept, the bootloader allows creating a proof of nested Cairo programs, or a tree-like structure made up of Cairo programs and bootloader proofs.

If this sounds too abstract, don't worry--this doc will attempt to explain in detail how the bootloader works and the two use cases mentioned above. (Note: this is based on the [Cairo v0.13.1 version of the bootloader](https://github.com/starkware-libs/cairo-lang/tree/v0.13.1/src/starkware/cairo/bootloaders))

## A short primer on how CairoVM works

Before diving into the details about the bootloader, let’s go over how CairoVM works. CairoVM is a virtual machine that can execute a Cairo program (technically, a Cairo program compiled to a Cairo-specific set of instructions) and in the end produce a memory vector and execution trace table that is populated throughout the execution of the program.

More specifically, the generated memory can be thought of as a vector that is divided into segments that have different functionalities (e.g. program instructions, execution, output). The execution trace is a table with 3 columns each representing a register value and rows that each represent the value of the registers over the execution of the program. This means that the number of rows is equal to the number of steps that the CairoVM runs, given that 1 step equals 1 instruction call.

<div style="text-align: center;">
    <img src="cairo_vm.png" alt="CairoVM execution structure" width="80%">
</div>

Note: the execution segment can be thought of as values that are written to the stack while running a program. Since the memory values can only be written once in CairoVM, the stack values are all written to this segment.

Using these results of the CairoVM execution, the Stone prover can create a proof that proves the following (very roughly speaking):

- there exists a valid execution trace of n steps on the CairoVM
  - which starts with a given set of initial register values
  - which ends with a given set of final register values
  - for some memory which satisfies certain constraints (e.g. read-only, continuous, consistent with the public memory portion)

### Builtins

Builtins are pre-allocated memory cells that can be used for certain commonly-used “builtin” functions. As of Cairo v0.13.1, these builtins are currently supported:

- Output
- Pedersen
- Range check
- Bitwise
- Elliptic curve operation
- Keccak
- Poseidon
- Range check (96 bits)

Each builtin will have a separate segment in the memory vector and depending on the builtin, you can just input the values you want to run the builtin on or additionally use the output of the builtin.

For example, to use the range-check builtin you can just input the values you want to range-check in a certain memory segment and the CairoVM will create the necessary constraints for you.

For a Pedersen builtin, on the other hand, for each builtin use, you can input two values and read the next cell as output since the VM provides a guarantee that the next cell will contain the correct output (i.e. the Pedersen hash of the two input values).

<div style="text-align: center;">
    <img src="builtin_memory_layout.png" alt="Builtin memory layout" width="80%">
</div>

In order to run a program that uses one of these builtins, we need to specify a specific layout to CairoVM that supports the builtins that the program uses. Below is a subset of layouts that are currently supported by the CairoVM. (Check this [code](https://github.com/lambdaclass/cairo-vm/blob/main/vm/src/types/instance_definitions/builtins_instance_def.rs) for the updated, comprehensive list)

|             | small | recursive | dex | recursive_with_poseidon | starknet | starknet_with_keccak |
| ----------- | :---: | :-------: | :-: | :---------------------: | :------: | :------------------: |
| output      |   O   |     O     |  O  |            O            |    O     |          O           |
| pedersen    |   O   |     O     |  O  |            O            |    O     |          O           |
| range_check |   O   |     O     |  O  |            O            |    O     |          O           |
| bitwise     |       |     O     |     |            O            |    O     |          O           |
| ecdsa       |       |           |  O  |                         |    O     |          O           |
| poseidon    |       |           |     |            O            |    O     |          O           |
| ec_op       |       |           |     |                         |    O     |          O           |
| keccak      |       |           |     |                         |          |          O           |

### Hints

Hints are non-deterministic computation (i.e. code that is run outside of the VM to populate values that are validated inside the VM)

For example, when computing the square root of an integer in a prime field, directly computing this using constraints is expensive. So instead, we can use hint to calculate the square root and create a constraint inside the VM that checks that the power of the given square root is equal to the original value.

So given a computation $x=\sqrt{y}$, compute $y$ using a hint and create a constraint: $y=x*x$

$x$ from the perspective of the VM is a wild guess, but the constraint makes sure that it’s valid.

In Cairo 0 language, you can run a hint by embedding a block of Python code. Below is an example of computing a square root of 25.

```python
[ap] = 25, ap++;
%{
    import math
    memory[ap] = int(math.sqrt(memory[ap - 1]))
%}
[ap - 1] = [ap] * [ap], ap++;
```

When compiling this Cairo 0 program into CASM, the compiler stores this Python code as a string and specifies that this hint should be run before running the ensuing instruction. In this case, the hint should be run before the `[ap - 1] = [ap] * [ap], ap++;` instruction.

## How the "simple bootloader" works

Coming back to the bootloader, as per the previous section, running the bootloader program will result in a memory vector and an execution trace. But since the bootloader program runs Cairo programs inside it, the generated memory and execution trace should include those of the Cairo programs as well. Below is an example of what the memory layout looks like after running the bootloader:

<div style="text-align: center;">
    <img src="simple_bootloader_memory_layout.png" alt="Simple bootloader memory layout" width="80%">
</div>

(Note: a **task** here refers to a single Cairo program and its inputs)

In other words, what the bootloader program does is it makes sure that the inner Cairo programs can write to various segments in the memory without any conflicts with each other or with the outer bootloader program.

This is a "simple" use of the bootloader and we can use this functionality using the `simple_bootloader.cairo` file ([link](https://github.com/starkware-libs/cairo-lang/blob/v0.13.1/src/starkware/cairo/bootloaders/simple_bootloader/simple_bootloader.cairo)) in the Cairo repo. We will discuss an "advanced" use of the bootloader in the next section.

### Some more details on how the "simple" bootloader works

#### What builtins does the bootloader use?

- output
  - the "simple" bootloader doesn't write any output to the memory
- pedersen
  - used to hash the task program bytecode
- range checks

  - used to check that each task program used the builtins correctly.
  - Each builtin has a different amounts of memory cell usage

    ```
      output=1,
      pedersen=3,
      range_check=1,
      ecdsa=2,
      bitwise=5,
      ec_op=7,
      keccak=16,
      poseidon=6,
    ```

  - So for example, since the pedersen builtin requires 3 cells (2 for inputs, 1 for output), the bootloader checks that the difference between the pedersen builtin pointer before and after the task program run is positive (this is where the range check comes in) and that it is divisible by 3.

#### How are task program bytecode loaded into memory?

The bootloader creates a separate segment in the memory for storing each task program bytecode

#### How are task outputs written to memory?

The bootloader writes the task outputs sequentially (i.e. in the order that the task programs are run) to the output builtin segment. Below is an example of what this looks like:

<div style="text-align: center;">
    <img src="simple_bootloader_output_memory_layout.png" alt="Simple bootloader output memory layout" width="100%">
</div>

#### How do task programs use builtins other than the output builtin?

- assuming that the selected layout supports the builtins that the task program uses, the bootloader will pass the necessary builtin pointers to the task program's main function, and the task program will use builtin cells as needed and return the pointers to the next unused builtin cells.
- at the end of the task program run, the bootloader will validate that the builtin pointers have been correctly advanced, as explained in the [What builtins does the bootloader use?](#what-builtins-does-the-bootloader-use) section.

#### How are task program hints handled?

- As mentioned in the [A short primer on how the CairoVM works](#a-short-primer-on-how-cairovm-works) section, hints are specified to run before a certain bytecode is executed. In CairoVM, that refers the the “pc” value, or the program counter, and this value is normally a value in Segment 0. So a typical hint should run before, say when pc=(Segment 0, Index 15).
- However, task program bytecode are loaded into a separate segment (e.g. Segment 11), so the pc values for the hints of the task program need to updated accordingly.

## How the "advanced" bootloader works

The "advanced" bootloader builds upon the "simple" bootloader by allowing as input not just Cairo programs but also a structured set of inputs that include both Cairo programs and simple bootloader proofs. In this section, we will go over how this works in multiple steps.

(The "advanced" bootloader refers to the `bootloader.cairo` file ([link](https://github.com/starkware-libs/cairo-lang/blob/v0.13.1/src/starkware/cairo/bootloaders/bootloader/bootloader.cairo)) in the Cairo repo.)

### Step 1: A Cairo verifier program

First, we need to understand what a Cairo verifier program is.

A Cairo verifier is a Cairo program that verifies a Cairo proof, and in this case, it will only verify a proof generated using the "simple" bootloader. Below is an example of what running a Cairo verifier program looks like.

<div style="text-align: center;">
    <img src="cairo_verifier_example.png" alt="Cairo verifier example" width="70%">
</div>

Note that when the Cairo verifier verifies the simple bootloader proof, it includes in the output the hash of the simple bootloader program and the hash of the outputs of the simple bootloader. This means that it is possible to commit to the output of the simple bootloader using the Cairo verifier and open it later.

### Step 2: Iteratively creating proofs

Looking closely at diagram above, you can see that we can tweak it a little bit to create a perfect loop. Since the simple bootloader can run any arbitrary program and the Cairo verifier requires a proof of the simple bootloader, we can do something like the following:

<div style="text-align: center;">
    <img src="perfect_loop.png" alt="Perfect loop" width="40%">
</div>

Of course, there is no point in just repeating this loop, but if we tweak this just a little bit again, we can do something more interesting: creating a proof that both verifies an existing proof and proves a new program. We'll call this "iteratively creating proofs", which is the core of what the "advanced" bootloader does.

<div style="text-align: center;">
    <img src="iteratively_creating_proofs.png" alt="Iteratively creating proofs" width="60%">
</div>

This might still not seem too useful, but it unlocks a new use-case for users who want to create proofs of programs as they come, not just when they have a fixed set. For example, L2 nodes that need to create a single proof of programs that are transmitted over a 15-minute period do not need to wait for the whole 15 minutes and create a proof at the last minute, which would take a long time if the number of programs to be proven are large. Instead, they can create a proof after the first 1 minute, and for the next 14 times every minute they can create a new proof that recursively proves the previous proof and proves the new programs.

This might still not seem too useful, but it unlocks a new use-case for users who want to create proofs of programs as they come, not just when they have a fixed set. For example, L2 nodes that need to create a single proof of programs that are transmitted over a 15-minute period do not need to wait for the whole 15 minutes and create a proof at the last minute, which would take a long time if the number of programs to be proven are large. Instead, they can create a proof after the first 1 minute, and for the next 14 times every minute they can create a new proof that recursively proves the previous proof and proves the new programs.

### Step 3: Verifying iteratively created proofs

At the end of the iterations, we will have a simple bootloader program proof but its output will be a compressed version of the intermediary outputs (as mentioned in [Step 1](#step-1-a-cairo-verifier-program), the Cairo verifier program will hash the outputs of the simple bootloader program), so there needs to be an additional verification process for the final output. The "advanced" bootloader achieves this using hints: through hints, it receives information about the structure of the intermediary outputs and the final output, and also the pre-images of the hashes used and verify inside the VM that the structure is correct.

You can think of the structure as a tree where each node is either a Cairo program (referred to as a "plain task") or a simple bootloader proof (referred to as a "composite task") and a non-data root node.

<div style="text-align: center;">
    <img src="output_structure.png" alt="Output structure" width="80%">
</div>

As you can see in the diagram above, the colored shapes represent the nodes of the tree, with the "Bootloader input" node being the final node (or the root).

The "advanced" bootloader takes in a "Bootloader input" node as an input, which will contain only the nodes of depth 1 (i.e. "Composite Task 2", "Plain Task 4", and "Plain Task 5"). Once the simple bootloader encounters a composite task, it will do a depth-first search traversal of the tree to find all the plain task nodes, at which point it will write the outputs to the memory.

![Depth-first search traversal](depth_first_search_traversal.png)

As you can see in the diagram above, at the end of the bootloader run, the memory will be populated with the outputs of all the plain task nodes.
