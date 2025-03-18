#import "@preview/polylux:0.4.0": *
#import "@preview/metropolis-polylux:0.1.0" as metropolis
#import metropolis: new-section, focus

#show: metropolis.setup

#show link:underline;

// Problem Defenition
// Threat Model
// Your Approach
// Implementation
// Experiments / Results

#slide[
  #set page(header: none, footer: none, margin: 3em)

 
  #text(size: 1.3em)[
    *Hermes*
  ]

   A practical SPARTA-LL Implementation

  #metropolis.divider
  
  #set text(size: .8em, weight: "light")
  Surendra Jammishetti

  Mar 16, 2025

  CSE 108C

]

#slide[
  = Problem Defenition

  - Current anonymous messaging systems aren't resilient to traffic analysis attacks.

  - SPARTA lays a framework for a fast, traffic analysis resistant solution.

  - Common pitfalls, such as user validation, that can starve user messages.

  // are we sure we wanna keep this in.
  - Lacks details for multi-device communication.
]


#slide[
  = Threat Model and Security Guarantees

  == Adversary
  - Inheriting SPARTA's threat model of a *global active attacker* who can
    - control / modify all network links
    - participate in the protocol
    - observe traffic for an arbitrary amount of time
    - can breach everything on the server excluding the enclave code

  == Differential Privacy
  - Guarantee that adversary cannot corrolate that one user is messaging another.
    - The base SPARTA-LL construction already acheives this.
]

#slide[
  = My Approach

  #image("arch.jpg")

]

#slide[
  = Implementation Details

  - No enclave (lack of hardware)

  - 2.5k lines of rust

  - GRPC as messaging protocol

  - #link("https://github.com/facebook/oram")[Facebook ORAM Implementation]

  - O(N log(N)) Implementation for UserStore OMAP

  - ed25519 Signatures to determine Fetch authenticity

  - Proxy has queue per device to hold older messages
]

#slide[
  = Results
  #align(center)[#image("fetch.png")]


]
