```bash

rustup override set nightly
cargo run

```

#### tokenize.rs

Splits input string into tokens

#### base1.rs

Evaluation logic for exprs like `A && B && !C => H = M`

#### base2.rs
Logic for exprs like `H = M => K = D + (D * E / 10)`

#### expr.rs
combines Base1 and Base2


1) Incoming string `A && B && !C => H = M` parses into intermidiate representation like 
```rust
vec![Token::A, Token::And, Token::B]; // and so on
```

2) Then it converts into second called `Executable`. Internally it is https://en.wikipedia.org/wiki/Reverse_Polish_notation
3) Is executes... 


base1 expressions (a && b) compared by `vec![Token]`
base2 expression (D / F) compares by `H = M`

Api examples in main.rs test mod.

GET: http://localhost:8000/true/true/true/1.0/52/1
POST: http://localhost:8000/true/true/true/1.0/52/1  with data 
```json
{"exprs": ["A && B && !C => H = P"]}
```
