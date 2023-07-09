# Basalt

> *extrusive igneous rock formed from the rapid cooling of low-viscosity lava rich in magnesium and iron*

## Building

### Build for desktop native

#### Dev dependencies: 

- [mold](https://github.com/rui314/mold) linker in your `PATH`
```bash
$ cargo run
```
### Build for web

```bash
$ darkhttpd ./public --addr 127.0.0.1 --port 8080 & # or other http server that can host files from directory
$ ./scripts/build-demo-web.sh # build with webgl

# optional
$ ./scripts/build-demo-web.sh --webgpu # build with webgpu instead
$ ./scripts/build-demo-web.sh --open # also open in browser on http://localhost:8080 automatically
$ ./scripts/build-demo-web.sh --port 8081 # override default port number
```
