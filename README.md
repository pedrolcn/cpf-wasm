# cpf-wasm

The fastest node cpf validation lib out there.  
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

A synthetic load of validating 10 million cpfs was ran in order to benchmark our lib against the most popular cpf validation libs on npm.
The libs we benchmarked against were:
- [cpf](https://npmjs.com/package/cpf)
- [@fnando/cpf](https://npmjs.com/package/@fnando/cpf)
- [cpf-check](https://npmjs.com/package/cpf-check)

The results obtained show that `cpf-wasm` is at least 1.53 X faster than other libs performing the same task

| lib          | time       | ratio    |
|:_____________|:__________:|:________:|
| **cpf-wasm** | **9.23s**  | **1.00** |
| cpf          | 19.02s     | 2.06     |
| @fnando/cpf  | 14.10s     | 1.53     |
| cpf-check    | 20.33s     | 2.20     |

benchmark code can be found at the `benchmark` folder

## Was this really necessary?
Probably not

## Should I use this in production?
Probably not