# nostr-chat
A simple desktop app for encrypted direct messages on Nostr protocol

# Install

### Linux

Install rust
see https://www.rust-lang.org/tools/install

On Linux, Druid requires gtk+3; see [GTK installation page](https://www.gtk.org/docs/installations/linux/). (On ubuntu-based distro, running sudo apt-get install libgtk-3-dev from the terminal will do the job.)
```
apt update
apt install libgtk-3-dev
```
Change directory and build
```
cd nostr-chat
cargo build
```
run nostr-chat
```
./target/debug/nostr-chat
```
