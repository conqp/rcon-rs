# rcon-rs

Implementation of RCon protocols in Rust.

## BattlEye RCon

To use the *BattlEye RCon* client, enable the `battleye` feature.

Official protocol documentation:

- https://www.battleye.com/downloads/BERConProtocol.txt

## Source RCON

To use the *Source RCON* protocol, enable the `source` feature.

Official protocol documentation:

- https://developer.valvesoftware.com/wiki/Source_RCON_Protocol

## Generic client

A generic client `rconclt` can be build using the features `cli`, `battleye`, and `source`.

## Game-specific extensions

The crate features some extension traits for game-specific capabilities, such as player management and messaging.

### DayZ

By enabling the feature `dayz`, you get additional traits on the `battleye::Client` for DayZ servers.

### Minecraft

An extension for Minecraft for the `source::Client` is planned.
