## Rusty Relay: High Performance Peer-to-Peer VPN

*__Disclamer__* _Rusty Relay is currently in active development, and there likely will be breaking changes. There are no guarantees, either implied or explicit, made by this software or me at this stage._

&NewLine;

> This program aims to be a secure, performant, & reliable peer to peer VPN, written in Rust. The goal is to to minimize the hassle of configuration and   > deployment, while also having multi-platform support, high security, high performance, and an easily auditable code base.

&NewLine;

To get a better understanding of the objective & timeline of the 'rusty-relay' project, please check out [MOTIVATION.md](https://github.com/Merlin1A/rusty-relay/blob/master/MOTIVATION.md).

### Modifications

'rusty-relay' is based on the [Kytan](https://github.com/changlan/kytan) peer to peer VPN. As the development of 'rusty-relay' progresses, it is likely that it will differ substantially from the original [Kytan] project. For the short term, a list of all major modifications that have been completed so far can be found below.

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
