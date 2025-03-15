
#import "@preview/ieee-monolith:0.1.0": ieee

#show: ieee.with(
  title: [Hermes: Building a practical multi-device SPARTA],
  authors: (
    (
      name: "Surendra Jammishetti",
      email: "sjammish@ucsc.edu"
    ),
  ),
  index-terms: ("Anonymous Communication", "Meta-data private messaging"),
  bibliography: bibliography("../refs.bib"),
  figure-supplement: [Fig.],
)

#show link: underline


= Problem Statement

// 1) A paragraph for the problem definition
Initially, I wanted to try to implement Groove, @groove, but after technical /
scope limitations, I pivoted towards Sparta, @sparta. While working
on the implementation for Sparta, ideas from Groove started to influence the path I wanted
to take my work. Groove has a rigorous explanation of multi-device support, which the existing work
for Sparta lacked. The goal for my project is to
embed the high-level design of Sparta with practical messaging semantics, like
multi-device support, authentication, and dialing.

= Implementation
// 2) What you have done so far (that includes papers that you might have
//  read for this project or the implementation progress)


+ Intel-SGX

  Unfortunately, I have an AMD laptop and have no access to an Intel
  machine. Therefore, using the simulator is impossible, and after
  talking with Apostolos, he said it was all right to forego the usage
  of SGX. Porting the current implementation to be SGX compatible is not
  impossible / wouldn't need an entirely new codebase, I would need access to
  an SGX machine and would move the security-relevant code into an enclave
  using #link("https://github.com/fortanix/rust-sgx")[Fortanix's] rust SGX
  development platform, and proxy in the requests from a userspace program
  into the enclave.

+ Sparta-LL

  I have a rudimentary version of Sparta-LL compiling and working, using
  #link("https://github.com/facebook/oram?tab=readme-ov-file")[Facebooks'
  oram] library.  The user store is still a non-oblivious data structure
  (a simple KV map) instead of an OMAP due to my previous difficulties
  understanding how OMAP's work. In the meantime, it is a non-oblivious data
  structure (A simple KV map). However, the message store is fully functional,
  along with the oblivious selection. Other than the user store's incomplete OMAP
  implementation, it works perfectly.

+ Client implementation

  I've completed a barebones implementation of a client sending and receiving messages, you
  can check it out in `/athens`.

== Papers read

=== Messaging

+ Groove, @groove
  - Multi-device, provider model, mix-nets, high latency / overhead.
+ Sparta, @sparta
  - Decided to implement Sparta-LL. Low-overhead, low latency, simple.

=== OMAP's

The following papers I read to understand how to create an efficient OMAP, hoping for more
pseudocode / simpler explanations, but this was before the lecture on Oblivious Data Structures,
so I'm much more confident now. One interesting thing to note is EnigMAP's criticism of Oblix,
showing that they didn't adhere to instruction level obliviousness, which as a concept is very
interesting. Reading their critique made me think about whether it's possible to have a compiler transform
all memory accesses to be oblivious, and do the same with branches, making the user-side implementation
trivial. 

+ Oblivious Data Structures, @wang
+ EnigMAP, @enigmap
+ Oblix, @oblix



= TO-DO

- [ ] implement the OMAP for the user store
	- https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/
	- maybe just do a trivial avl implementation and slap in oram to access nodes?
		- $O(log^3(N))$ time complexity

- [ ] have a simple client side impl, preferably on pc / cli

- [ ] simple mobile client impl

- [ ] proxy / provider model to facilitate multi-device support
    - This would be the anonymizing layer that Kyle talked about in his lecture.

- [ ] Nice documentation for everything

- [ ] End 2 End encryption
\

