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

### check

Running `zmsg check` iterates through all available z_address and display transactions where `change == false`.

```shell
$ zmsg check

==========================================================================================
> Got 3 messages.
==========================================================================================
| Message #0 (val = 1)
| To: ztestsapling13vnh6svpvtpgntphha7nsafchx7zhhzu0gnptkztvahtvyueujwha2gcf976vt609qd8xjkaqwf
| Date: Sat May  8, 2021 at 23:48:02
|
|   Thanks for using zfaucet!
==========================================================================================
| Message #1 (val = 0.5)
| To: ztestsapling1hcm5sjeeesrzzhahr9txasjkupu5u0ajstxljy97qytxx3y6qku3w8gyf6tnqjfpz9f9w9jd9eg
| Date: Sat May 15, 2021 at 06:26:13
|
|   second payment
==========================================================================================
| Message #2 (val = 0.01)
| To: ztestsapling1hcm5sjeeesrzzhahr9txasjkupu5u0ajstxljy97qytxx3y6qku3w8gyf6tnqjfpz9f9w9jd9eg
| Date: Sat May  8, 2021 at 23:51:37
|
|   hello zcash
==========================================================================================
```

Note that this will take about a minute to compute the zero-knowledge proof, and another few minutes before the transaction gets propagated and confirmed for the other side to see it.

[rust]: https://rust-lang.org
[zcash-node]: https://zcash.readthedocs.io/en/latest/rtd_pages/zcashd.html