# bevy_minecraft

A toolkit to create minecraft servers using bevy, in an ECS-native way.

## Status

This project is just starting development, we currently have these "phases" of development planned:

### First Server Ping

- [ ] Implement a structured way to handle the various packets in the minecraft protocol.
- [ ] Run a TCP server as an async task, to handle the incoming connections.
- [ ] Implement a state machine to keep track of the connection state.
- [ ] Implement enough of the packets so that we can receive the ping request, from the Minecraft server list, and respond appropriately.

### Login Sequence

- [ ] Build out more of the packets to handle the full login sequence.
- [ ] Get the client to show the "Joining world" message.

### Joining the world

- [ ] Handle more packets to get the client to join the world (empty world to start).

And much more that will be implemented over time!

## Contributing

All contributions are welcome, but we do enforce [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages. This helps with managing breaking changes, and maintaining and accurate [changelog](CHANGELOG.md).

## License

As most projects in the `bevy` ecosystem, this project is dual-licensed under both the MIT and the Apache 2.0 licenses. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for more information.
