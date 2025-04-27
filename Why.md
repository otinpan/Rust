## 可変参照
* ある特定のデータに対しては1つしか可変な参照を持てない(一人で編集)  
* 可変と不変の参照を組み合わせることが出来ない(読んでいる間に勝手に変えられる!危ない!)  
* 不変な参照をしている間は可変な参照をすることはできない(みんなで読むだけ)  
* 可変な参照をしている間は元の変数に直接アクセスしてはいけない(編集するから元のもの持ってくな)  

`s1`は`s`を可変として参照する。
```rust
use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s; //可変
 
  s.push_str("!"); 
  s1.push_str(",world"); //データが2か所から編集された
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
`s1`は`s`が先に変更されるこのを嫌がる。  
`s1`は`s`の可変参照になる。可変参照が存在している間、元の変数`s`に直接アクセスすることは禁止。このルールに反しているため。`s1`が生きているあいだに`s`に変更が加えられてはいけない。  
```rust
use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s;  

  s1.push_str(",world"); //s1をここで使い切った
  s.push_str("!");
}
```
これは通る。よくわからん。　　
`s1.push_str(",world");`この行で`s1`を使い切ったと思う。もう`s1`は使われない、`s`にたいして安全にアクセスできるとコンパイラが判断する。NNL(Non-Lexical Lifetimes)という仕組みのおかげで、実際に使われた最後の場所から安全かどうかを判断する。  
```rust
use std::io;
fn main() {
  let mut s=String::from("hello");
  let s1=&mut s;  //ここで1回目の借用が起こっている

  change(&mut s);  //参照元が変更される。ダメ
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
  //s1.push_str(",world"); //「p1はもう使われないからsの変更してもいいよね」
  s.push_str("!");
}

fn change(some_string:&mut String){
  some_string.push_str("world");
}
```
これは通る。

### 文字列スライス型
```rust
fn main() {
    let mut s = String::from("hello");
    let slice: &str = &s;  //不変

    s.clear(); //不変参照のあとにsに変更が加えられた。これはsliceは想定していない
    // println!("{}", slice); 
}
```
`slice`は`s`を参照している。  
* 不変な参照をしている間は可変な参照をすることはできない 

に反している? `print`関数をコメントアウトをするとエラーを吐かなくなる。「`slice`が存在しないけど、使わないなら気にしないよ」って感じ。