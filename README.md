# Distributed Server Monitoring and Failure Detection

Chatter is distributed server monitoring and failure detection system
that propagate status information over UDP using gossiping protocols.

## Using Tokio

The implementation uses Tokio and is mostly intended to be an example
of how to implement this kind of solution using Rust and Tokio.

## Message Propagation

The current implementation does "rumor mongering" to forward messages
to all members of the cluster.  To improve the message complexity:
just forward the messages to a limited set of members in the cluster.

Messages are forwarded over UDP, so they can be lost. Since each node
will forward gossip to other nodes in the cluster, the likelihood of
losing an update is small.

# How to build

```
cd chatter-rs
cargo build
```

# Open Issues

* Filter duplicate messages.

* Anti-aliasing updates.

* Integration tests.

* Unit tests.

# License

Distributed using the Apache License Version 2.0. See `LICENSE` file
for more information.
