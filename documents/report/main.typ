
#import "@preview/algo:0.3.6": algo, i, d, comment, code
#import "@preview/ieee-monolith:0.1.0": ieee
#show: ieee.with(
  title: [Hermes \ A Practical Multi-Device  SPARTA],
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

#show link: underline



// Introduction. Describe the problem, discuss attacks that lead to your solution, discuss prior works
// that provide a solution for the same problem, potential applications of these works, get inspired by
// the papers given to you.


= Introduction

With the threat of a global adversary looming over online communication,
its becoming a more pressing concern to have secure, relaible, messaging
services. We came up with E2E encryption to protect the contents of our
messages, but its not enough to protect against a global adversary. E2E
encryption doesnt hide the metadata of our conversations, as the adversary
can still reconstruct who is talking to who, and when. As former NSA Chief
Gen. Michael Hayden said, "The U.S. government kills people based on metadata"
#cite(<nsa>), necissating the need for metadata-private systems.


Groove #cite(<groove>), an existing system, uses mix-nets and public providers
to offer a metadata-private solution, but has many pitfalls. It is an
synchronous system, requiring and limited to one message per round. Additionally,
the latencies are in the order of epoch times, with a really complex architecture
due to the underlying mixnets to route messages.

The SPARTA #cite(<sparta>) construction offers a metadata-private, anonymous communication
system, and for the first part of my project it'll detail the implementation
of SPARTA-LL. Then, taking inspiration from Groove, I wanted to add multi-device
functionality to SPARTA. Additionally I've been able to get my SPARTA
implementation running inside an AWS Nitro Enclave!

== Threat Model
We inherit the threat model presented by the original SPARTA paper. The adversary is a global
active attacker with the following capabilities. 

- control / modify all network links
- participate in the protocol
- observe traffic for an arbitrary amount of time
- can breach everything on the server excluding the enclave code

// Technical Sections. Describe the methods, algorithms, or formal results related to your project
// as precisely and concisely as you can. Generously use examples for clarity.
= Base Sparta

For the core implementation of SPARTA, I followed the pseudocode provided in the paper #cite(<sparta>).
Since an oram is required for all of SPARTA's internal data structures, I used
#link("https://github.com/facebook/oram/tree/main")[#text(fill: blue)[Facebook's implementation]].

== Facebook Oram discussion

The facebook PathORAM implementation requires two things to be cryptographically secure;
oram clients are running in a secure enclave architecture, that also provides memory
encryption as they do not encrypt on write themselves. Additionally they implement the
oblivious position map and stash from Oblix #cite(<oblix>). Like Oblix, its a pure
rust implementation, and is recursive in nature. It recurses down until the size of
the oram is small enough to fit inside of a Linear Time ORAM, which maintains oblivousness by
reading/writing over each memory location per access. I'm unsure why they chose to have a
constant time ORAM ($O(N)$) for the base case instead of a PathORAM ($O(log(N))$). It would
be interesting to benchmark the difference between these two base cases.


== Personal Implementations

I implemented the other data structures and operations myself, using Facebooks PathORAM as the
underlying oram data structure.

+ Oblivious Select

  I ended up making a function that would take in a conditional, and two integers, where
  it would execute in constant time without branching and would return the first integer if
  the condition was true, and the second integer if it was false. I used oblivious
  select in send / fetch implementation, using it to select between two pointers
  obliviously.

+ Oblivious Multi-Queue

  The oblivious multi-queue is baked into the send and fetch operations, as their pseudocode
  constructs this multi-queue by reading the queue location from the user store and then en/de queueing when necessary. 

+ Oblivious Map

  The oblivious map used for the userstore is a custom construction thats not as efficient
  as state of the art. It boasts a time complexity of  $O(N log(N))$, as it iterates
  through every element in the allocated oram. It's akin to the LinearTime Oram in the
  Facebook oram library. I implemented it this way because its really simple and
  trying to go for an Oblix #cite(<oblix>) or Wang #cite(<wang>) style OMAP would
  take way too long to get correct, ecpecially as the only person working on this
  project.
  
//TODO: just do this later bro

== Technical Details
The core of the sparta server is a GRPC server that binds to a virtual socket inside of
an AWS Nitro Enclave. As the enclave implementation was only done in the past week, there
are a few holes that need to be cleaned up for it to be a deployable instance of SPARTA.

+ TLS

  Amazon has support for enclave traffic to be encrypted via TLS from inside the enclave,
  ensuring that any client and SPARTA have a secure channel for communication. It would
  require to move the core logic and state out of the GRPC server and instead wrap it
  with a HTTP server, and then proxy the traffic via NGINX as described #link("https://aws.amazon.com/about-aws/whats-new/2020/10/announcing-aws-certificate-manager-for-nitro-enclaves/")[#text(fill: blue)[here]].

Currently there is a custom, insecure,  GRPC proxy, called Trojan that wraps around the SPARTA
vsock and enables the exchange of traffic between SPARTA and clients. I originally chose
GRPC as the protocol as its highly efficient with serialization and deserialization and
generally has better DX compared to HTTP. 
  
= Multi-Device Extension
// Implementation. Focus on the key design decisions in your code. Do not paste your source code
// (unless you think it is necessary for some reason); summarize how it works.

I propose a multi-device extension to SPARTA, via a trusted proxy on the
client side. This trusted proxy would be responsible for fetching said clients
messages from sparta on a configurable interval, and making a copy of each real
message, as well as the number of dummy messages that each device would have
received. Then the clients devices would fetch the messaages they need, when
ever they please from this trusted proxy. This would reduce the bandwidth
requirement for a mobile device / other low power devices to zero when not
in use.

The key idea is that the proxy would
send the messages stored for that specific device, as well as the number of dummy
messages that would have been received for that device. We need to send these
dummy messages since the threat model describes an adversary who can monitor all network
links. If we assume that the proxy doesnt send any dummy messages and the
adversary observers a clients device pinging the proxy, which then returns
a small datagram, the adversary can infer that the client hasnt received much traffic
between the last time they fetched compared to now. This leakage would break the
notion of traffic analysis resistance as the adversary now knows precicely how much real
traffic there is per client, simply by observing the proxy's response to client fetch
requests.

This notion of the proxy sending the client real messages, and then dummy messages to fill in
the volume of messages that have been fetched, can be seen as an extension of deffered retreival
between the device and the proxy.

// what else

== Implementation details
  The proxy is implemented as a GRPC server that interally has a client connected to Trojan. It
  spawns a thread to fetch from SPARTA with the granularity specified by cli arguments, and
  then the server takes care of responding to client requests.

  Additionally, the proxy has an authentication
  layer to ensure that any requests made are coming from verified devices, whose public keys
  have been stored on the proxy ahead of time. The proxy then verifies the requests's signature
  with its stored public key, and will only process the request if the verification succeeds. I
  chose this form of authentication as its reminiscent of ssh and was also relatively easy to implement.
  The authentication layer can be changed out for any other scheme as well.

// Experiments. Include setup details (e.g., what machine you used), results presented in tables or
// figures, and observations. Always comment on your results. What should we take away from them?
= Experiments and Results
All experiments were ran on an AWS m5.xlarge with the ami-04acda42f3629e02b image id for the
base os image. The user store was set to a size of 256 for all tests. 

= Conclusion and Future Work
// Conclusion. This should be short with the goal to remind the reader of the points that you think
// are the most important.
