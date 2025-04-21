#Rust基本文法
公式 [TheRustProgrammingLanguage](https://doc.rust-jp.rs/book-ja/foreword.html)  
ここで使われる依存とは「コードが必要とする」を意味する  

## 数当てゲーム
```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Plewase input your guess.");

    let mut guess=String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}",guess);
}
```
`use std::io;`はユーザー入力を受け付ける機能などを利用できるようにする  
### 変数
`let mut guess=String::new();`
`mut`はmutable可変の意味で`guess`を変数にする。これがない場合`guess`は不変である。Cの`const`と同じイメージ  
`String::new()`では新しいUTF-8でエンコードされたStrign型の新しい空の値を示す  
### 入力
```rust
io::stdin()
.read_line(&mut guess)
```
ではユーザーの入力を受け付ける。もし最初に`use std::io`と描いていなくても`std::io::stdin()`と書けばこの関数を利用できる。`read_line`の引数として`&mut guess`を与えることでユーザー入力をどの文字列に格納するかを指定する。参照はデフォルトで不変であるため`mut`で可変にしないといけない。Rustの大きな利点の1つは参照を安全かつ簡単に使用できることである。  
`read_line`は同時に`io::Result`も返す。Result型は列挙型で列挙子は`Ok`か`Err`である。`.expect()`は列挙子が`Err`だった場合プログラムをクラッシュさせ、引数のメッセージを表示する。  
### 変数の出力
`println!("You guessed: {}", guess);`  
`{}`は1つ目の変数`guess`を表す。これによって変数を文字列に埋め込み表示することが出来る。  
### クレートを使用して機能を追加する  
これまで作ってきたプロジェクトはバイナリクレートであり、これは実行可能ファイルになる。randクレートはlibraryクレートであり、単独で実行することは出来ない。Cargoの.tomlファイルを編集してrandクレートを依存関係に含める必要がある。.tomlファイルの[dependecis]に`rand = "0.8.3"`を加える。この0.8.3は^0.8.3を意味し、0.83以上0.9.0未満の任意のバージョンを意味する。明示的にアップグレードするまではプロジェクトは0.8.3を使い続ける。クレートを本当にアップグレードしたいとき`cargo update`を使用する。Cargoは.lockファイルを無視して.tomlファイル内のすべての指定に適合する最新バージョンを算出する。ただし、デフォルトでCargoは0.8.3以上0.9.0未満のバージョンのみを検索する。もしrandクレートの新しいバージョンとして0.8.4と0.9.0がリリースされていたなら、`cargo update`を実行しても0.8.4にアップデートされる。もしrandのバージョンを0.9.xにしたいなら.tomlのファイルの[dependecis]を`rand = "0.9.0"`に変更する必要がある。
### 乱数を生成する
```rust
let secret_number=rand::thread_rng().gen_range(1..101);
```
`rand::thread_rng`関数で、これから使う、ある特定の乱数生成期を取得する。そして、`gen_range`メソッドで指定した範囲の乱数を生成する。   
### 数字を比較する  
