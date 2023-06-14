# CryptoHack

## Mathematics

### Lattices


#### Vectors

A vector space $V$ over a field $F$ is a set defined with two binary operators. For a vector $v \in V$, and a scalar $a \in F$, vector addition takes two vectors and produces another vector: $v + w = z$, for $v,w,z \in V$ and scalar multiplication takes a vector and a scalar and produces a vector: $a*v = w$, for $v,w \in V$, $a \in F$.


Let's consider a two dimensional vector space over the reals. A vector $v ∈ V$ can be considered as a pair of numbers: $v = (a,b)$ for $a,b ∈ R$. Vector addition works as $v + w = (a,b) + (c,d) = (a+c, b+d)$, and scalar multiplication by $c*v = c*(a,b) = (c*a, c*b)$.

One can also define the inner product (also called the dot product), which takes two vectors and returns a scalar. Formally we think of this as $v ∙ w = a$ for $v,w ∈ V$, $a ∈ F$. In our two-dimensional example, the inner product works as $v ∙ w = (a,b) ∙ (c,d) = a*c + b*d$.

Time for the flag! Given a three dimensional vector space defined over the reals, where $v = (2,6,3), w = (1,0,0)$ and $u = (7,7,2)$, calculate $3*(2*v - w) ∙ 2*u$.