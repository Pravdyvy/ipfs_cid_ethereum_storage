# ipfs_cid_ethereum_storage
How to Use:

Prerequisites:

There have to be local ipfs instance and local Ethereum node. To work correctly
ipfs have to be started at localhost:5001, and Ethereum node at localhost:8545.

Clone repository;

Go to directory:

$ cd ipfs_cid_ethereum_storage

Build binary:

$ cargo build

You can run compiled binary(it is ipfs_cid_ethereum_storage/target/debug/ipfs_cid_ethereum_storage)

$ ./ipfs_cid_ethereum_storage -P PATH

where PATH - is path to artifacts and source(for example "~/ipfs_cid_ethereum_storage")


