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


## License
This project is licensed under either of Apache License, Version 2.0, 
([LICENSE-APACHE](LICENSE-APACHE) or MIT license ([LICENSE-MIT](LICENSE-MIT).
Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in this crate by you, as defined in the Apache-2.0 license, 
shall be dual licensed as above, without any additional terms or conditions.
