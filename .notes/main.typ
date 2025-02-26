#import "@preview/cheq:0.2.2": checklist

#show: checklist

= TODO 

== MVP
- [ ] implement the OMAP for the user store
	- https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/
- [ ] have a simple client side impl, preferably on pc / cli
- [ ] Nice documentation for everything
- [ ] figure out how adding a user works (OMAP semantics)
- [ ] End 2 End encryption
		- Ask him what stuff I need

== Post MVP
- [ ] multi-device support via support of proxy
	- inspired by how groovy had the provider system
	- to make sparta-ll into a provider based system, its fairly cheap to have a small
		embedded device to act as a proxy for each user its not super unfeasable

- [ ] client side implementation with sqlite?

- [ ] figure out how to get this building on fortranix sgx
	- If I dont reach this, could sell this as not feasable in such a short amount of time but looking forward to do it in the future.

= Project Structure

Hermes
- proxy? or could be the name for the entire thing

Athens
- Client Cli
- Tauri mobile app

Sparta
- Sparta LL implementation


= Questions for Kyle
+ Does sparta support users with multiple devices?
+ What sort of E2E encryption scheme can be added onto sparta?


= Qucklinks
oram library:
- https://github.com/facebook/oram?tab=readme-ov-file
	- only secure inside of an enclave with memory encryption

enclave framework:
- https://github.com/fortanix/rust-sgx

intel-sgx?
- https://github.com/intel/linux-sgx-driver
- i dont have the hardware



