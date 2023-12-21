# Code Theory

* I think "succinct proofs and linear algebra" provides a good intro and notation

## Hamming weight

hamming weight of a string is the number of non-zero entries.

> The Hamming weight of a string is the number of symbols that are different from the zero-symbol of the alphabet used

For $s = (1, 0, 0, 1, 1)$ we have:

$$
\text{hamming_weight}(s) = 3
$$

in other words for $\vec{x} \in F^n$ we have:

$$
\text{hamming_weight}(\vec{x}) = |\{ i | x[i] \neq 0 \}|
$$

## Hamming Distance

> In information theory, the Hamming distance between two strings of equal length is the number of positions at which the corresponding symbols are different.

$$
\text{hamming_distance}(\vec{x}, \vec{y}) = |\{ i | x[i] \neq y[i] \}|
$$

## Repeated code

$$
G = [I_n, \cdots, I_n]^t
$$

($I_n$ is the identity, repeated $k$ times in the matrix)

$$
G \vec{x} = [x[0], \cdots, x[n-1], x[0], \cdots, x[n-1], \cdots]
$$

($\vec{x}$ repeated $k$ times)

## Reed–Solomon error correcting code

$$
RS[F, D, \rho] := \{ f : D \rightarrow F : deg(f) < \rho \cdot |D| \}
$$

"the RS code of rate $\rho$ evaluated over $D$".

## Johnson bound

What is Johnson bound?

> TODO: We should explain it here, and also make sure we have defined everything we need to understand it.

Let $V \in F^D$ be a code with minimum relative distance $1 - \rho$, for $\rho \in (0, 1)$.
Then $V$ is $(1 - \sqrt{\rho} - \epsilon, 1/(2 \epsilon \sqrt{\rho}))$-list-decodable for every $\epsilon \in (0, 1 - \sqrt{\rho})$.

> TODO: is this the conjecture, or is conjecture 2.3 from the DEEP paper the conjecture based on the Jonhson bound?
