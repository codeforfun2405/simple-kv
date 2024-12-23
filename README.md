# Simple KV

A Simple Key Value Server build with **tokio** and **dashmap**.

## Support Commands

* Set
* Get
* Del

## Client

Encode the Cmd(Get, set, del) to protobuf.

## Server

* Decode the tcp bytes to Cmd
* Handle the Command to get / set / del data in Storage.
* Write back the response to the tcp connnection.

## Dependency

```sh
cargo new --bin simple-kv

cargo add prost

cargo add prost-types bytes anyhow thiserror

cargo add tokio-util --features codec

cargo add dashmap

cargo add tokio --features rt --features rt-multi-thread --features macros  --features io-util --features net
```