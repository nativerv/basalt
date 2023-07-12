# Basalt

> *extrusive igneous rock formed from the rapid cooling of low-viscosity lava rich in magnesium and iron*

## Building

### Build for desktop native

#### Dev dependencies: 

- [mold](https://github.com/rui314/mold) linker at `/usr/bin/mold`
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
## Todo

- [ ] theming (a way to set background, foreground, primary accent, secondary accent from a config file/environment variables/CLI options)
- [ ] graph renderer for notes
    - [ ] `Graph` trait
    - [ ] mapper `[String] -> impl Graph` (note contents to graph)
    - [ ] basic renderer of specific instance of Graph
    - [ ] force-based placement
    - [ ] hand edits/corrections to force-placed nodes, saved in JSON format in *notes directory* inside a hidden *metadata directory*
- [ ] live-updated markdown renderer (github-flavored markdown)
    - [ ] mermaid (or at least some other) diagram support
- [ ] event socket (send app events): send clicked links/graph notes (markdown file names/links), etc: for extendability (external editor, etc)
- [ ] command socket (receive commands): open notes by name, edit note contents (re-render), etc: for extendability (external editor, etc)
