# Modular Math 

## Quadratic Residues

We say that an integer $x$ is a **Quadratic Residue** if there exists an $a$ such that $a^2 = x\ mod\ p$. If there is no such solution, then the integer is a **Quadratic Non-Residue**.

## Legendre Symbol
First, an interesting property of quadratic residues:
| Multiplication           | Quadratic Residue    | Quadratic Non-residue |
|--------------------------|----------------------|-----------------------|
| **Quadratic Residue**    |Quadratic Residue     | Quadratic Non-residue |
| **Quadratic Non-residue**|Quadratic Non-residue | Quadratic Residue     |


**Legendre's Symbol**: $(a/p) \equiv a^{(p-1)/2}\ mod\ p$ obeys:
- (a/p) = 1 if $a$ is a quadratic residue and $a \neq 0\ mod\ p$
- (a/p) = -1 if $a$ is a quadratic non-residue $ mod\ p$
- (a/p) = 0 if $a \equiv 0\ mod\ p$

That means that calculating $a^{(p-1)/2}\ mod\ p$ is enough to determine if $a$ is a quadratic residue.


## Modular Square Root



