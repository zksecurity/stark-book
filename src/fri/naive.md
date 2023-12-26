## How does FRI work?
FRI protocol relies on a fact that if a source polynomial $p_0$ can be recursively folded into a constant polynomial, then $p_0$ has an expected upper bound degree. By folding, we mean each round halves the degree of $p_0$.

Simply put, it is a protocol for verifier to quickly check if a polynomial has a upper degree bound claimed by prover.

This post will explain the intuition of the math behind how polynomial folding works through some simple calculation examples end to end. These examples will demonstrate a naive version of FRI protocol without the the uses of commitment, which will explained in another post. 

## What is folding?

The core of the protocol relies on a technique to recursively fold the original polynomial and derive a constant value at the end. Here is an example:

$$
\begin{align*}
p_0\text{ : } & 1 + 2x + 3x^2 & \\
p_1\text{ : } & 3 + 3y & \\
p_2\text{ : } & 6 &
\end{align*}
$$


$p_0$ is the source polynomial. After a round of folding, it results in $p_1$. Then, the final round results in a constant polynomial $p_2$.

To see how this folding algorithm works, please refer to the [code](https://github.com/katat/fri/blob/ab5aad54b8fd1e37b881ed7558d6ad22b6911442/src/poly.rs#L77).

These folded polynomials have the following relationship with their source polynomials:

$$p_i\left(x^2\right)=\frac{p_{i-1}\left(x\right)+p_{i-1}\left(-x\right)}{2}+\beta_i \frac{p_{i-1}\left(x\right)-p_{i-1}\left(-x\right)}{2 x}$$


*For simplicity, the random number $\beta$ will be assumed as 1 in this post. We would touch the implication behind this random number in another post that explains the commitment.*

$p_0$ is the original polynomial, and $p_2$ is a constant polynomial that can't be folded anymore. 

These folded polynomials can be traced back to their source polynomials. Thus, the evaluations in a folded polynomial can be accumulated from the corresponding source polynomial. 

These relationships can serve as a proof for verifier to the consistency among the evaluations. The consistency check implies if there is a $p_0$ exists that the prover claimed.

With the same folding setup, the final polynomial will be always the same value. Verifier can check against the same final value against the reduced values from different evaluation points.

If the final polynomial can be derived after expected rounds of folding, it implies the $p_0$ has an expected upper bound degree. In other words, if the final $p_k$ can be derived after $k=log(d)$ rounds of folding from $p_0$ which is of degree $d$, then $p_0$ has upper bound degree $d$.

More formally, there are two properties:
- the number of rounds $k=log(d)$ to derive the final polynomial implies the expected upper bound degree $d$
- the same final value can be re-used to check consistency of the evaluation claims across layers

We will refer this folding example again with some actual evaluations to demonstrate the interesting relationship among them.

## What is the problem?

Using the notation from [what is proving](https://nmohnblatt.github.io/zk-jargon-decoder/intro_to_zk/what_is_proving.html), we define the following:

$$
\mathcal{L}(\mathcal{R}_{\text{consistent-layers}})
$$

for instance, $\left(a_1, w_1\right) \in \mathcal{R}$, where $a_1$ can represent a FRI query and constant value from $p_2$, while $w_1$ represents the corresponding evaluations across layers.

So the problem is to do the check to see if $a_1$ is in $\mathcal{L}$. In other words, the verifier checks if the prover can provide $w_1$ satisfying the query $a_1$.

## naive FRI

Let's assume prover is honest guy in this naive version of FRI. The goal is for verifier to do fast low degree test using the proof from prover. 

### Hard honest work by prover

Prover can use the folded polynomial to obtain the evaluations for each layers. These evaluations of layers can serve as a proof for the verifier to sample for their consistency across layers.

Using the same polynomial example shown earlier, here are two simple calculation examples.

For evaluation point at *1*:

$$
\begin{align} 
\text{Layer 1:} & \\
x_0&=1 \\
p_0(x_0) &= 1 + 2x_0 + 3x_0^2 \\ 
p_0(1) &= 6 \\ 
p_0(-1) &= 2 \\\\
\text{Layer 2:} & \\
x_1&=x_0^2 \\
p_1(x_1) &= 3 + 3x_1 \\
p_1(1^2) &= 6 \\ 
p_1(-1^2) &= 0 \\\\
\text{Layer 3:} & \\
x_2&=x_1^2 \\
p_2(x_2) &= 6 \\
p_2((1^2)^2) &= 6 \\\\
\end{align}
$$

Similarly, for evaluation point at *2*:

$$
\begin{align} 
\text{Layer 1:} & \\
x_0&=2 \\
p_0(x_0) &= 1 + 2x_0 + 3x_0^2 \\ 
p_0(2) &= 17 \\ 
p_0(-2) &= 9 \\\\
\text{Layer 2:} & \\
x_1&=x_0^2 \\
p_1(x_1) &= 3 + 3x_1 \\
p_1(2^2) &= 15 \\ 
p_1(-2^2) &= 9 \\\\
\text{Layer 3:} & \\
x_2&=x_1^2 \\
p_2(x_2) &= 6 \\
p_2((2^2)^2) &= 6 \\\\
\end{align}
$$



In this manner, the prover can keep calculating for the whole domain. Then they would response the verifier query with the evaluations across layers for checking if an instance $a_i$ corresponding to the query is in the $\mathcal{L}\left(\mathcal{R}_{\text{consistent-layers }}\right)$

### Consistency check

Supposed the verifier samples the evaluation point *2* as demonstrated in the example above, these evaluations across layers for that point will be provided by the prover.

Based on the relationship implied from the equation of folding polynomial, the verifier can check the consistency among the evaluations between layers. Let's review and put this equation into use:

$$p_i\left(x^2\right)=\frac{p_{i-1}\left(x\right)+p_{i-1}\left(-x\right)}{2}+\beta_i \frac{p_{i-1}\left(x\right)-p_{i-1}\left(-x\right)}{2 x}$$


This equation is useful for the verifier to check the consistency of the evaluations across layers. That is, $p_i$ can be evaluated using two symmetric evaluations from its source polynomial $p_{i-1}$ by connecting the source domain $x$ and folded domain $x^2$.

Take the `query = 2` or $a_2$, which corresponds to the $x_0=2$ for example, the prover send the following evaluations as witness $w_2$:


$$
\begin{align}
\text{Layer 0:} & \quad p_0(2) = 17, \quad p_0(-2) = 9 \\
\text{Layer 1:} & \quad p_1(2^2) = 15, \quad p_1(-(2^2)) = -9 \\
\text{Layer 2:} & \quad p_2((2^2)^2) = 6
\end{align}
$$

Then the verifier recursively plug in these evaluations to the equation $p_i$:

$$
\begin{align} 
\text{Check layer 2: } & \\
p_1(2^2) &= \frac{p_0(2) + p_0(-2)}{2} + \frac{p_0(2) - p_0(-2)}{2 \times 2} \\ 
 & \\ 
&= \frac{17 + 9}{2} + \frac{17 - 9}{4} = 13 + 2 = 15 \\\\ 
\text{Check layer 3(final):} & \\ 
p_2((2^2)^2) &= \frac{p_1(2^2) + p_1(-2^2)}{2} + \frac{p_1(2^2) - p_1(-2^2)}{2 \times 2^2} \\
&= \frac{15 - 9}{2} + \frac{15 + 9}{2 \times 2^2} = 3 + 3 = 6 \end{align}
$$

Indeed, the value provided by prover, $p_2((2^2)^2)$, is consistent with the recursively accumulated sum from symmetric points on previous layers.

Therefore, $(a_2,w_2)$ is a pair in $\mathcal{R}_{\text {consistent-layers }}$. So through a single query, the prover tries to convince the verifier that $p_0$ has an expected degree bound. The verifier may want to sample more points to ensure the soundness, which we will try to answer what it means in FRI.

For the next, we will see how to deal with a malicious prover by improving this naive version.

*Keep in mind the verifier only knows the evaluations of these polynomials. Here the notation $p_i$, where the i represent the layer number, is just to relate to the evaluations at required points for checking consistency instead of doing the polynomial evaluation directly at verifier side.