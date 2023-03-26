## Tasks

### serve
```
cd www
export NODE_OPTIONS=--openssl-legacy-provider
npm run start
```

### compile
```
wasm-pack build
```

### watch
```
fswatch -o -r ./src|xargs -I {} wasm-pack build --debug
```