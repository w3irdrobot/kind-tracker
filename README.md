# Kind Tracker

Simple program to listen to all events starting from ninety days ago, tally up the number of each event kind that is seen, and output them to the terminal once the timeout is hit.

```shell
# defaults to a timeout of 60 seconds
cargo run
# custom timeout specified in seconds
cargo run -- 20
```
