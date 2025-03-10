#import "@preview/cheq:0.2.2": checklist

#show: checklist

= TODO 
https://github.com/fortanix/rust-sgx/issues/401

== MVP
- [ ] implement the OMAP for the user store
	- https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/
	- @oblix
	- @wang
	- maybe just do a trivial avl implementation and slap in oram to access nodes that way?
		- $O(log^3(N))$ time complexity
	- nahhh just do the dumb version, linked list user store, just compare each one
		to make this version oblivious, need to traverse the entire list each time.
			- $O(N log(N))$

- [x] user server implementation to create new user
- [ ] Add TLS

- [ ] have a simple client side impl, preferably on pc / cli
- [ ] Nice documentation for everything
- [ ] figure out how adding a user works (OMAP semantics)
- [ ] End 2 End encryption


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
+ How does authentication work with oblivious systems? 


= Qucklinks
oram library:
- https://github.com/facebook/oram?tab=readme-ov-file
	- only secure inside of an enclave with memory encryption

enclave framework:
- https://github.com/fortanix/rust-sgx

intel-sgx?
- https://github.com/intel/linux-sgx-driver
- i dont have the hardware



= Kyle Notes


encryption isnt sufficient to protect messaging

"with enough metadata you dont really need content" - NSA

theoretically sparta can have multiple layers
an anonymyzing layer could be used to aggregate your devices and then pull them that way.

#bibliography("refs.bib")



