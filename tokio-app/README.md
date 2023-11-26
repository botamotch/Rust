
1. start up redis server
```
$ redis-server
```

2. setup TCP connection app
```
$ cargo run

new client: 127.0.0.1:60548
socket spawned!; PollEvented { io: Some(TcpStream { addr: 127.0.0.1:6379, peer: 127.0.0.1:60548, fd: 10 }) }
```

3. access to server
```
$ cargo run --example hello-redis

value from server; result=b"world"
```
