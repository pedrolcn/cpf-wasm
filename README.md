# CPF-WASM

Probably the fastest node cpf validation lib out there.  
Written in rust, compiled to WASM

## Installation
Install using the package manager of your choice, it already ships with typings

```bash
$ npm add cpf-wasm
$ yarn add cpf-wasn
```

## Quick start

```typescript
const { isValid } = require('cpf-wasm');

isValid('41154981452')
// true

isValid('11111111111')
// false
```

## Benchmarks

coming soon

## Was this really necessary?
Probably not

## Should I use this in production?
Probably not