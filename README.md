Run server.

```console
$ cargo run --bin server
```

It outputs the name of IpcOneShotServer; named socket.

```
/tmp/.tmpSi0A6n/socket
```

Run client with the name.

```console
$ cargo run --bin client /tmp/.tmpSi0A6n/socket
```

Then, server received the paylodas.

```console
[16, 17, 18, 19]
```
