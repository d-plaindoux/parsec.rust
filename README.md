# Parser Combinator in Rust

[![Build Status](https://travis-ci.org/d-plaindoux/parsec.rust.svg?branch=master)](https://travis-ci.org/d-plaindoux/parsec.rust)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

# Objective 

A [parser combinator library](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/parsec-paper-letter.pdf)
implementation from scratch in [Rust](https://www.rust-lang.org/en-US/).

# Basic parsers & Macros

## Core definition

A parser is a trait providing a `do_parse method.

```rust
pub trait Parser<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A>;
}
```

## Basic parsers

```rust
returns :: A  -> Parser<char> where A: Copy
fails   :: () -> Parser<A>
any     :: () -> Parser<char>
eos     :: () -> Parser<()>
```

## Macros

### Basic

```rust
do_try!    :: Parser<A> -> Parser<A>
lookahead! :: Parser<A> -> Parser<A>
```

### Monadic

```rust
fmap!      :: (Fn(A) -> B) -> Parser<A> -> Parser<B>
bind!      :: (Fn(A> -> Box<Parser<B>>) -> Parser<A> -> Parser<B>
```

### Flow

```rust
seq!       :: Parser<A> -> Parser<B> -> Parser<(A,B)>
or!        :: Parser<A> -> Parser<A> -> Parser<A>
opt!       :: Parser<A> -> Parser<Option<A>>
optrep!    :: Parser<A> -> Parser<Vec<A>>
rep!       :: Parser<A> -> Parser<Vec<A>>
```


# Example

```rust
// item    ::= [^,]*
// csvline ::= item (',' item)*

let item    = || take_while!(|c| *c != ',');
let csvline = seq!(item(), optrep!(fmap!(|_,b| b, ',', item())));
```

# License

Copyright 2018 D. Plaindoux.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
