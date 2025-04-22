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
`match`式は複数のアームで構成される。各アームはマッチされるパターンと`match`式に与えられた値がそのパターンにマッチするときに実行されるコードで構成される。  
Rustのデフォルトはi32型であり、`secret_number`は数値型であるため`guess`も数値型にする必要がある。  
`let guess: u32 =guess.trim().parse().expect("Please type a number!")`  
`guess`という名前の変数を作成する。Rustでは`guess`の前の値を新しい値で**覆い隠す**(shadow)が許される。右辺の`guess`は文字列として格納されたオリジナルの`guess`変数を指す。`trim`メソッドは文字列の先頭と末尾の空白をすべて削除する。ユーザーが例えば5とタイプすると`guess`は`5/n`になる。この「改行」を削除する。`parse`メソッドは文字列をパース（解析）して何らかの数値にする。このメソッドは文字列を様々な数値型へとパース出来るため`let guess: u32`としてRustに正確な数値型を伝える必要がある。`guess`のあとにコロン`:`をつけることで変数の型に注釈をつける。ここで初めて`guess`がu32型と分かる。　　
### ループ  
`loop`は無限ループを作成する。`while(ture){}`みたいな感じ。`quit`を入力すれば終了する。`break`でループを抜けることができる。  
### 不正な入力の処理  
`expect`の呼び出しから`match`式に切り替える。もし`parse`が数値への変換に成功したなら、結果の数値を保持する`Ok`値を返す。そして`num`の値を返す  
### 数当てゲーム
```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number=rand::thread_rng().gen_range(1..101);
     //この時点では具体的な型(u32 or u64)は分からない
    //println!("The secret number is: {}",secret_number);
    //let num=rand::thread_rng().gen_range(-1..3);
    //println!("num={}",num);

    loop{
        println!("Plewase input your guess.");
    
        let mut guess=String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>break,
        };
    
        println!("You guessed: {}",guess);
    
        //guessはu32型だから、それと比較されているsecret_numberもu32型と推測する
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }
    
    }
}
```
## 変数と可変性  
```rust
use std::io;
fn main(){
    let x=5;
    println!("The value of x is: {}",x);
    x=6;
    printlt!("The value of x is: {}",x);
}
```
もしこのようなプログラムがコンパイルされるときエラーが吐き出される。これは不変変数である`x`に値が2度代入されたからである。`mut`により可変にすることで未来の読者に対してコードの別の部分がこの変数の値を変える可能性があるということを示す。
### 定数について  
`const MAX_POINT: u32 = 1000_000;`  
これは定数を示す  
* 定数には`mut`キーワードは使えない  
* 定数は`const`で宣言し、値の型は必ず注釈する必要がる  
* 定数はどんなスコープでも定義できる。いろんなところで使用される可能性のある値を定義するのに役立つ  
* 定数は定数式にしかセットできない。呼び出しは実行時に評価される値にはセットできない  
```rust
use std::io;
const MAX_POINT: u32 = 1000_000;
fn main(){
    let mut x=5;
    println!("The value of x is: {}",x);
    x=6;
    println!("The value of x is: {}",x);

    println!("{}",MAX_POINT);
}
```  
### シャドーイング  
```rust
use std::io;
const MAX_POINT: u32 = 1000_000;
fn main(){
    let x=5;
    let x=x+1;
    let mut x=x;//可変にシャドーイング
    {
        let x=x*2;
        println!("x={}",x); //12
    }
    println!("x={}",x); //5
    let x=12;
    println!("x={}",x); //12
}
```  
**シャドーイングと`mul`の違い**  
`let`を使わずに、誤ってこの不変変数に再代入するとエラーが出る。値にちょっとした加工は行えるが、その加工が終わったら、変数は不変になる。`let`を再度使用したら、実効的には新しい変数を生成していることになる。値の型を変えつつ、同じ変数名を使いまわすことが出来る
```rust
    let spaces="  ";
    let spaces=spaces.len();

    let mut kuhaku=" ";
    kuhaku=kuhaku.len(); //Error missmathed types 
```   

## データ型  
Rustに置ける値はすべて、何らかのデータ型になり、コンパイラがどんなデータが指定されているのか知れるため、そのデータの取り扱い方も把握できる。  
ustは静的型付き言語ある。コンパイル時にすべての変数の型が判明している必要がある。  
`let geusss: u32="42".parse().expect("Not Number");`  
このように型を明示する必要がある。(`parse`はResult型であるため`expect`も必要)
## スカラー型  
整数型、浮動小数点、論理値、文字がある  
### 整数型
小数部分のない数値のこと
| 大きさ | 符号付き | 符号なし |
| ------ | ------- | ------- |   
| 8-bit | i8 | u16 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| arch | isize | usize |  
isizeとusize型はプログラムが作動しているコンピュータの種類に依存する。64ビットアーキテクチャなら64ビット。 
### 浮動小数点型
浮動小数点に対しても、2種類の基本型が存在する。`f32`と`f64`である。基準型は`f64`である。現在のCPUでは、`f32`とほとんどスピードが変わらないからである。
```rust
    let x=2.0; //f64
    let x:f32=3.0; //f32
```  
### 数値演算
```rust
    let x=3.4;
    //let y=2;
    let y:f32=2.0;
    let z=x+y;
    println!("{}",z);
```  
数値演算は小数と整数の間ではできないが、型が異なっていてもできる  
### 論理値型
```rust 
   let t=true;
   let f=false;
```
### 文字型  
```rust
    let c='z';
    let str="zzz";
    println!("{}",str);
    println!("{}",c);
```
`char`型はシングルクォートで指定される。  
## 複合型  
複数の値を1つの型に纏めることが出来る。Rustには基本的にタプルと配列が存在する
### タプル型  
タプルは複数の型から何らかの値を1つの複合型にまとめる。
```rust
    let tup:(i32,f64,u8)=(500,6.5,1);
    let tup2=(500,6.4,1);
    let (x,y,z)=tup2; //x(u32)、y(f64),z(u32)
    //let q=x*y; //error
    println!("{}",q);
    let five_hundred=x.0;
    let six_point_four=x.1;
    let one=x.2;
```  
tupleの値に直接アクセスすることもできる。  
### 配列型  
配列によって複数の値のコレクションを得ることが出来る。ただ、tupleと違い全要素は同じ型でなければならない。またRustの配列は固定長であり、1度宣言されたらサイズを変えることはできない。
```rust
use std::io;
fn main(){
    let a=[1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index=String::new();
    io::stdin()
    .read_line(&mut index)
    .expect("Failde to read line");

    let index: usize=index
    .trim()
    .parse()
    .expect("Not number");

    let element=a[index];
    println!("The number of element at index {} is {}",index,element);
}
```
配列の大きさ以上のindexはエラーになる  
