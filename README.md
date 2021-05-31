### On macos:

```
$ cargo run
# in other pane:
$ ps aux | grep target
# find the `panic_safety` PID
$ sudo dtruss -p PID

# in another pane:
$ curl localhost:3000
$ curl localhost:3000/err
$ curl localhost:3000/panic
```
