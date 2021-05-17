# zmsg

A zero knowledge messaging system built on zcash.

zmsg uses the encrypted memo field of zcash sheilded transactions to send messages to other parties. The sent messages get stored in the zcash blockchain and the recipient can check for messages at any time (no need to be online at the same time). Since the messages get stored in the blockchain, they are on every full zcash node. The implication here is that its not possible to tell who the target of any given message is.

Currently, each message sends 0.0001 ZEC. You can change this value by setting the  `--txval` flag on `sendmsg`.

Installation
------------
First, make sure you have [rust][rust] installed, then:

```shell
cargo install github.com/firmalabs/zmsg
```

Usage
-----
Note: To use zmsg, you'll need a running [zcash daemon][zcash-node] (node), a z_address, and some spare ZEC in that address.

### sendmsg

```shell
$ export TARGET_ZADDR=zchfvC6iubfsAxaNrbM4kkGDSpwjafECjqQ1BZBFXtotXyXARz2NoYRVEyfLEKGCFRY7Xfj2Q3jFueoHHmQKb63C3zumYnU
$ zmsg sendmsg --to=$TARGET_ZADDR "Hello zmsg! welcome to pretty secure messaging"
Message sent to zchfvC6iubfsAxaNrbM4kkGDSpwjafECjqQ1BZBFXtotXyXARz2NoYRVEyfLEKGCFRY7Xfj2Q3jFueoHHmQKb63C3zumYnU with opid = <operation id>
```

Note that this will take about a minute to compute the zero-knowledge proof, and another few minutes before the transaction gets propagated and confirmed for the other side to see it.

[rust]: https://rust-lang.org
[zcash-node]: https://zcash.readthedocs.io/en/latest/rtd_pages/zcashd.html