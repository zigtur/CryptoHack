# Elliptic curves - Background

## Elliptic curve format
$Y^2 = X^3 + a X + b$

An elliptic curve E is the set of solutions to a Weierstrass equation:
$$E: Y^2 = X^3 + a X + b$$
together with a point at infinity $O$. The constants $a,b$ must satisfy the relationship:
$$4a^3 + 27 b^2 \neq 0$$
to ensure that the are no singularities on the curve.

ECC relies on the hardness of finding the $n$ such that $Q = nP$ given $Q$ and $P$.

![Elliptic Curves Point operations](images/point-operations.PNG)

## Operations
Point addition has the following properties:
- (a) $P + O = O + P = P$
- (b) $P + (−P) = O$
- (c) $(P + Q) + R = P + (Q + R)$
- (d) $P + Q = Q + P$


## Elliptic Curve Cryptography
We study elliptic curves over a finite field $\mathbb{F}_p$. The elliptic curve will no longer be a curve, but a collection of points whose $x,y$ coordinates are integers in $\mathbb{F}_p$.


# Starter
## Point Negation
### Summary of text
$E(\mathbb{F}_p) = \{(x,y) : x,y \in \mathbb{F}_p satisfying: y^2 = x^3 + a x + b\} \cup O$

For all the starter challenges, we will work with $E: Y^2 = X^3 + 497 X + 1768, p: 9739$

The solution is basic: $\forall P in \mathbb{F}_p$ such that $P={x,y}$, $-P={x,-y}$


## Point addition

The algorithm for the addition of two points $P+Q$:
- 1. If $P=O$, then $P+Q=Q$
- 2. Otherwise, if $Q=O$, then $P+Q=P$
- 3. Otherwise, say $P={x_1,y_1}$ and $Q={x_2,y_2}$
  - 1. If $x_1=x_2$, and $y_1=-y_2$, then $P+Q=O$
  - 2. Otherwise:
    - 1.1 if $P \neq Q: \lambda = (y_2 - y_1)/(x_2 - x_1)$
    - 1.2 if $P = Q: \lambda = (3x_1^2 + a) / 2y_1$
    - 2. $x_3 = \lambda^2 - x_1 - x_2$, and $y_3 = \lambda (x_1-x_3) - y_1$
    - 3. $P+Q = (x_3, y_3)$

Remark: As we work in a finite field, calculations are done $mod\ p$. We do not "divide" by an integer, we instead multiply by the modular inverse of a number. (e.g. $1/5 = 9\ mod\ 11$)

Working with $E: Y^2 = X^3 + 497 X + 1768,\ p: 9739$





