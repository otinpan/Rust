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
## 関数  
Rustの関数と変数の命名規則は、**スネークケース**を使うことが慣例。スネークケースとは全文字を小文字にし、単語区切りにアンダースコアを使うこと。また、Rustにおいては関数をどこで定義してもよい。どこかで定義されているか、ということだけ気にする。
```rust
use std::io;
fn main(){
    println!("Hello,World!");

    //another_function(-1); //関数の引数の型と一致していないからError
    another_function(5,'k');
}

fn another_function(x:u32,unit_label: char){
    println!("u32={},cahr={}",x,unit_label);
}

//引数の型を宣言しないとError
/*fn another_function2(x){
    println!("{}",x)
}*/
```
### 引数  
引数には2つの種類がある。引数の部分に実際の値を与えることができこの実際の値を**実引数**とよぶ。それ以外を**仮引数**という。Rustの関数では各仮引数の型を宣言しなければならない。
### 関数本体は文と式を含む  
関数本体は、文が並び、最後に式を置くか文を置くという形で形成される。Rustは式指向型言語であり、これは理解すべき重要な差異になる。
```rust
fn main(){
    let y=6;//文である
}
```  
これは文である。関数定義も文になり、全体としても文になる。文は値を返さない。故に、`let`文を他の変数に代入することはできない。
```rust
fn main(){
    let x=(let y=6);
}
```  
Errorになる。`let y=6`という文は値を返さないため、`X`に束縛するものはない。これはCやRubyとは異なる。`x=y=6`はダメ。
```rust
use std::io;
fn main(){
    let y={
        let x=3; //これは文
        x+1  //これは式
    }; //これも文

    //println!("{}",x); //スコープ外で定義されたErrorになる
    println!("{}",y);
}
```
新しいスコープを作る際に使用するブロック({})は式である。ブロックの中身を見ると`x+1`はセミコロンがついていないことに気づく。式は終端にセミコロンを含まない。終端にセミコロンをつけたら文に変わり、値を返さない。また、文は式を含むことが出来る。
### 戻り値  
関数はそれを呼び出したコードに値を返すことが出来る。戻り値に名前を付けはしないが、`->`の後に型を描いて宣言する必要がある。また、関数の最後は文ではなく式にし、値を返す。
```rust
use std::io;
fn main(){
    let x=2+five();
    let y=plus_one(6);

    println!("x={},y={}",x,y);
}

fn five()->i32{
    5
}

fn plus_one(x:i32)->i32{
    x+1
}
```
もし関数の最後の行を式ではなく文にしてしまうと(`x+1;`)エラーになり、型が合いませんというメッセージを吐かれる。これは定義で`i32`型を返すと行っているのに文を返してしまっているからである。
## 制御フロー  
```rust
use std::io;
fn main(){
    let number=3;
    //制御式はbool型
    if number<5{
        println!("lower than 5");
    }else if number<10{
        println!("lower than 10");
    }else{
        println!("higher than 9");
    }
}
``` 
制御式は`bool`型でなけらばならない。条件式と紐づけられる一連のコードは**アーム**と呼ばれる。`else if`で条件を分岐させることが出来る。`if`文は式にもできるため`let`文の右辺にも持ってくることが出来る。
### let文内でif式を使う  
```rust
use std::io;
fn main(){
    let condition=true;
    let number={
        if condition{
            println!("Yes");
            -5
        }else{
            println!("No");
            6
        }
    };
    println!("{}",number);
}
```
`let`文内で`if`文を使う子もできるが`if`文は最後に式を与える必要がある。また、`if`と`else`の返す値の型は一致している必要がある。例えば
```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```
はで`if`ブロックでは整数、`else`ブロックでは文字列に評価されエラーになる。
## ループでの繰り返し  
### loop
`loop`を使用すると、同じコードを何度も永遠に繰り返す。loopは強制的に`ctr+c`で抜け出すことが出来る。
```rust
use std::io;
fn main(){
    let mut count=0;
    'counting_up:loop{
        println!("count={}",count);
        let mut remaining=10;

        loop{
            println!("remaining={}",remaining);
            if remaining==9{
                break;
            }
            if count==2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("End count={}",count);
}
```  
c言語と同じように`break`や`continue`を使うことが出来る。また`loop`に``counting_up`というラベルがついていることがわかる。このように**ループラベル**を使用することで`break`や`continue`を適用するループを指定することが出来る。
### while  
```rust
use std::io;
fn main(){
    let mut number=3;
    while number!=0{
        println!("{}",number);
        number-=1;
    }
}
```  
c言語のように`while`で条件付ループを作ることも出来る。`while(num<5)`のようなもの  
### for  
```rust
use std::io;
fn main(){
    let a=[10,20,30,40,50];
    let mut index=0;
    while index<5{
        println!("{}",a[index]);
        index+=1;
    }
}
```  
このようにすれば配列の中身をすべて見ることができる。ただ、これは配列の長さをミスしたり、ループの回数ごとに境界地チェックを行うため遅い。そこで`for`ループを使う  
```rust
use std::io;
fn main(){
    let a=[10,20,30,40,50];

    for element in a{
        println!("{}",element);
    }
}
```
バグの可能性を削除する。また値を順番に更新することもできる。c++での`for(int i=0;i<n;i++)`のようなもの。
```rust
use std::io;
fn main(){
    for number in 1..4{
        println!("{}",number);
    }

    //reverseする
    for number in (1..4).rev(){
        println!("{}",number);
    }

    //1個とばし
    for number in (1..10).step_by(2){
        println!("{}",number);
    }
}
```  
**華氏、摂氏変換**  
```rust
use std::io;
fn main(){
    let f:f64=32.2;
    let c:f64=20.5;

    println!("{}",convert_to_celsius(f));
    println!("{}",convert_to_fahrenheit(c));
}

fn convert_to_celsius(f:f64)->f64{
    5.0/9.0*(f-32.0) as f64
}

fn convert_to_fahrenheit(c:f64)->f64{
    9.0/5.0*c+32.0 as f64
}
```  
**フィボナッチ数列**  
```rust
use std::io;
fn main(){
    let mut input=String::new();
    let mut num:u32;

    //数値の入力
    loop{
        println!("please input number");

        io::stdin()
        .read_line(&mut input)
        .unwrap();
    
        num=input.trim().parse().expect("can not convert");

        if num<=0{
            continue;
        }else{
            break;
        }
    }

    if num==1||num==2{
        println!("1");
        return;
    }
    let mut pre=1;
    let mut ppre=1;
    let mut now=2;
    for i in 2..num{
        let keep=now;
        now=pre+ppre;
        ppre=pre;
        pre=now;
    }

    println!("{}",now);
}
```
## 所有権
所有権はRustの最もユニークな機能であり、これのおかげでガレージコレクタなしで安全性担保を行うことができる。Rustにおいて所有権がどのように動作するのかを理解することは重要になる。GC(ガベージコレクション)とはプログラムが使わなくなったメモリを自動で回収する仕組みのことである。例えば、Python、Java、C#、Go、RubyにありC、C++、Rustにはない。メリットは手動でメモリ管理をする必要がない、メモリリークやクラッシュを防ぎやすい、安全で直感的なコードがかける。デメリットは実行中にGCが走ると「一時的な遅延」がでることがある、高速リアルタイム処理には向かない、メモリ使用量がやや多くなることがある。Rustは第3の選択肢を持つ。メモリはコンパイラがコンパイル時にチェックする一定の規則とともに所有権システムを通じて管理される。所有権機能は実行中にプログラムの動作を遅くすることはない。
### スタックとヒープ  
スタックもヒープも、実行時にコードが使用できるメモリの一部になるが、異なる手段で構成される。  
スタックは、**last in, first out**であり**push**で追加し、**pop**で取り出す。この方法は高速であるがデータはすべて既知の固定サイズでなければならない。Cでいうところの型宣言みたいなもの(?)。このアクセス方法のおかげでスタックは高速。データが常に一番上に存在するためデータを取得する場所を探す必要がない。  
ヒープはサイズが可変のデータを格納する。ヒープにデータを置くとき、蟻サイズのスペースを求める。OSはヒープに十分な大きさの空の領域を見つけ、使用中に資ポインタを返す。この過程は**allocateing on the heap**とよばれる。データアクセスはポインタを追って目的の場所に到達しなければならないためスタックよりも低速になる。C言語では`malooc()`、`free()`でヒープを使う。  
### 所有権規則
* Rustの各値は、**所有者**と呼ばれる変数と対応している
* いかなるときも所有者は一つである
* 所有者がスコープから外れたら、値は破棄される  
### 変数スコープ
**スコープ**とは要素が有効になるプログラム内の範囲のこと。
```rust
{                      // sは、ここでは有効ではない。まだ宣言されていない
    let s = "hello";   // sは、ここから有効になる

    // sで作業をする
}                      // このスコープは終わり。もうsは有効ではない
```  
### String型
文字列リテラル(`"Hello!"`など)は便利であるがテキストを使いたいかもしれないすべての場面において最適なわけではない。それは文字列リテラルが不変であり、またコードをかく際にすべての文字列が判明するわけではないからである。不変であるためスタックに保存され高速である。Rustには文字列リテラルのほかに`String`型が存在する。これは可変でありヒープに保存される。  
```rust
let mut s=String::from("Hello");
s.push_str(",world!");
```
このように文字列リテラルを`String`型に変換することが出来、可変化する。  
### メモリと確保
文字列リテラルの場合、中身はコンパイル時に判明しているので、テキストは最終的なバイナルファイルにハードコードされる。そのため高速で効率的になる。対して可変である`String`型はコンパイル時に不明な寮のメモリをヒープに保存する  
* メモリは実行時にOSに要求される
* `String`型を使用したらOSにメモリを変換する方法が必要

ガベージコレクタ(GC)月の言語ではGCがこれ以上使用されないメモリを検知して片付けるため、プログラマはそのことを考慮する必要はない。GCがないなら、メモリがもう使用されないことを見計らい、明示的に返還しなければならない。これはプログラマの責任になる。もし返還し忘れていたらメモリが無駄になり、2回解放してもバグになる。`allocate`と`free`は完璧に1対1対応しなければならない。  
Rustは異なる道を歩む。メモリを所有している変数がスコープを抜けたらメモリは自動的に返還される。変数がスコープを抜けるとき、Rustは特別な関数`drop`を呼ぶ。これでメモリを解放する。
### ムーブ
Rustには複数の変数が同じデータに対して異なる手段で相互作用することが出来る。
```rust
let x=5;
let y=x;
```
値`5`を`x`に束縛してから`x`の値をコピーして`y`に束縛する。固定サイズの単純な値であるためこれらの2つの`5`はスタックにつまれる。  
```rust
let s1 = String::from("hello");
let s2 = s1;
```
`String`型になると少し事情は変わる。`s1`ポインタ、長さ、許容量をもつ。ポインタは空のメモリを指す。
![Rust Memory Share](https://doc.rust-jp.rs/book-ja/img/trpl04-04.svg)  
`s1`を`s2`に代入すると`String`型のデータがコピーされる。スタックにあるポインタ、長さ、許容量をコピーし、ヒープ上のデータはコピーしない。ここにRustのバグの1つが隠されている。`s1`、`s2`がスコープを抜けたら、両方とも同じメモリを解放しようとする。これは**二重解放エラー**として知られ、メモリ安全性上のバグの一つになる。  
メモリ安全性を保証するためにこの場面で起こるバグがもう一つある  
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```  
確保されたメモリをコピーしようとする代わりに、コンパイラは`s1`が最早有効ではないと考え、`s1`がスコープを抜けた際に何も解放する必要がなくなる。`s2`の生成後に`s1`を使用しようとしたら動かなくなる。これは`s1`が`s2`に**ムーブ**されたと表現され、`s1`は無効に、`s2`のみ有効とみなされる。これは**shallow copy**(表面だけのコピー、中のデータは元と共有している)ににているが少し違うことがわかる(共有というより移動に近い)。Rustでは**deep copy**(データもコピーされる)が自動で行われることは絶対にない。  
### クローン  
仮にスタック上のデータだけでなく本当に`String`型のヒープデータのdeep copyが必要なら`clone`と呼ばれるメソッドを使うことが出来る  
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```
![Clone](https://doc.rust-jp.rs/book-ja/img/trpl04-03.svg)  
### スタックのみのデータ
```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
これはエラーが出ると思うかもしれないがしない。これは一見矛盾しているように見える。`clone`メソッドがないのに`x`は有効で`y`にムーブされていないように見れる。それは、整数のようなコンパイル時に既知のサイズを持つ型は、スタック上にすっぽり保持されるため、実際の値をコピーするのも高速だからである。言い換えると`x`を無効化する理由がなくなり、shallow coppyとdeep copyの違いがなくなる。  
* あらゆる整数型
* 論理値型
* 浮動小数点型
* 文字型
* タプル。`(i32,i32)`はokだが`(String,i32)`はダメ
これらはムーブでなく`Copy`  
### 所有権と関数  
```rust
fn main() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数の引数にムーブされる
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
```
`takes_ownership`の後に`s`を呼び出そうとすると、コンパイラはエラーを投げる。これは関数の引数に`s`がムーブされ、`s`が無効になったからである。それに対して、`x`は整数型であるためムーブではなくコピーされる。  
```rust
use std::io;
fn main() {
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                        // ムーブする

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
    println!("{}",s1);
    //println!("{}",s2);                //s2は無効
    println!("{}",s3);
    
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
```
値を返すことでも所有権は移動する。変数に値を代入されるとムーブされ、スコープを抜けると`drop`により片付けられる。
## 参照と借用  
```rust
use std::io;
fn main(){
  let s1=String::from("hello");

  let len=calculate_length(&s1); //参照を渡している

  println!("{},{}",s1,len);
}

fn calculate_length(s:&String)->usize{
  s.len()
}//sはスコープ外になるが、
//参照しているものの所有権を持っていないため何も起こらない
```
`calculate_length`は`String`型ではなく`&String`を受け取る。これによって関数内の`s`は所有権をもらうことなく、値を参照する。  
![借用](https://doc.rust-jp.rs/book-ja/img/trpl04-05.svg)  
変数`s`が有効なスコープはあらゆる関数の引数と同じでスコープ内に限るが、`s`は引数を参照しているだけで、参照が指しているものをドロップすることはない。所有権をもらわないため、所有権を返す目的で値を返す必要がない。  
このように引数に参照をとることを**借用**と呼ぶ。 
```rust
use std::io;
fn main() {
  let s = String::from("hello");
  let s1=&s; //sを参照
  let s2=&s; //sを参照
  let s3=&s1; //s1を参照
  let s4=&s3; //s3を参照

  // Stringの内部バッファの先頭アドレスを *mut u8 として取得
  let s_ptr = s.as_ptr();
  let s1_ptr=s1.as_ptr();
  let s2_ptr=s2.as_ptr();
  let s3_ptr=s3.as_ptr();
  let s4_ptr=s4.as_ptr();

  println!("pointer of s: {:p}", s_ptr);
  println!("pointer of s1: {:p}",s1_ptr);
  println!("pointer of s2: {:p}",s2_ptr);
  println!("pointer of s3: {:p}",s3_ptr);
  println!("pointer of s4: {:p}",s4_ptr);
}

```  
これは実際同じ生ポインタをとる。もし、借用した何かを変更しようとしたら、どうなるか。それはエラーになる。参照は不変である。 
```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```  
これはエラーになってしまう。  
## 可変な参照  
上のコードは少し変更を加えるだけでエラーを解決することが出来る。
```rust
use std::io;
fn main(){
  let mut s~String::from("hello");
  change(&mut s);
}

fn change(some_string:mut& String){
  some_string.push_str(",world");
}
```
まず`s`を`mut`に変えて、`&mut s`で可変な参照を生成する。そして`some_string:&mut String`で可変な参照を受け入れる必要がある。すると参照した値を変化させることが出来る。  
だが、可変な参照には大きな制約が一つある。それは、特定のスコープで、ある特定のデータに対しては、一つしか可変な参照を持てないことである。
```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```
たとえばこれはエラーになる。この制約は可変化を許可するものの、それを非常に統制の取れた形で行える。この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点である。データ競合とはこれら3つの振る舞いが起きるときに発生する。  
* 2つ以上のポインタが  
* 少なくとも1つのポインタがデータに書き込みを行っている  
* データへのアクセスを同期する機構が使用されない  

Rustはデータ競合が起こるコードをコンパイルさえしないため、この問題が発生しないようにしてくれる。スコープの外で**同時並行**でなく複数の可変な参照を作ることはできる。
```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

let r2 = &mut s;
```  
可変と不変の参照を組み合わせることは出来ない
```rust
let mut s = String::from("hello");

let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！
```
不変な参照をしている間は、可変な参照をすることは出来ない。不変参照の使用者は、それ以降に値が突然変わることなんて予想していない!
### 宙に浮いた参照
ポインタのある言語では、誤ってダングリングポインタを生成してしまいやすい。これは、他人に渡されてしまった可能性のあるメモリを指すポインタのことである、その箇所へのポインタを保持している間にメモリを解放してしまうことで発生する。Rustではダングリング参照が起こらないことを保証する。なんらかのデータへの参照があったら、コンパイラは参照がスコープを抜けるまで、データがスコープを抜けることがないように確認する。
```rust
fn main() {
    let reference_to_nothing = dangle(); //sを参照
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}//ここでsはスコープを抜けドロップされる。そのメモリは消される
//危険!!
```
これはエラーになる。`S`がスコープを抜けたことで無効な参照を返そうとするため。
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
この場合`s`はの所有権はムーブされ、何も解放されることはない。
## スライス型  
```rust
use std::io;
fn main(){
  let mut s=String::from("hello everyone! hello world!");
  let word=first_word(&s);

  println!("{}",word);
  s.clear(); //Stringを空にする。
  println!("{},{}",s,word); //sは空になり、wordは5のまま
}

fn first_word(s:&String)->usize{
  let bytes=s.as_bytes();
  for(i,&item)in bytes.iter().enumerate(){
    if item==b' '{
      return i
    }
  }
  s.len()
}
```
このコードは文字列の区切れ目の位置を表示するものである。`first_word`関数は`String`型の`s`を参照し、空白の位置を返す。`word`は`s`に最初に現れる空白の位置を持つ。その後、`s`を`clear()`で空にしても`word`は`5`のままで`0`にはならない。これは`word`と`s`が同期されていないことを表す。例えば、2つ目の区切れ目、3つ目の区切れ目、、、を保存したいとき管理が難しくなる。
### 文字列スライス
これを解決する方法がRustでは用意されている。それが**文字列スライス**である。
```rust
  let s=String::from("hello world");
  let hello=&s[0..5];
  let world=&s[6..s.len()];
```  
この方法で文字列を切り取ることが出来る。ただ、範囲外のスライスを指定するとエラーがでる。この方法で文から単語を抜き取る関数を書き直す  
![Slice](https://doc.rust-jp.rs/book-ja/img/trpl04-06.svg)
```rust
fn first_word1(s: &String)->&str{
  let bytes=s.as_bytes();
  for (i,&item) in bytes.iter().enumerate(){
    if item==b' '{
      return &s[0..i]
    }
  }
  &s[..]
}
```  
`first_word`関数は引数を参照して、空白の位置を返す。それによって空白の位置を保存することが出来るが、それが元のデータと同期をとれないというのがバグの元であった。しかし、この関数を使うと引数を参照して、空白を発見したら、文字列スライスを返す。これは元のデータと同期が取れている。
```rust
use std::io;
fn main(){
  let s=String::from("hello world");

  let k=first_word1(&s);
  s.clear(); //Error
  println!("{}",k);
}
```
しかし、このように`k`に`s`のスライスが保存されている状態で`s`が`clear()`されるとコンパイルエラーになる。
```

5 |   let k=first_word1(&s);
  |                     -- immutable borrow occurs here
6 |   s.clear(); //Error
  |   ^^^^^^^^^ mutable borrow occurs here
7 |   println!("{}",k);
  |                 - immutable borrow later used here
```  
これは不変参照`k=first_word1(&s)`が起こった後に、さらに可変な参照`s.clera()`が起こっているからである。借用規則から、何か不変な参照があるとき、さらに可変な参照を得ることはできないということに反している。  
###　文字列リテラルはスライスである  
`let s="Hello,world!"`ここでの`s`の型は`&str`である。これはバイナリの特定の位置を指すスライスである。`&str`は不変な参照であるため、文字列リテラルは不変である。
```rust
fn first_word(s: &str) -> &str {
    //...
}
```
このように引数を取ると`String`型も`&str`型も受け取ることが出来る。`String`型は文字列スライスとして引数に渡される。　　
整数型の配列もスライスできる
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```