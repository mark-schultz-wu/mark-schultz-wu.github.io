---
layout: post
title: An Introduction to Zero-Knowledge Proofs
---

My advisor is running a topics seminar in cryptography, and for better or worse
many of the students who signed up have no cryptographic background.
As a result, we have been encouraged to give talks that keep this in mind,
meaning that are

* non-technical (to the extent possible), and
* focus on *definitions* over constructions.

As I plan on talking about [some recent work in lattize-based zero-knowledge
proofs](https://eprint.iacr.org/2022/284), I thought it might make sense to
write up my notes for the talk as a blog post.
Of course, there are many other introductions to zero-knowledge proofs.
Still, as I was writing up notes anyway, I thought it might make sense to stick
them here.

# Overview of Proofs and Knowledge

Zero-Knowledge proofs are a particularly interesting area in cryptography, as
they show the power of a definition.
In particular, to define a zero-knowledge proof, you need to

* define what a "proof" is, and
* define what "knowledge" is.

While defining proofs are relatively straightforward, how the concept of
"knowledge" is technically captured has always held a special place in my heart.
Without seeing the definition, it can be hard to imagine how you could capture
something as abstract as "knowledge".
Despite this, a perfectly convincing definition exists.
Before getting into technical definitions, lets first work on a
non-technical level, and get an idea of what both of the above mean.

## What are proofs?

A "proof" is at least a relatively straightforward concept to formalize.
In its colloquial sense, you prove something to someone if you convince them of
the veracity of it.
This can come in many forms --- those of us who had proofs-based math courses
may imagine writing a few lines of reasoning, which someone
who is reading can easily verify the correctness of.
There are other forms of proofs though --- for example, if my initial few lines
are not convincing enough, an unconvinced party could ask clarifying questions,
which I could then respond to.

First, to even define what a proof is, we need some notion of truth.
Fortunately, computer scientists sidestep the whole issue of "what is truth" via
the notion of a *language*, which I won't review.
Roughly, we consider the set of all (finite-length) binary strings
$$\{0,1\}^\ast$$, and say that a *language* is a subset $$L\subseteq
\{0,1\}^\ast$$.
Then, some string $$x$$ is "true" if $$x\in L$$, and "false" if $$x\not\in L$$.
Simple (albiet somewhat technical).

Now, how might we produce a proof?
Really, what we are interested in aren't individual proofs, but what are known
as *proof systems*, or techniques to generate and verify proofs.
In computing, we phrase a proof system as an interactive protocol (or, a
dialogue, if you must) between a prover $$P$$ and a verifier $$V$$.
Proof systems have two abstract properties we care about

* Completeness: Can you prove every true statement?
* Soundness: Can you prove *false* statements?

These properties are typically quantified in some way, but ideally a proof
system would be such that you can prove every true statement, but not false
statements (known as *perfect* soundness and completeness).

As a basic example, many American students take geometry courses with "two
column proofs".
You have to construct various shapes using a straightedge and compass.
To do this, you split the construction into a sequence of easily-verifiable
steps.
This can be viewed as a particularly simple proof system (for the language of
"shapes constructable in plane geometry").

## How could a proof be Zero-Knowledge?

*Zero-knowledge* is an additional property a proof system can have.
The intuitive idea is that we want the verifier to be convinced that $$x\in L$$
is actually true, but "nothing more".
For example, if there was a way for the verifier to be convinced our geometry
student's homework was correct *without reading it*, it is clear that the
verifier "learned nothing" from the homework, e.g. it was a zero-knowledge
proof.

This works for (trivial) situations, but not reading the proof risks allowing a
nefarious prover to produce a *fake proof* of a *false statement* $$x\not\in
L$$.
Ideally, a proof system would both be sound (and not have "fake proofs"), while
these proofs "not being too informative".
How could this possibly work?

Fortunately, there is a relatively straightforward (non-technical) example,
namely that of *wine tasting*.
Say you have some Sommelier Sam, who wants to sell a course on wine tasting.
He's trying to make a sale to a cynic Cedric, who thinks that fancy wines and
cheap wines taste the same.
Sam could teach Cedric his craft, so Cedric can tell the difference between
fancy wines and cheap wines.
But then he would have no reason to buy Sam's course!
What is Sam to do?

A basic way to convince Cedric is what is known as *blinded tasting*.
Cedric chooses any two different wines, and blindfolds Sam.
Cedric then poors a glass of wine, and lets Sam taste it.
Sam tries to guess which wine it was from the taste.

Now, if Sam *didn't* know the difference between the two wines, he has at best a
1/2 shot of getting it right.
For this reason, the proof isn't too convincing (yet).
Of course, Cedric can always *repeat* this test.
If he repeats it $$k$$ times, the chance that Sam (by luck) passes all the
trials is at most $$1/2^k$$.
Very quickly, this will become a *very* convincing demonstration, despite Sam
not giving up his underlying technique.

This gives some (non-technical) indication how you can prove something without
"revealing knowledge".
It does not yet give an indication of what the *right* definition of knowledge is though.
I'll delay that to the technical section.

# Technical Definition of a Proof System

The typical definition of a proof system is via the complexity class [IP](https://en.wikipedia.org/wiki/Interactive_proof_system), of "Interactive Proof Systems".
First, note that our (non-technical) discussion of proof systems stated that a
(convincing) written statement is a form of a proof.
Typically, such a statement is called a *witness* $$w$$ to the inclusion $$x\in
L$$.
The class of languages $$L$$ for which there exists an efficient Turing Machine
$$V(x, w)$$ such that whenever $$x\in L$$, there exists a witness $$w$$ such
that $$V(x, w) = 1$$ is precisely the complexity class NP.
From these "written proofs", we get that whatever we end up formalizing IP as,
it should contain NP as a complexity class, which is perhaps somewhere to start.

Next, we can look at our wine tasting example.
Key to it was that the verifier (Cedric) can flip coins, and use these (random)
choices to build certain tests for a prover (Sam), such that Sam either needs to
* "actually know" what he is talking about, or
* get *really* lucky.

The idea of a randomized verifier is fairly important to the construction of
proof systems, so we might as well throw this into things.
This already bumps us up from NP to a slightly larger class known as
[Merlin-Arthur](https://en.wikipedia.org/wiki/Arthur%E2%80%93Merlin_protocol#MA).
The name refers to what you might think (myths from old England), and there is a
reason for it, namely to distinguish it from Arthur-Merlin (on the same page).
This distinction isn't relevant to this post though, and it suffices to think of
MA as a "randomized" variant of NP.

While IP ends up being *much* larger than NP/MA (it is famously equal to
PSPACE), it will be useful for us to restrict to interactive proofs for which
there is a well-defined witness $$w$$, e.g. restrict to interactive proofs for
languages in NP/MA.
These are known as [*proofs of knowledge*](https://en.wikipedia.org/wiki/Proof_of_knowledge).
Defining precisely what an interactive protocol is (in terms of the machines
$$P$$ and $$V$$) is somewhat annoying, and not too enlightening, so I won't do
it.
It proceeds roughly how you might imagine, namely $$P$$ and $$V$$
send messages back and forth, until $$V$$ chooses to end the protocol and output
0 or 1.
I will write this process $$[ P\leftrightarrow V ](x, w)$$, with the understanding
that only $$P$$ gets $$w$$ as an input.

> An *interactive proof* for a language $$L$$ is a pair of randomized poly-time
> turing
> machines $$(P, V)$$ such that
> * **Completeness**: For any $$x\in L$$, there exists a witness $$w$$ such
>   that $$\Pr[[P\leftrightarrow V ](x,w) = 1] \geq 2/3$$, and
> * **Soundness**: For any $$x\not\in L$$, for any string $$w$$, 
    $$\Pr[[P\leftrightarrow V](v,w) = 0] \geq 2/3$$.

As is typical in complexity theory, the choice of 2/3 is arbitrary, and can be
any constant that is non-negligibly larger than 1/2.
Moreover, repeating the protocol can "boost" both constants 2/3 to be
arbitrarily close to 1.

# Technical Definition of Zero-Knowledge

Now that we have a technical definition of a proof system, where is the
definition of knowledge (and zero-knowledge) that I have been so excited about?
In the above formalization of a proof system, the "knowledge" is easy to spot
--- it is the string $$w$$.
Both $$P$$ and $$V$$ are assumed to be efficient (randomized) machines, so the
"extra thing" that $$P$$ knows and $$V$$ doesn't is purely this witness $$w$$.

It's worth thinking about what the general point of the definition of
zero-knowledge is.
For example, we could have a proof system $$(P, V)$$ that contains as a message
the pair $$(Enc_k(w), k)$$, where $$k$$ is the key of some (secure) encryption scheme.
On one hand, this "hides" $$w$$ (we encrypt it!).
On the other hand, a cheating verifier could decrypt it, so it doesn't really
hide anything.
Therefore, the property of being zero-knowledge is a property of *the prover*,
that protects against *any possible cheating verifier*.

How might we formalize this?
One way is to say define what is called a *simulator* $$S$$ that, given access
to any (potentially cheating) verifier $$V^\ast$$, can "simulate its output".
This means that the distribution (over the randomness of the internal coins
$$r$$ used) of $$S^{V^\ast(x)}$$ is very close to the distribution of the output
of $$V^\ast(x)$$ in the interactive protocol $$[P\leftrightarrow V](x, w)$$.
Importantly, the simulator can "guess" $$V^\ast$$'s output *without knowing the
witness* $$w$$.
Formally

> An interactive proof system $$(P, V)$$ for an NP language $$L$$ and NP
> relation $$R_L = \left\lbrace (x, w) : x\in L}\right\rbrace$$ (and $$w$$ is a witness --- this latex
> compiler is not good) is said to be
> *zero-knowledge* if there exists a PPT oracle machine $$S$$ such that, for
> every PPT algorithm $$V^\ast$$, it holds that
> 
> $$\left\lbrace [P\leftrightarrow V^\ast](x, w)\right\rbrace_{(x, w)\in R_L}
> \approx \left\lbrace S^{V^\ast}(x)\right\rbrace_{x\in L}$$

This idea can take a while to come to terms with.
For an *excellent* in-depth look into simulators, I would recommend
Lindell's [*How to Simulate
it*](https://eprint.iacr.org/2016/046.pdf).

Note in the above definition that the simulator does *not* depend on the witness
$$w$$.
So for any $$x\in L$$, for any $$V^\ast$$, we can "guess" the output of
verification, without knowing a particular witness.
One might think this lets us decide whether $$x\stackrel{?}{\in}L$$, but notice
the above definition doesn't say what the simulator does when $$x\not\in L$$, so
it may not be particularly useful.


# The Standard Zero-Knowledge Proof

I'll next go over the standard zero-knowledge proof (of 3COLOR), before
discussing the paper linked at the beginning of this blogpost.
Given a graph $$G = (V, E)$$, recall the 3COLOR problem

> A 3-coloring of a graph is a function $$\varphi : V\to \{0,1,2\}$$ such that
> for all edges $$(a,b)\in E$$, $$\varphi(a) \neq \varphi(b)$$.

3COLOR is a typical NP-complete problem.
A zero-knowledge proof for 3COLOR therefore implies a zero-knowledge proof for
any language in NP.

The proof itself is simple.
The prover first randomizes the colors in the 3-coloring, e.g. replaces
$$\varphi(x)$$ with $$\varphi'(x) = \psi(\varphi(x))$$, where $$\psi$$ is a
random permutation of $$\{0,1,2\}$$.



