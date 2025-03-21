#import "@preview/ieee-monolith:0.1.0": ieee
#show: ieee.with(
  title: [Hermes \ Building a practical multi-device  SPARTA],
  authors: (
    (
      name: "Surendra Jammishetti",
      organization: [CSE 108C],
      email: "sjammish@ucsc.edu"
    ),
  ),
  index-terms: ("TEE", "Anonymous Messaging"),
  bibliography: bibliography("refs.bib"),
)


// Introduction. Describe the problem, discuss attacks that lead to your solution, discuss prior works
// that provide a solution for the same problem, potential applications of these works, get inspired by
// the papers given to you.


= Introduction

With the threat of a global adversary looming over online communication, its becoming a more pressing concern
to have secure, relaible, messaging services. We came up with E2E encryption to protect the contents of our
messages, but its not enough to protect against a global adversary. E2E encryption doesnt hide the metadata of
our conversations, as the adversary can still reconstruct who is talking to who, and when. As former NSA Chief Gen. Michael
Hayden said, "The U.S. government kills people based on metadata" #cite()

The SPARTA construction offers a metadata-private anonymous communication system, and for the first part of my project
it'll detail the implementation of SPARTA-LL. After initially reading the Groove paper, which has support
for users to have multiple devices via a untrusted provider, it inspired me to try incorporating similar
functionality into my project. Additionally I've been able to get my SPARTA implementation running
inside an AWS Nitro Enclave!   

// Technical Sections. Describe the methods, algorithms, or formal results related to your project
// as precisely and concisely as you can. Generously use examples for clarity.
//
= Base Sparta

== Facebook PathORAM discussion

= Multi-Device Extension
// Implementation. Focus on the key design decisions in your code. Do not paste your source code
// (unless you think it is necessary for some reason); summarize how it works.
//
//
// 
// Experiments. Include setup details (e.g., what machine you used), results presented in tables or
// figures, and observations. Always comment on your results. What should we take away from them?
= Experiments and Results
//
//
//
// 
= Conclusion
// Conclusion. This should be short with the goal to remind the reader of the points that you think
// are the most important.
