# Hours Helper WASM

> PizzaQL hours helper written in Rust and compiled to WASM.

[![Build Status](https://travis-ci.org/pizzaql/hours-helper-wasm.svg?branch=master)](https://travis-ci.org/pizzaql/hours-helper-wasm)

## Install

```
$ npm install @pizzaql/hours-helper
```

## Usage

```js
import * as wasm from '@pizzaql/hours-helper';

wasm.get_time(3, true);
```

## Build

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

To build a npm package run:

```
$ wasm-pack build
```

## License

MIT

