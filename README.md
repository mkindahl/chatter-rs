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

# Usage

## How to build

```
cd chatter-rs
cargo build
```

## How to start

* To listen on the default port (port 2428) and all available
  addresses:

  ```
  target/debug/chatterd
  ```

* To listen on a specific address or port:

  ```
  target/debug/chatterd --listen 127.0.0.1:8080
  ```

* To set the log level (it can be `error`, `warn`, `info`, `debug`, or
  `trace`).

  ```
  RUST_LOG=debug target/debug/chatterd
  ```

## Using `chatter-inject`

It is possible to inject gossip into the network using the
`chatter-inject` utility. It accepts an address of the format
`192.0.2.1:8080` and a JSON string representing the gossip message.

```
chatter-inject 192.0.2.1:8080 '{"DebugMessage":{"text":"hello world"}}'
```

If only an address is provided, the JSON message is read from standard
input. For example:

```
chatter-inject 192.0.2.1:8080 <<END_OF_JSON
{
    "DeviceGossip": {
	"DeviceAdded": {
	    "origin": "541b10e7-d13a-45e8-8567-7d450ce86603",
	    "name": "gateway",
	    "description": "ASUS Router model RT-N55U "
	}
    }
}
END_OF_JSON
```

# Open Issues

* Filter duplicate messages.

* Anti-aliasing updates.

* Integration tests.

* Unit tests.

# License

Distributed using the Apache License Version 2.0. See `LICENSE` file
for more information.
