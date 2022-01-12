# Loquaz
A simple desktop app for encrypted direct messages on Nostr protocol

# Build steps

## 1. Install rust
see https://www.rust-lang.org/tools/install

## 2. OS's requirements

### Linux

On Linux, Druid requires gtk+3; see [GTK installation page](https://www.gtk.org/docs/installations/linux/). (On ubuntu-based distro, running sudo apt-get install libgtk-3-dev from the terminal will do the job.)

```
apt update
apt install libgtk-3-dev
```

## 3. Clone 

```
git clone  https://github.com/emeceve/nostr-chat.git 
```
## 4. Build and run!!

Change directory
 ```
cd nostr-chat
```
build and run

```
cargo run
```
