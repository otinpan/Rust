## 可変参照
ある特定のデータに対しては1つしか可変な参照を持てない。  
`s1`は`s`を可変として参照する。
```rust
use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s; 

  s.push_str("!"); 
  s1.push_str(",world"); //データが2か所から借用されたと思い込んだ?
  s.push_str("!");
}
```
```
warning: unused import: `std::io`
 --> src/main.rs:1:5
  |
1 | use std::io;
  |     ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:6:3
  |
4 |   let s1=&mut s;
  |          ------ first mutable borrow occurs here
5 |
6 |   s.push_str("!");
  |   ^ second mutable borrow occurs here
7 |   s1.push_str(",world");
  |   -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
warning: `practice` (bin "practice") generated 1 warning
error: could not compile `practice` (bin "practice") due to 1 previous error; 1 warning emitted
```
`s1`は`s`が先に変更されるこのを嫌がる
```rust
use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s;  

  s1.push_str(",world");
  s.push_str("!");
}
```
これは通る。よくわからん
```rust
use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s;  //ここで1回目の借用が起こっている

  change(&mut s);
  s1.push_str(",world");
  s.push_str("!");
}

fn change(some_string:&mut String){
  some_string.push_str("world");
}
```  
```
et s1=&mut s;  //ここで1回目の借用が起こっている
  |          ------ first mutable borrow occurs here
5 |
6 |   change(&mut s);
  |          ^^^^^^ second mutable borrow occurs here
7 |   s1.push_str(",world");
  |   -- first borrow later used here
  ```
  ```rust
  use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s;  //ここで1回目の借用が起こっている

  change(&mut s);
  //s1.push_str(",world");
  s.push_str("!");
}

fn change(some_string:&mut String){
  some_string.push_str("world");
}
```
これは通る。