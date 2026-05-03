---
layout: page
title: ensemble
nav: true
nav_order: 2
---

# 🍜 docs.dev: ensemble

- [🍜 docs.dev: ensemble](#-docsdev-ensemble)
  - [mathematical framework: ensemble](#mathematical-framework-ensemble)
    - [the weight set $W$](#the-weight-set-w)
      - [idea: Gating](#idea-gating)

---

At the end of the day, LASA similarity is **perceptual similarity**. So, we must consider all features that is attributed to how humans **identify** a specific word.

- **segmental** (sound),
- **suprasegmental** (stress), and
- **structural** (syllable).

## mathematical framework: ensemble

Computing perceptual similarity is inherently an ensemble of functions that look into one of the features mentioned above:
$$\textsf{PerceptSim}(X,Y) = w_{1}\cdot\textsf{Segmental}(X,Y) + w_{2}\cdot\textsf{Suprasegmental}(X,Y) + w_{3}\cdot\textsf{Structural}(X,Y)$$
Where:

- $w_{1}+w_{2}+w_{3}=1$
- $X,Y\in\mathcal{\Sigma^*}$
- $\mathsf{PerceptSim}\in[0,1]$

> We can **component algorithms** (like $\textsf{Segmental}$) may be ensembles themselves. Furthermore, selection of component algorithms (and the algorithms that compose them) should be informed by Filipino linguistics.
>
> The **weight set** $W=\{w_{1},w_{2},w_{3}\}$ are **learned** from a dataset $\mathcal{D}$ which contains an ordered pair of drug names from the Philippine human drug registry. $D=P \cup U$, the subset $P$ is the set of true perceptually similar words while $U$ is the set of unlabeled pairs (may or may not be perceptually similar). Also, $|U|\gg |P|, P\cap U=\emptyset$.

### the weight set $W$

This section discusses ways on how the weight set $W$ may be assigned

#### idea: Gating
>
>💡Not all features are important for specific words.
>
> 🤓 This idea directly addresses the limitations of previous literature.
> Previous literature assigned equal weights or learned weights without considering the interactions of each algorithm (example, a high $\mathbf{LCS}$ will also result in a relatively high $\mathbf{LEV}^{-1}$!). Consider the example where $\mathsf{PerceptSim}$ is composed of $n$ individual components, and $\mathbf{Aline}$ is the only phonetically-informed algorithm. The remaining $n-1$ component algorithms will skew the resulting score.

$$
\textsf{PerceptSim}(X,Y) = G_{\textsf{Segmental}}(X,Y)\cdot\textsf{Segmental}(X,Y)
+ G_{\textsf{Suprasegmental}}(X,Y)\cdot\textsf{Suprasegmental}(X,Y)
+ G_{\textsf{Structural}}\cdot\textsf{Structural}(X,Y)
$$
Where:

- $\sum_{i=1}^3 G_{i}(X,Y)=1$

We may consider initializing $W$ from whatever $X$ and $Y$ are given. Suppose $X$ and $Y$ are relatively short words (say, one syllable), a gating function $G(X,Y)$ assigns a weight on the fly. In this case, the gating function attributed to $\textsf{Segmental}$ will dominate:
$$G_{\textsf{Segmental}}\gg G_{\textsf{Suprasegmental}} + G_{\textsf{Structural}}$$
It could even be the case that: $G_{\textsf{Segmental}} =1$ while the other two are 0.

Of course the downside is the computational cost of $\textsf{PerceptSim}$ and training each $G_i$.
