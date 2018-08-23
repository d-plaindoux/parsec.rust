# Parser Combinator in Rust

[![Build Status](https://travis-ci.org/d-plaindoux/parsec.rust.svg?branch=master)](https://travis-ci.org/d-plaindoux/parsec.rust)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

# Objective 

A [parser combinator library](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/parsec-paper-letter.pdf)
implementation from scratch in [Rust](https://www.rust-lang.org/en-US/).

# Parsers

## Core definition

A parser is specified by the following `Trait`.

```rust
pub trait Parser<A> {}
```

Since the Parser size is not known Rust does not allow the Trait type to be returned and used as is. For this reason each parser is denoted by a specific
structure (`struct`) and the corresponding `Parser` trait implementation.

## Basic parsers

module `parsecute::parsers::core`

```rust
pub fn returns<A>(v: A) -> Return<A>;
pub fn fail() -> Fail;
pub fn any() -> Any;
pub fn eos() -> Eos;
```

```rust
pub fn satisfy<E, A>(p: E, f: Box<Fn(&A) -> bool>) -> Satisfy<E, A> where E: Parser<A>;
pub fn do_try<E, A>(p: E) -> Try<E, A> where E: Parser<A>;
pub fn lookahead<E, A>(p: E) -> Lookahead<E, A> where E: Parser<A>;
```

### Monadic 

module `parsecute::parsers::monadics`

```rust
fn fmap(self, f: Box<(Fn(A) -> B)>) -> FMap<E, A, B> where Self: Parser<A>;
fn bind(self, f: Box<(Fn(A) -> R)>) -> Bind<E, A, R, B> where Self: Parser<A>;
```

### Flow

module `parsecute::parsers::flow`

```rust
then       :: Parser<A> -> Parser<B> -> Parser<(A,B)>
or         :: Parser<A> -> Parser<A> -> Parser<A>
opt        :: Parser<A> -> Parser<Option<A>>
optrep     :: Parser<A> -> Parser<Vec<A>>
rep        :: Parser<A> -> Parser<Vec<A>>
take_while :: (Fn(&char) -> bool) -> Parser<Vec<char>>
take_one   :: (Fn(&char) -> bool) -> Parser<Option<char>>
```

## Literals

module `parsecute::parsers::literals`

`char` and `string` data types implement the `do_parse` method.

```rust
digit        :: () -> Parser<char>
letter       :: () -> Parser<char>
natural      :: () -> Parser<i32>
string_delim :: () -> Parser<String>
char_delim   :: () -> Parser<char>
```

# Example

```rust
// item    ::= [^,]*
// line ::= item (',' item)*

let atom = || take_while(Box::new(|c| *c != ','));
let line = atom().then(','.then(atom()).fmap(Box::new(|(_,b)| b)).optrep());
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
