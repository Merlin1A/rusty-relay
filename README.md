## Rusty Relay: High Performance Peer-to-Peer VPN

`rusty-relay` is a high performance peer to peer VPN written in Rust, based on the [Kytan peer to peer VPN](https://github.com/changlan/kytan).
The goal is to to minimize the hassle of configuration and deployment, while also having
multi-platform support, high security, high performance, and an easily auditable code base.

### Modifications

Since 'rusty-relay' is completely based on the [Kytan peer to peer VPN](https://github.com/changlan/kytan), it is useful to denote changes and 
additions, especially in the short term. Below is a list of all major modifications that have been completed so far.

### Supported Platforms

- Linux
- macOS (Client mode only)

### Installation

You can compile it from source if
your machine is installed with [Rust](https://www.rust-lang.org/en-US/install.html).

```
$ git clone https://github.com/Merlin1A/rusty-relay.git
$ cd rusty-relay
$ cargo build --release
```

### Running `rusty-relay`

For complete information:

```
$ sudo ./rusty-relay -h
```

#### Server Mode

Like any other VPN server, you need to configure `iptables` as following to make
sure IP masquerading (or NAT) is enabled, which should be done only once. In the
future, `kytan` will automate these steps. You may change `<INTERFACE>` to the
interface name on your server (e.g. `eth0`):

```
$ sudo iptables -t nat -A POSTROUTING -s 10.10.10.0/24 -o <INTERFACE> -j MASQUERADE
```

To run `rusty-relay` in server mode and listen on UDP port `9527` with password `hello`:

```
$ sudo ./rusty-relay server -k hello 
```
If you want open log display (`info` is log level, you can change it by your idea)

```
$ sudo RUST_LOG=info ./rusty-relay server -k hello 
```

#### Client Mode

To run `rusty-relay` in client mode and connect to the server `<SERVER>:9527` using password `hello`:

```
$ sudo ./rusty-relay client -s <SERVER> -p 9527 -k hello
```

if you want open log display (`info` is log level, you can change it by your idea)

```
$ sudo RUST_LOG=info ./rusty-relay client -s <SERVER> -p 9527 -k hello
```

### License

Apache 2.0
