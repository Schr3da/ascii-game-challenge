# Ascii Game Challenge

This repository contains a work in progress ascii game targeting web and the steamdeck using tauri.

## Prerequirements

### Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Node
```
https://nodejs.org/en/download
```

### Yarn
```
npm install -g yarn
```

### Install dependencies for frontend
```
cd ./crates/app
yarn install
```


## Start Application 

### launch frontend 
```
cd ./crates/app
yarn run dev 
```

### launch tauri
Launch the application from the root directory of the repositiory
```
cargo run
```



