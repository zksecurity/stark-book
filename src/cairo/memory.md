# Public Memory

Here are some notes on how the [Cairo zkVM](https://eprint.iacr.org/2021/1063) encodes its (public) memory in the AIR (arithmetization) of the STARK.

If you'd rather watch a 25min video of the article, here it is:

<iframe width="560" height="315" src="https://www.youtube.com/embed/VkGH3U4L2n4?si=dRrvpDs4wt14UVwJ" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

The AIR arithmetization is limited on how it can handle public inputs and outputs, as it only offer boundary constraints.
These boundary constraints can only be used on a few rows, otherwise they're expensive to compute for the verifier. 
(A verifier would have to compute $\prod_{i \in S} (x - g^i)$ for some given $x$, so we want to keep $|S|$ small.)

For this reason Cairo introduce another way to get the program and its public inputs/outputs in: **public memory**. This public memory is strongly related to the **memory** vector of cairo which a program can read and write to.

In this article we'll talk about both. This is to accompany [this video](https://www.youtube.com/watch?v=VkGH3U4L2n4) and section [9.7 of the Cairo paper](https://eprint.iacr.org/2021/1063).

## Cairo's memory

Cairo's memory layout is a single vector that is indexed (each rows/entries is assigned to an address starting from 1) and is segmented. For example, the first $l$ rows are reserved for the program itself, some other rows are reserved for the program to write and read cells, etc.

Cairo uses a very natural "constraint-led" approach to memory, by making it **write-once** instead of read-write. That is, all accesses to the same address should yield the same value. Thus, we will check at some point that for an address $a$ and a value $v$, there'll be some constraint that for any two $(a_1, v_1)$ and $(a_2, v_2)$ such that $a_1 = a_2$, then $v_1 = v_2$.

## Accesses are part of the execution trace

At the beginning of our STARK, we saw in [How STARKs work if you don't care about FRI](https://cryptologie.net/article/601/how-starks-work-if-you-dont-care-about-fri/) that the prover encodes, commits, and sends the columns of the execution trace to the verifier.

The memory, or memory accesses rather (as we will see), are columns of the execution trace as well.

The first two columns introduced in the paper are called $L_1.a$ and $L_1.v$. For each rows in these columns, they represent the access made to the address $a$ in memory, with value $v$. As said previously, we don't care if that access is a write or a read as the difference between them are blurred (any read for a specific address could be _the_ write).

These columns can be used as part of the Cairo CPU, but they don't really prevent the prover from lying about the memory accesses:

1. First, we haven't proven that all accesses to the same addresses $a_i$ always return the same value $v_i$.
2. Second, we haven't proven that the memory contains fixed values in specific addresses. For example, it should contain the program itself in the first $l$ cells.

Let's tackle the first question first, and we will address the second one later.

## Another list to help

In order to prove that the two columns in the $L_1$ part of the execution trace, Cairo adds two columns to the execution trace: $L_2.a'$ and $L_2.v'$. These two columns contain essentially the same things as the $L_1$ columns, except that these times the accesses are sorted by address.

> One might wonder at this point, why can't L1 memory accesses be sorted? Because these accesses represents the actual memory accesses of the program during runtime, and this row by row (or step by step). The program might read the next instruction in some address, then jump and read the next instruction at some other address, etc. We can't force the accesses to be sorted at this point.

We will have to prove (later) that $L_1$ and $L_2$ represent the same accesses (up to some permutation we don't care about).

So let's assume for now that $L_2$ correctly contains the same accesses as $L_1$ but sorted, what can we check on $L_2$?

The first thing we want to check is that it is indeed sorted. Or in other words:

* each access is on the same address as previous: $a'_{i+1} = a'_i $
* or on the next address: $a'_{i+1} = a'_i + 1$

For this, Cairo adds a **continuity constraint** to its AIR:

![Screenshot 2023-11-21 at 10.55.07 AM](https://hackmd.io/_uploads/S1Yz6u546.png)

The second thing we want to check is that accesses to the same addresses yield the same values. Now that things are sorted its easy to check this! We just need to check that:

* either the values are the same: $v'_{i+1} = v'_i$
* or the address being accessed was bumped so it's fine to have different values: $a'_{i+1} = a'_i + 1$

For this, Cairo adds a **single-valued constraint** to its AIR:

![Screenshot 2023-11-21 at 10.56.11 AM](https://hackmd.io/_uploads/HJIITd5NT.png)

And that's it! We now have proven that the $L_2$ columns represent correct memory accesses through the whole memory (although we didn't check that the first access was at address $1$, not sure if Cairo checks that somewhere), and that the accesses are correct.

That is, as long as $L_2$ contains the same list of accesses as $L_1$.

## A multiset check between $L_1$ and $L_2$

To ensure that two list of elements match, up to some permutation (meaning we don't care how they were reordered), we can use the same permutation that Plonk uses (except that plonk fixes the permutation).

The check we want to perform is the following:

$$
\{ (a_i, v_i) \}_i = \{ (a'_i, v'_i) \}_i
$$

But we can't check tuples like that, so let's get a random value $\alpha$ from the verifier and encode tuples as linear combinations:

$$
\{ a_i + \alpha \cdot v_i \}_i = \{ a'_i + \alpha \cdot v'_i \}_i
$$

Now, let's observe that instead of checking that these two sets match, we can just check that two polynomials have the same roots (where the roots have been encoded to be the elements in our lists):

$$
\prod_i [X - (a_i + \alpha \cdot v_i)] = \prod_i [X - (a'_i + \alpha \cdot v'_i)]
$$

Which is the same as checking that

$$
\frac{\prod_i [X - (a_i + \alpha \cdot v_i)]}{\prod_i [X - (a'_i + \alpha \cdot v'_i)]} = 1
$$

Finally, we observe that we can use Schwartz-Zippel to reduce this claim to evaluating the LHS at a random verifier point $z$. If the following is true at the random point $z$ then with high probability it is true in general:

$$
\frac{\prod_i [z - (a_i + \alpha \cdot v_i)]}{\prod_i [z - (a'_i + \alpha \cdot v'_i)]} = 1
$$

The next question to answer is, how do we check this thing in our STARK?

## Creating a circuit for the multiset check

Recall that our AIR allows us to write a circuit using successive pairs of rows in the columns of our execution trace.

That is, while we can't access all the $a_i$ and $a'_i$ and $v_i$ and $v'_i$ in one shot, we can access them row by row.

So the idea is to write a circuit that produces the previous section's ratio row by row. To do that, we introduce a new column $p$ in our execution trace which will help us keep track of the ratio as we produce it.

$$
p_i = p_{i-1} \cdot \frac{z - (a_i + \alpha \cdot v_i)}{z - (a'_i + \alpha \cdot v'_i)}
$$

This is how you compute that $p$ column of the execution trace as the prover. 

Note that on the verifier side, as we can't divide, we will have to create the circuit constraint by moving the denominator to the right-hand side:

$$
p(g \cdot x) \cdot [z - (a'(x) + \alpha \cdot v'(x))] = p(x) \cdot [z - (a(x) + \alpha \cdot v(x))]
$$

There are two additional (boundary) constraints that the verifier needs to impose to ensure that the multiset check is coherent:

* the initial value $p_0$ should be computed correctly ($p_0 = \frac{z - (a_0 + \alpha \cdot v_0)}{z - (a'_0 + \alpha \cdot v'_0)}$)
* the final value $p_{-1}$ should contain $1$

Importantly, let me note that this new column $p$ of the execution trace cannot be created, encoded to a polynomial, committed, and sent to the verifier in the same round as other columns of the execution trace. This is because it makes uses of two verifier challenges $z$ and $\alpha$ which have to be revealed _after_ the other columns of the execution trace have been sent to the verifier.

> Note: a way to understand this article is that the prover is now building the execution trace interactively with the help of the verifier, and parts of the circuits (here a permutation circuit) will need to use these columns of the execution trace that are built at different stages of the proof.

## Inserting the public memory in the memory

Now is time to address the second half of the problem we stated earlier: 

> Second, we haven't proven that the memory contains fixed values in specific addresses. For example, it should contain the program itself in the first $l$ cells.

To do this, the first $l$ accesses are replaced with accesses to $(0,0)$ in $L_1$. $L_2$ on the other hand uses acceses to the first parts of the memory and retrieves values from the public memory $m^*$ (e.g. $(1, m^*[0]), (2, m^*[1]), \cdots$).

This means two things:

1. the nominator of $p$ will contain $z - (0 + \alpha \cdot 0) = z$ in the first $l$ iterations (so $z^l$). Furthermore, these will not be cancelled by any values in the denominator (as $L_2$ is supposedly using actual accesses to the public memory)
2. the denominator of $p$ will contain $\prod_{i \in [[0, l]]} [z - (a'_i + \alpha \cdot m^*[i])]$, and these values won't be canceled by values in the nominator either

As such, the final value of the accumulator should look like this if the prover followed our directions:

$$
\frac{z^l}{\prod_{i \in [[0, l]]} [z - (a'_i + \alpha \cdot m^*[i])]}
$$

which we can enforce (as the verifier) with a boundary constraint. 

Section 9.8 of the Cairo paper writes exactly that:

![Screenshot 2023-11-21 at 11.31.39 AM](https://hackmd.io/_uploads/HkUiHYcV6.png)
