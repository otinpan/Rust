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
### 文字列リテラルはスライスである  
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
## 構造体
### 構造体の定義とインスタンス化
```rust
use std::io;
struct User{
  //フィールド
  username:String,
  email:String,
  sign_in_count:u64,
  active:bool,
}

fn main(){
  //インスタンス化
  let user1=User{
    email:String::from("someone@example.com"),
    username:String::from("someusername123"),
    active:true,
    sign_in_count:1,
  };

  let mut user2=User{
    email:String::from("someone2@example.com"),
    username:String::from("someone2"),
    //構造体更新記法
    ..user1 //明示的にセットされていない残りのフィールドが
    //与えられたインスタンスのフィールドと同じ名なるようにする
  };
  user2.email=String::from("anotheremail@example.com");
  println!("user1name:{}",user1.username);
  println!("user2email:{}",user2.email);
}
```
構造体は`struct`で定義できる。構造値のデータ片の名前と型を**フィールド**に定義し、構造体をデータ片の集まりととらえることが出来る。インスタンスが可変であればフィールドに値を代入することで変更することが出来る。しかしインスタンス全体が可変でならなければならないことに注意する必要がある。一部のフィールドのみを可変にすることは出来ない。`User`構造体定義において`&str`文字列スライス型ではなく、所有権のある`String`型を使用した。これは構造体のインスタンスには全データを所有してもらう必要があり、このデータは構造体全体が有効な間はずっと有効である必要がある。(文字列スライス型は文字列データへの参照であり、自分ではデータを所有していない。データはどこか別の場所にあり、`&str`はその場所を指しているだけ)    　
### 構造体更新記法
**構造体更新記法**で他のインスタンスからインスタンスを生成することができる。明示的にセットされていない残りのフィールドを`.."インスタンス"`で与えられてたインスタンスのフィールドと同じ値になるように指定する。  
### 初期化省略記法  
```rust 
fn build_user(email:String,username:String){
  User{
    email,
    username,
    active:true,
    sign_in_count:1,
  }
}
```
このように関数の引数と、構造体のフィールドの値が同じ名前の場合`email:emai;,`とかく必要がなく、省略できる。
### タプル構造体
```rust
use std::io;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
  //インスタンス化
  let black=Color(0,0,0);
  let origin=Point(1,0,0);

  println!("{}",black.0);
  println!("{}",origin.0);
}
```
構造体名により追加の意味を含むものの、フィールドに紐づけられた名前がなく、フィールドの型だけの**タプル構造体**と呼ばれる、タプルに似た構造体を定義することが出来る。構造体のフィールドが同じ型であても、それ自身が独自の型になる。そのため`Color`と`Point`は別のものであり、`Color`型を引数に取る関数は`Point`を引数に取ることはできない  
### ユニット様構造体
フィールドのない構造体をユニット様構造体と呼ぶ。
### 構造体を使ったプログラムの例  
```rust
use std::io;
struct Rect{
  width:u32,
  height:u32,
}

fn main(){
  let rect1=Rect{width:30,height:50};

  println!("{}",get_area(&rect1));
}

fn get_area(rect:&Rect)->u32{
  rect.width*rect.height
}
```  
例えばこれは長方形の構造体と、その面積を求める関数である。構造体のフィールドを作ることによって関数の意図を示すようになった。関数はインスタンスへの不変借用をとり、所有権は映っていない。  
### トレイトの導出で有用な機能を追加する  
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1は{}です
    println!("rect1 is {}", rect1);
}
```   
このようにインスタンス`rect1`を出力しようとするとエラーになる。
```
error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
(エラー: トレイト境界`Rectangle: std::fmt::Display`が満たされていません)
```
`println!`には様々な整形があり、標準では波括弧は`Display`として知られる整形をするように`println!`に指示をする。  
```
`Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
(注釈: `Rectangle`は、デフォルト整形機では、整形できません; フォーマット文字列を使うのなら
代わりに`:?`を試してみてください)
```
ただ構造体には`Display`が実装されていないため`println!`に別の出力整形を使いたいという指示をする必要がある。その出力整形が`Debug`である。これを指示するために`println!("rect1 is {:?},rect1)`と書き換える。ただ、それでもエラーになる  
```
error[E0277]: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
(エラー: トレイト境界`Rectangle: std::fmt::Debug`が満たされていません)
```  
```
`Rectangle` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
(注釈: `Rectangle`は`:?`を使って整形できません; 自分のクレートで定義しているのなら
`#[derive(Debug)]`を追加するか、手動で実装してください)
```
確かにRustにはデバッグ用の情報を出力する機能が備わっているが、この機能を構造体で使えるようにするには、明示的な選択をする必要がある。構造体の直前に`#[derive(Debug)]`という注釈を追加する。
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```
```rect1 is Rectangle {
    width: 30,
    height: 50
}
```  
## メソッド記法
メソッドとは型に基づいた関数であり`."関数"()`のようにドットで呼び出される関数である。`"String"型.len()`もメソッド記法の一つである。
```rust
use std::io;
#[derive(Debug)]
struct Rectangle{
  width:u32,
  height:u32,
}

impl  Rectangle{
  fn area(&self)->u32{
    self.width*self.height
  }
}

fn main(){
  let rect1=Rectangle{width:30,height:50};

  println!{"{}",rect1.area()};
}
```  
構造体の文脈内で関数を定義するには`imple (構造体)`で始める。それから`area`関数を作成するが、引数に`&self`とすることで`Rectangle`型の借用をする。もし`&`をつかない`self`の場合は所有権を奪うメソッドになる。このテクニックは通常、メソッドが`self`を何か別のものに変形し、変形後に呼び出し元が元のインスタンスを使用できないようにしたい場合に使用される。またメソッド内で変更をしたい場合は引数に`&mut self`をとることで変更を受け付けることが出来る。  
### `->`演算子は？  
CとC++では、メソッド呼び出しには2種類の異なる演算子が使用される。オブジェクトに対して直接メソッドを呼び出すなら`.`を使用するし、オブジェクトのポインタに対してメソッドを呼び出し、先にポインタを参照外しする必要があるなら`->`を使用する。`object`がポインタなら、`object->something()`と`(*object).something()`は同等である。  
Rustには`->`演算子の代わりはない。ただ**自動参照および参照外し**という機能がある。`object.something()`メソッドを呼び出すと、コンパイラは`object`メソッドのシグニチャと合致するように、自動で`&`か`&mut`、`*`を付与する。
```
p1.distance(&p2);
(&p1).disance(&p2);
```
は同じもの。受け手とメソッド名が与えられればコンパイラは確実にメソッドが読み込み専用`(&self)`か、所有権を奪う`(self)`のか、書き込みもする`(&mut self)`のかを判断する。  
### 引数が複数のメソッド  
```rust
use std::io;
#[derive(Debug)]
struct Rectangle{
  width:u32,
  height:u32,
}

impl  Rectangle{
  fn can_hold(&self,other:&Rectangle)->bool{
    self.width>other.width&&self.height>other.height
  }

  fn compare_area(&self,other:&Square)->bool{
    self.width*self.height>other.length*other.length
  }
}

#[derive(Debug)]
struct Square{
  length:u32,
}

fn main(){
  let mut rect1=Rectangle{width:30,height:50};
  let mut rect2=Rectangle{width:40,height:60};
  let mut square1=Square{length:40};
  println!("{}",rect1.can_hold(&rect2));
  println!("{}",rect1.compare_area(&square1));
}
```
別のインスタンスを不変借用したい場合は第2引数に`other:&(構造体名)`と書く。  
### 関連関数    
`impl`ブロックではブロック内に`self`を引数に取らない関数を定義できる。これは構造体に関連付けられているため**関連関数**と呼ばれる。関連関数は関数であり、メソッドではない。関連関数は、構造体の新規インスタンスを返すコンストラクタに良く使用される。たとえば長さと幅両方を同じ長さの`Rectanle`を生成したい場合は
```rust
impl Rectangle{
  fn square(size:u32)->Rectangle{
    Rectangle{width:size,height:size}
  }
}
fn main(){
  let sq=Rectangle::square(3);
}
```  
と記述する。関連関数を呼び出すために`::`記法を使用する。
## 列挙型
列挙子とは「列挙型の中で定義される個々の定数」のことである。例えばIPアドレスを扱う必要が出たとする。現在IPアドレスの規格はバージョン4とバージョン6の2つがある。どんなIPアドレスも、バージョン4かバージョン6のどちらかになるが、同時に両方になることはない。このように、いくつかの成分の中からどれかの状態である、といった場合はenumデータ構造が適切なものになる。  
```rust
use std::io;
enum IpAddrKind{
  V4,  //列挙子
  V6,
}
fn main(){
  let four=IpAddrKind::V4;  //インスタンス
  let six=IpAddrKind::V6;
}
fn route(ip_type:IpAddrKind){
  
}
```  
これで`IpAddrKind`はコードの他の場所で使用できる独自のデータ型になる。enumの列挙子は関数の引数としても使える。現状では実際のIPアドレスのデータを保存する方法はない。そこでenumの列挙子に直接データを格納して、IPアドレスを保存する。  
実際IPアドレスを格納してその種類をコード化したくなるということは一般的であり、Rustの標準ライブラリに使用可能な定義が存在する。`IpAddr`の定義のされ方を見てみる。  
```rust
struct Ipv4Addr {
    // 省略
}

struct Ipv6Addr {
    // 省略
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```  
このように構造体を列挙子に埋め込むことも可能。  
```rust
use std::io;
enum Message{
  Quit,
  Move{x:i32,y:i32},
  Write(String),
  ChangeColor(i32,i32,i32),
}

impl Message{
  fn call(&self){
    //code
  }
}

fn main(){
  let m=Message::Write(String::from("hello"));
  m.call();
}
```
また、enumは構造体のように`impl`をつかって構造あいにメソッドを定義できるのと同様にメソッドを定義することが出来る。  
### Optin  
`Option`も昇順ライブラリにより定義されているenumである。`Option`はいろんな箇所で使用される。なぜなら、値が何か、そうでないかという非常に一般的な筋書きをコード化するからである。Rustには**null**が存在しない。nullがある言語において、変数は常に二者択一のどちらかになる。nullかそうでないかである。nullの問題は、nullの値をnullでない値のように使用しようとしたら、何らかのエラーが出ることである。それでもnullは役に立つ。Rustではnullがない代わりに、標準ライブラリに値が存在するか不在かという概念をコード化するenumがある。このenumが`Option<T>`で以下のように標準ライブラリに定義されている。
```rust
enum Option<T>{
  Some(T),
  None,
}
```  
`Option<T>`は有益すぎて、初期化処理にさえ含まれている。つまり、明示的にスコープに導入する必要がない。さらに`Some`と`None`を`Option::`の接頭辞なしに直接使える。`<T>`という記法はジェネリック型引数であり、あらゆる型のデータを1つだけ持つことが出来る。これは`Option`値をつかって数値型や文字列型を保持する例である。
```rust
use std::io;
fn main(){
  let some_number=Some(5);
  let some_string=Some("a string");
  let potential_number:Option<i32>=Some(3);
  let absent_number:Option<i32>=None;
  absent_number=Some(3);
}
```  
`None`をつかったらコンパイラに`Option<T>`の型が何になるかを教えなければならない。というのも、`None`を見ただけでは`Some`列挙子が保存する型をコンパイラが推論できないからである。`Some`があるとき、値が存在するとわかる。では、なぜ`Option<T>`がnullよりも好ましいと言えるのか。それは`Option<T>`と`T`は異なる型であるため、コンパイラが`Option<T>`値を確実に有効な値かのようには使用させてくれないからである。例えば
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
これはエラーになる。このように、プログラマーはnullの可能性がある値を使用する前に、nullであることをチェックする必要はない。プログラマーは`Option<T>`があるときのみ、値を保持していない可能性を考慮する必要があるわけで、コンパイラが確かめてくれる。
## match制御フロー演算子  
Rustには一連のパターンに対して値を比較し、マッチしたパターンに応じてコードを実行させてくれる`match`という制御フロー演算子がある。例えばコインの価値をセントで返す関数を実装する
```rust
use std::io;
enum Coin{
  Penny,
  Nickel,
  Dime,
  Quater,
}
fn main(){
  let coin=Coin::Penny;
  println!("{}",value_in_cents(coin));
}

fn value_in_cents(coin:Coin)->u32{
  match coin{
    Coin::Penny=>{
      println!("Lucky penny!");
      1
    },
    Coin::Nickel=>5,
    Coin::Dime=>10,
    Coin::Quater=>25,
    //一つでも列挙子がかけているとエラーになる
  }
}
```
`match`式が実行されると、結果の値を各アームのパターンと順番に比較する。パターンに値がマッチしたら、そのコードに紐づけられたコードが実行される。
```rust
#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```
このような列挙型があったとする。
```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```
このように列挙子が持つデータを`match`式で扱うことが出来る。`value_in_cents(Coin::Quater(UsState::Alaska))`というように呼び出すことが出来る。    
### `Option<T>`とのマッチ  
`match`を使って`Option<T>`を扱うこともできる。基本的に`match`式の動作の使用は同じ。`Optino<i32>`を取る関数を描きたくなったとし、中に値があったら、その値に1を加えることにする。
```rust
use std::io;
fn main(){
  let five=Some(5);
  let six=plus_one(five);
  let none=plus_one(None);
}

fn plus_one(x:Option<i32>)->Option<i32>{
  match x{
    None=>None,
    Some(i)=>Some(i+1),
  }
}
```
値が`None`のときはなにもしないから`None`を返す。`Some`のときは`i`は`Some`に含まれる値に束縛され、`i`の値に1が足され新しい`Some`を生成する。`match`は包括的でなければならないため`None`、`Some`どれか一つでもアームが抜けているとエラーになる。Rustではすべてのあらゆる可能性を網羅しつくさなければ、コードは有効にならない。すべての可能性を列挙したくないときは`_`のパターンを使用できる。
```rust
use std::io;
fn main(){
  let k=2;
  match k{
    1=>println!("one"),
    3=>println!("three"),
    _=>(),
  }
}
``` 
## パッケージとクレート
クレートはバイナリかライブラリのどちらかで、**クレートルート**とは、Rustコンパイラの開始点となり、クレートのルートモジュールをつくるソースファイルのこと。**パッケージ**はある機能群を提供する1つ以上のクレートである。パッケージは**Cargo.toml**という、それらのクレートをどのようにビルドするかを説明するファイルを持っている。  
パッケージが何を持ってよいかはいくつかのルールで決まっている。まず、パッケージは0個か、1個のライブラリクレートを持っていないといけない。2個以上はだめ。バイナリクレートはいくつ持っても良い。パッケージを作ると  
```rust
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```  
Cargoは`Cargo.toml`ファイルを作り、`src/main.rs`をつくる。`Cargo.toml`は特に`src/main.rs`については何も書かない。これはCragoは`src/main.rs`がパッケージと同じ名前を持つバイナリクレートのクレートルートであるという習慣に従ているためである。同じようにCargoはパッケージディレクトリに`src/librs`が含まれていたら、パッケージにはパッケージと同じ名前のライブラリクレートが含まれており、`src/lib.rs`がそのクレートルートなのだと判断する。Cargoはクレートルートファイルを`rstc`に渡し、ライブラリやバイナリをビルドする。
```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── src/bin/
    ├── tool1.rs   ← バイナリクレート1
    └── tool2.rs   ← バイナリクレート2
```
このようにパッケージはバイナリクレートは複数持つことが出来るが、ライブラリライブラリクレートは0か1つである。パッケージを作った直後ではライブラリクレートなく、バイナリクレートは`src/main.rs`の1つである。  
例えば`rand`ライブラリを外部から呼び出したとする。クレートの機能をそれ自身のスコープの中に入れておくことで、ある機能が自分のクレートで定義されたのか`rand`クレートで定義されたのかを明確にし、名前の衝突を予防してくれる。例えば自分のクレートで`Rng`という名前の`struct`を定義することもできる。`rand`を依存先としてつかしても、コンパイラは`Rng`という名前が何を意味するかについて混乱することはない。`rand`クレートの`Rng`トレイとには`rand::Rng`でアクセスするということである。  
## モジュールシステム  
**モジュール**はクレート内のコードをグループ化し、可読性と再利用性を上げるのに役に立つ。また、モジュールは**public**と**private**を使い分けることで**プライバシー制御**も行える。  
``restaurant`という新しい名前のライブラリを`cargo new --lib restaurant`と実行することで作成し、**`src/lib.rs`**に書き込む、モジュールと関数のシグネチャを定義する。
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```  
モジュールは`mod`キーワードを書き、次にモジュールの名前を指定することで定義される。モジュールの中には他のモジュール、構造体、enum、定数、トレイと、関数を置くことが出来る。  
**`src/main.rs`**と**`src/lib.rs`**はクレートルート**と呼ばれている。この名前の訳は、**モジュールツリー**と呼ばれるクレートのモジュール構造の根っこにこれら2つのファイルの中身が`crate`というモジュールを形成するからである。
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
### モジュールツリーの要素を示すためのパス  
パスは2つの形を取ることが出来る
* **絶対パス**はクレートの名前か`crate`という文字列を使うことで、クレートルートからスタートする。
* **相対パス**は`self`、`super`または今のモジュール内の識別子を使うことで、現在のモジュールからスタートする。
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant(){
    //絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    //相対パス
    front_of_house::hosting::add_to_waitlist();
}
```
絶対パスで指定する場合は`crate`キーワードで絶対パスを始めることが出来る。これはファイルシステムに置き換えると`/front_of_house/hosting/add_to_waitlist`にに相当する。`/`は`crate`に対応する。  
相対パスの場合は`eat_at_restaurant()`はと同じ階層で定義されているモジュールである`front_of_house`からスタートする。ファイルシステムにおける`front_of_house/hosting/add_to_waitlist`に相当する。  
相対パスか、絶対パスを使うかはプロジェクトによって決める。例えば、`front_of_house`モジュールと`eat_at_restaurant`関数を`customer_experience`というモジュールに移動させると、`add_to_waitlist`への絶対パスを更新しないといけないが、相対パスはそのままである。しかし、`eat_at_restaurant`関数だけを`dining`というモジュールに移動察せると、`add_to_waitlist`への絶対パスは同じままだが相対パスは更新しないといけない。  
実際上のコードをコンパイルしてみると、エラーになる。メッセージによると`hosting`は非公開だと言っている。Rustにおけるプライバシーは「あらゆる要素は標準では非公開」というやり方で動いている。親モジュールの要素は子モジュールの非公開要素を使えないが、子モジュールの要素は祖先モジュールの要素を使える。
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}
```
ではこれではどうか。これでもエラーになる。`hosting`は公開されたが、中身はまだ非公開。モジュールに`pub`キーワードがついていても、先祖モジュールのコードはモジュールを参照できるようになるだけである。
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```
このように関数にも`pub`をつけることでモジュールの外から呼び出すことが出来る。
### 相対パスをsuperで始める
靄モジュールから始まる相対パスなら、`super`を最初につけることで交際できる。ファイルシステムの`..`に似ている。
```rust
fn serve_order(){}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}

    mod cookware{
        fn get_cookware(){
            super::super::serve_order();
        }
    }
}
```
将来このコードが別のモジュールに移動するとしても更新する場所が少なくて済む。
### 構造体とenumを公開する
構造体とenumにおいても`pub`を使って公開するよう指定できるが、追加の細目がある。
```rust
mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal=back_of_house::Breakfast::summer("Rye");

    meal.toast=String::from("Wheat");
    println!("{}",meal.toast);
}
```
構造体を`pub`にするだけではフィールドは公開されない。それぞれのフィールドで`pub`を設定する必要がある。  
enumの場合公開するとそのヴァリアントはすべて公開される。
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
### useキーワードでパスをスコープに持ち込む  
これまで関数呼び出しのために書いてきたパスは、長く、繰り返しも多くて不便なものだった。例えば絶対パス、相対パスを使うかに関わらず、`add_to_waitlist`関数を予防と思うたびに`front_of_house`と`hosting`も指定しないといけない。これを簡単化する方法が`use`キーワードである。これはパスをスコープに持ち込み、それ以降はパス内の要素がローカルにあるかのように呼び出すことができる。
```rust
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            ////
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```  
`use`と相対パスで要素をスコープに持ち込むこともできる。その場合は`use self::front_of_house::hosting`と書き換える。では`hosting`も省略したい場合、`use crate::front_of_house::hosting::add_to_waitlist;`にすればいいと思うだろう。これは慣例的ではない。このように書いたとき、この関数がどこで定義されたのかが不明瞭である。ただ構造体やenum、その他の要素を`use`で持ち込むときは古パスを描くのが慣例的である。
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
### エイリアス  
Rustでは同じ2つの要素を`use`でスコープに持ち込むことはゆるされていない。そのとき、上記の慣例は例外的に不可能。
```rust
use std::fmt;
use std::io;

fn function1()->fmt::Result{

}

fn function2()->io::Result<()>{
    
}
```  
このように、親モジュールを使うことで2つの`Result`型を区別できる。もし`use std::fmt::Result`と`use std::io::Result`と書いていたら、２つの`Result`型が同じスコープに存在することになり、私たちが`Result`を使ったときにどちらのことを意味しているかRustは分からなくなってしまう。同じ2つの型を`use`を使って同じスコープに持ち込むという問題には、もう1つ解決策がある。パスの後に、`as`と型の新しいローカル名、すなわちエイリアスを指定すればよい。
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### pub use  
`pub use`はRustにおいてモジュールやクレート内の項目を外部に再公開するための構文である。  
`pub use`の目的
1. 内部モジュール構造を隠しつつ、必要なものだけ外部に公開したい
2. APIを整理して使いやすくしたい
3. 複数のモジュールにまたがるものをまとめたい  

```rust
mod a{
    pub fn hello(){
        println!("Hello from a");
    }
}

pub use a::hello;
```  
```rust
use restaurant::hello;
fn main(){
    hello();
}
```
またクレートの中身を一括で再公開することもできる
```rust
// lib.rs
pub use crate::module_a::*;
pub use crate::module_b::*;
```
`use`ｈスコープ内で使えるようになるだけ。`pub use`は他のモジュールや外部からも使えるようになる。
### 外部のパッケージを使う  
外部のパッケージを使う場合、**Caargo.toml**にパッケージ情報を持ち込む必要がある。また`use`をつかって要素をクレートからスコープへ持ち込めばよい。標準ライブラリ(`std`)に限ってはCargo.tomlに変更を加える必要はない。
### 巨大なuseのリストをネストしたパスを使って整理する
同じクレートか同じモジュールで定義された複数の要素を使おうとするとき、それぞれの要素を一行一行並べると、縦に大量のスペースを取ってしまう。
```rust
// --snip--
// （略）
use std::cmp::Ordering;
use std::io;
// --snip--
// （略）
```
ネストしたパスを使うことで、同じ一連の要素を1行でスコープに持ち込める。
```rust
// --snip--
// （略）
use std::{cmp::Ordering, io};
// --snip--
// （略）
```  
```rust
use std::io;
use std::io::Write;
```
この場合2つのパスの共通部分は`std::io`である。この場合は
```rust
use std::io::{self, Write};
```
と`self`を使えばよい。
### glob演算子
パスにおいて定義されているすべての公開要素をスコープに持ち込みたいときは`glob`演算子`*`をパスの後ろに続けてかく。
```rust
use std::collections::*;
```
## モジュールを複数のファイルに分割する  
今までは複数のモジュールを一つのファイルに定義していた。モジュールが大きくなるとき、コードを読みやすくするため、これらの定義を別のファイルへ移動させたくなるかもしれない。例えば`src/front_of_house.rs`に`hosting`モジュールが定義されているとする。クレートルートファイルは`src/lib.rs`であるため、ここにまとめる必要がある。これは`src/main.rs`でも構わない。
```
restaurant/
├── Cargo.toml
└── src/
    ├── main.rs         // eat_at_restaurant を呼び出す
    ├── lib.rs          // ライブラリの定義
    └── front_of_house  //hostingモジュールの定義
```
`front_of_house`    
```rust
pub mod hosting{
    pub fn add_to_waitlist(){
        println!("Hello");
    }
}
```
`lib.rs`  
```rust
mod front_of_house;

pub use crate::front_of_house::hosting; //pub出なくても通る

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}
```  
main.rs
```rust
use restaurant::eat_at_restaurant;
fn main(){
    eat_at_restaurant();
}
```  
`lib.rs`のように`mod front_of_house`の後にブロックではなくセミコロンを使うと、Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令する。
```
restaurant/
├── Cargo.toml
└── src/
    ├── main.rs         // eat_at_restaurant を呼び出す
    ├── lib.rs          // ライブラリの定義
    └── front_of_house  //hostingモジュールの定義 
        ├── hosting.rs  //pub fn add_to_waitlist(){...}
```
このようにファイル自体をモジュールとして扱うこともできる。
## ベクタ
`Vec<T>`は**ベクタ**とよばれ、単体のデータに複数の値を保持する。ベクタでは値をメモリ上に隣り合わせに並べる。またそれらの値は同じ型である必要がある。
### ベクタの生成
```rust
let v:Vec<i32> =Vec::new()
```
これによって`i32`型の空のベクタを生成する。型注釈をつけないとコンパイラは私たちがどんなデータを保持するか推測できない。もし値を初期化時に挿入する場合は型はコンパイラによって推測される
```rust
let v=vec![1.2,3.1,4.1];  //f64型
```
### ベクタの更新
配列とは違い、ベクタは長さを変更することが出来る。ただ、値を変更したかったり、長さを変化させたい場合は`mut`をつける必要がある。また、型を初期値と同じようにする必要がある  
```rust
  let mut v=vec![1,2,3];
  v.push(4);
  v.push(5);
  v.push(6);
```  
ベクタは変数と同じようにスコープを抜けるとドロップされる。ベクタのすべての要素が解放される。
### ベクタの要素を読む
```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        //                      "3つ目の要素は{}です"
        Some(third) => println!("The third element is {}", third),
        //               "3つ目の要素はありません。"
        None => println!("There is no third element."),
    }
```
ベクタの要素を読む方法は主に2つある
1. 参照を得る`let third:&i32=&v[2]`
2. `get`メソッドに引数として添え字を渡して`Option<T>`を得る  

1の場合参照を得るため、所有権は移動しない。2の場合も引数に参照(`&i32`)を渡しているため所有権は移動しない。`Option<&T>`で得るため、ベクタの長さを越えた要素にアクセスしようとしても`None`を得るだけである。もし参照を使用した場合はプログラムはクラッシュする。  
1.2の方法は要素を変数から参照するため参照規則は適用される
* ある特定のデータに対しては1つしか可変な参照を持てない(一人で編集)  
* 可変と不変の参照を組み合わせることが出来ない(読んでいる間に勝手に変えられる!危ない!)  
* 不変な参照をしている間は可変な参照をすることはできない(みんなで読むだけ)  
* 参照をしている間は元の変数に直接アクセスしてはいけない(編集するから元のもの持ってくな)  

### 可変な参照
```rust
fn main(){
  let mut v=vec![1,2,3,4,5];
  let first=&mut v[0];

  *first=2;
  println!("{}",v[0]);
}
```  
ベクタでも可変な参照はできる。`first`は可変な参照で、`v`の0番目のアドレスを示している(?)。では1つのベクタに2つの可変な参照変数が存在するときどのようになるでしょう。
```rust
  let mut v=vec![1,2,3,4,5];
  let first=&mut v[0];
  let second=&mut v[2];

  *first=2;
  *second=10;
```
これはエラーになる。一つのデータ(ベクタ)に対して2つの可変な参照は規則に反している。では次はどうだろう
```rust
  let mut v=vec![1,2,3,4,5];
  let first=&mut v[0];
  *first=0;
  v.push(6);
```
これは通る。
1. `first`は`v`のアドレスをもつ
2. `first`の参照先のデータが変更される
3. もとのデータ`v`が変更される  

これは何も問題がない。では少し変更を加える。
```rust
  let mut v=vec![1,2,3,4,5];
  let first=&mut v[0];
  v.push(6);
  *first=0;
```
これはエラーになる。
1. `first`は`v`のアドレスをもつ
2. もとのデータ`v`が変更される
3. `first`は参照先のデータを変更する
なぜエラーになるのだろうか。それは元のデータ`v`が変更されることによって、データを保存するスペースがなくなり、新しいスペースに移動する可能性があるからである。もしそうなった場合、`first`は解放されたメモリを指すことになる。そのようなことを未然に防ぐため借用規則が設けられている。  
### ベクタ内の値を順番に処理する  
```rust
  for i in &v{
    println!("{}",i);
  }

  for i in &mut v{
    *i+=50;
  }
```
ベクタ内の値を順に見ていく場合は、不変な参照を得る。ベクタ内の値を変化させたい場合は可変な参照をとり、参照外し演算子(`*`)を使用して、`i`の値にたどり着く必要がある。

### EnumとVector  
```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```
ベクタは同じ型の値しか保持できない。ただ、enumを用いればenum型をつかって整数型、浮動小数点、文字列、構造体などを組み合わせることが出来る。

## 文字列
文字列は思っているよりも複雑なデータ構造である。「文字列」とは一般的に`String`と文字列スライスの`&str`のことを意味する。ただ言語の核としては1種類しか文字列型は存在しない。文字列スライスの`&str`は、別の場所に格納されたUTF-8エンコードされた文字列データへの参照である。`String`型はRustの標準ライブラリで提供され、伸長可能、可変、所有権のあるUTF-8エンコードされた文字列型である。
### 文字列の生成
空の文字列を生成したい場合は`new`関数から始める。
```rust
let mut s=String::new();
```
文字列リテラルを`String`型にするには`String::from()`と`.to_string()`を使うことが出来る。適切にエンコードされた文字列なら、どんなものでも有効である。
### 文字列の更新
`push_str`を使うことで`String`を伸ばすことが出来る。
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```
これはエラーになるだろうか。これはならない。`push_str()`では引数の参照をとり、所有権までは奪わない。`s2`は有効なままである。`push()`は文字列ではなく、文字型を追加するときに有効である。`String`型の2つの変数を合体させたいときに簡単な記法として+演算子を使うことがある。
```rust
  let s1=String::from("I ");
  let s2=String::from("have ");
  let s3 =String::from("a pen.");

  let s=s1+&s2+&s3;
  println!("{}",s);
  //println!("{}",s1); //所有権は奪われている!
  println!("{}",s2);
```
なぜ第2引数に参照を指定する必要があるのか。それは+演算子を使用したときに呼び出される`add`メソッドに関係がある。
```rust
fn add(self, s: &str) -> String {
```
`add`メソッドでは第二引数に文字列スライス型を指定する。コンパイラは`&String`型を`&str`に**型強制**してくれる。  
もし文字列を連結して表示させたいときは`format!()`が有効である。
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```
### 文字列の添え字にアクセスする
```rust
let s1 = String::from("hello");
let h = s1[0];
```
これはエラーになる。ではなぜでしょう。  
まずUTF-8の要点は、Rustの文字列を見るには3つの関連した文法があることである。バイト、スカラー地、書記素クラスタである。ヒンディー語の単語“नमस्ते”を表記する。
バイトとしては
```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```
スカラー値としては
```
['न', 'म', 'स', '्', 'त', 'े']
```
書記素クラスタとしては
```
["न", "म", "स्", "ते"]
```
である。このように1文字が1バイトとは限らないため`s1[0]`のように「0番目のバイトが1文字の意味を持つ」と保証できないのである。　　
文字列から1文字取り出したいときは以下のように書く
```rust
let s1 = String::from("hello");
let h = s1.chars().nth(0);   //Option<chat>になる
if let Some(c) = s1.chars().nth(0) {
    println!("First char: {}", c);
}
```

## ハッシュマップ  
`HashMap<k,V>`は`K`型のキーと`V`型の値の対応関係を保持する。例えば、ゲームにおいて各チームのスコアをハッシュマップで追いかけることができる。
### ハッシュマップの生成
```rust
use std::io;
use std::collections::HashMap;
fn main(){
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"),10);
  scores.insert(String::from("Yellow"),50);

  println!("{}",scores["Blue"]);
}
```
まず標準ライブラリのコレクション部分から`HashMap`を`use`する必要がある。ベクタと同様にハッシュマップはデータをヒープに保持する。この`HashMap`はキーが`String`型、値が`i32`型ですべて同じ型でなければならない。べつの方法もある。それはメソッドを使うことである。
```rust
use std::io;
use std::collections::HashMap;
fn main(){
  let teams=vec!['i','j','k'];
  let scores=vec![10,30,20];

  let res:HashMap<_,_>=teams.iter().zip(scores.iter()).collect();

  println!("{}",res[&'i']);
}
```  
別々のベクタに対して`zip`メソッドでタプルのベクタを作り上げる。それから`collect`メソッドをつかって、タプルのベクタをハッシュマップに変換する。
### ハッシュマップと所有権  
```rust
　let mut map=HashMap::new();
  let name='i';
  let score=10;

  map.insert(name,score);
```
この場合、`name`と`score`は有効なのだろうか。有効でない。データの所有権っは`map`に映ったからである。
### ハッシュマップの値にアクセスする
```rust
use std::io;
use std::collections::HashMap;
fn main(){
  let mut map=HashMap::new();
  map.insert(String::from("Blue"),10);
  map.insert(String::from("Red"),5);

  let team=String::from("Blue");
  let score=map.get(&team); //Option<T>

  for(key,value) in &map{
    println!("{},{}",key,value);
  }
}
```
`HashMap`のデータにアクセスする方法の1つは`get`メソッドを使うことである。これはベクタで扱った時と同じで`Option<&T>`型を返す。そのため、実際の値を表示したい場合は`if let`を使う。別の方法は`for`でキーと値を同時に出すことである。
### 値の更新
ハッシュマップ内のデータを変えたいときは、すでにキーに値が紐づいている場合の扱い方を決めなければならない。値の変え方はいくつか方法がある  

1. 値を上書きする
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```
シンプルにかける。  

2. キーに値がなかったときのみ値を挿入する
```rust
use std::io;
use std::collections::HashMap;
fn main(){
  let mut map=HashMap::new();
  map.insert(String::from("Blue"),10);

  map.entry(String::from("Red")).or_insert(50);
  map.entry(String::from("Blue")).or_insert(30);

  if !(map.contains_key("Yellow")){
    map.insert(String::from("Yellow"),10);
  }
  for x in &map{
    println!("{:?}",x);
  }
}
```
`entry`で引数としてチェックしたいキーを取る。このメソッドの元理知は`Entry`と呼ばれるenumであり、これは存在したりしなかったりする可能性のある値を表す。`or_insert`は対応する`Entry`キーが存在したときにそのキーに対する値への可変参照を返すために定義されており、もしなかったら、引数をこのキーの新しい値として挿入し、新しい値への可変参照を返す。  

3. 古い値に基づいて値を更新する
```rust
use std::io;
use std::collections::HashMap;
fn main(){
  let text="hello world wonderful world";
  let mut map=HashMap::new();

  for world in text.split_whitespace(){
    let count=map.entry(world).or_insert(0);
    *count+=1;
  }

  println!("{:?}",map);
}
```
`or_insert`では値の可変参照を返す。その可変参照を`count`変数に保持しているため、その値に代入するには`*`で参照外しをしなければならない。  
`HashMap`暗号学的に安全なハッシュ関数を使用するため、最速ではない。ただ、パフォーマンスの低下と引き換えに安全性を得るというトレードオフは価値がある。

## エラー処理
Rsutへの信頼性の傾倒はエラー処理にも及ぶ。コンパイラはプログラマにエラーの可能性を知り、コードのコンパイルが通るまでに何かしら対応を行うことを要求してくる。  
Rustでは大きく2つにエラーは分類される。**回復可能**と**回復不能**なエラーである。回復可能エラーとは、適切な処理を行えば、プログラムを継続することができるエラーである。例えばファイルが見つからなかったりするのは回復可能エラーである。回復不能エラーはメモリ不足、論理的にあり得ない状態、ヌルポインタ参照のような、対処不能であり、エラーが発生した時点でプログラムの事項を中断せざるを得ないエラーである。多くの言語では、2種のエラーを区別することはないが、Rustではこの2種のエラーを区別する。回復可能なエラーには`Result<T,E>`値があり、回復不能なエラーには`panic!`マクロがある。  
## `panic!`で回復不能なエラー
`panic!`マクロが実行されるとプログラムは失敗のメッセージを表示し、スタックを巻き戻し掃除して、終了する。これが最もありふれて起こるのは、何かしらのバグが検出されたときであり、プログラマには、動エラーを処理すればよいか明確ではない。  
**パニックに対してスタックを巻き戻すか異常終了する**  
標準ではパニックが発生すると、プログラムは**巻き戻しを始める**。つまり、言語がスタックをさかのぼり、遭遇した各関数のデータを片付けるということ。しかし、このさかのぼりと片付けはすべきことが多くなる。対立案は、即座に異常終了して、片付けをせずにプログラムを終了させることである。そうなると、プログラムが使用していたメモリは、OSが片付ける必要がある。プロジェクトにおいて、実行可能ファイルを極力小さくする必要があらばCargo.tomlファイルの適切な[`profile`]欄に`panic='abort'`と追記することで、パニック時に巻き戻しかあ異常終了に切り替えることが出来る。  
```rust
fn main() {
    panic!("crash and burn");  //クラッシュして炎上
}
```
実際に`panic!`マクロを使ってみる。すると
```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:4
('main'スレッドはsrc/main.rs:2:4の「クラッシュして炎上」でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
このようなエラーがdる。どこの行で`panic!`マクロが呼び出されているかを見ることが出来る。`panic!`バックトレースを使用すると問題を起こしているぞ分のコードの箇所を割り出すことが出来る。
### `panic!`バックトレースを使用する  
```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```
ベクタは3つしか要素が存在しないが、100番目の要素にアクセスしようとする。ほかの言語では、この場面でほしいものではないのにも関わらず、まさしく要求したもその返そうとしてくる。メモリがベクタに属して否に野にもかかわらず、ベクタ内のその要素に対応するメモリ上の箇所にあるものを何か返してくる。これは、**バッファー外読み出し**と呼ばれる。これを実行してみると
```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
('main'スレッドは、/checkout/src/liballoc/vec.rs:1555:10の
「境界外番号: 長さは3なのに、添え字は99です」でパニックしました)
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
とエラーが出る。バックトレースではパニックに至るまでに呼び出された関数すべての一覧を表示する。バックトレースを表示させたい場合は、`RUST_BACKTRACE=1 cargo run`とコマンドする。`panic!`が呼び出されているところから遡ることで、パニックを起こしている根源を割り出すことが出来る。

## `Result`で回復可能なエラー
多くのエラーは、プログラムを完全に
ストップさせるほど深刻ではない。例えば、ファイルを開こうとして、ファイルが存在しないために処理が失敗したら、プロセスを停止するのではなく、ファイルを作成したいことがある。この場合、`Result`型が有効である。
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
`Result`型はこのように定義される。例えばファイルを開きたいときに、そのファイルが存在するかを調べるにはどうすれば良いのでしょう。
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```
まず`File::open`の戻り値が`Result`型であることを知る必要がある。それは`File::open()`にあえて間違えた型を指定してコンパイラで調べることで知ることができる。実際に`u32`型に代入して調べてみるとこのようなエラーが表示される。
```
error[E0308]: mismatched types
(エラー: 型が合いません)
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  (注釈: 予期した型は`u32`です)
             found type `std::result::Result<std::fs::File, std::io::Error>`
  (実際の型は`std::result::Result<std::fs::File, std::io::Error>`です)
```
これにより戻り値の型は`Result<T,E>`であることが分かる。`File::open`が成功した場合、変数`f`の値はファイルハンドルを含む`Ok`インスタンスになる。失敗した場合、発生したエラーの種類に関する情報をより多く含む`Err`インスタンスが`f`の値になる。
### いろいろなエラーにマッチする
上の例では`File::open`が失敗した理由に関わらず`panic!`する。代わりにしたいことは、失敗理由によって動作を変えることである。ファイルが存在しないために`File::open`が失敗したなら、ファイルを、作成し、その新しいファイルへのハンドルを返したい。他の理由で、`Filr::opne`が失敗したら、`panic!`してほしい。
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```
`File::open`が`Err`列挙子に含めて変える値の型は、`io::Error`であり、これは標準ラうぶらりで提供されている構造体である。この構造体には、呼び出すと`io::ErrorKind`値が得られる`kind`メソッドがある。`io::ErrorKind`というeunmは、標準ライブラリで提供されていて、`io`処理の結果発生する可能性のあるいろいろな種類のエラーを表す列挙子がある。使用したい列挙子は、`ErrorKind::Notfound`で、これは開こうとしているファイルがまだ存在しないことを示唆する。  
`if error.kind()==ErrorKind::Notfound`という条件式は、**マッチガード**と呼ばれる。アームのパターンを更に洗練する`match`アーム乗のおまけの条件式である。この条件式は、そのアームのコードが実行されるには死んでなければいけない。そうでなければ、パターンマッチングは敬是櫛、`match`の次のアームを考慮する。パターンの`ref`は、`error`がガード条件式にムーブされないように必要。参照みたいなもの。  
マッチガードで精査したい条件は`error.kind()`による返り値が`ErrorKind`eunmの`NotFound`列挙子であるかということである。もしそうなら、`File::create`でファイル作成を試みる。`File::create`も成功したなら`File`型の値を返し、失敗したなら`Error`型の値を返す。

### エラー時にパニックするショートカット
`match`の使用は十分に仕事を素てくれるが、いささか冗長になるうえ、甘楽図師も意図を良く伝えるとは限らない。`Result<T,E>`型にはいろいろな作業をするヘルパーメソッドが多く定義されている。これらの関数の一つは`unwrap`と呼ばれる。これは`Result`値が`Ok`列挙子なら、`unwrap`は`Ok`の中身を返す。`Result`が`Err`列挙子なら、`unwrap`は`panic!`マクロを読んでくれる。
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
別のメソッドである`expect`は`unwrap`に似ているが`panic!`のエラーメッセージも選択させてくれる。エラーメッセージはぷっろぐらまが指定できる。
```rust
use std::fs::File;

fn main() {
    // hello.txtを開くのに失敗しました
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```
### エラーを委譲する
失敗する何かを呼び出す実装をした関数を書く際、関数内でエアーを処理する代わりに、呼び出し元がどうするか決められるようにエラーを返すことができる。これはエラーの**委譲**として認知されている。例えば、ファイルからユーザー名を読み取りたい。ファイルが存在しなかったり、読み込みできなければ、関数はエラーを呼び出し元のコードに返す。
```rust
use std::fs::File;
use std::io::{self,Read};

fn main() {
    read_user_name_from_file();
}

fn read_user_name_from_file()->Result<String,io::Error>{
  let f=File::open("hello.txt");
  let mut f=match f{
    Ok(file)=>file,
    Err(e)=>return Err(e),
  };

  let mut s=String::new();

  match f.read_to_string(&mut s){
    Ok(_)=>Ok(s),
    Err(e)=>Err(e),
  }
}
```
関数の戻り値は`Result<String,io::Error>`である。つまり、この関数は`Result<T,E>`型の値を返している。ここで`T`は具体型`String`で埋められ、`E`は`io::Error`で埋められている。この関数が何も問題なく成功すれば、この関数を呼び出したコードは`String`を保持する`Ok`値を受け取る。なにか問題が有ったら`Err`値を受け取る。戻り値として`io::Error`を選んだのは、この関数で失敗する可能性のある、ファイルの読み込みと文字列の入手がどちらもこの型をエラー値として返すからである。  
`File::open`でファイルを開く。ファイルが正しく開かれているなら`f`にそのファイルを代入する。もし正しく開かれていないなら`File::open`から呼び出さ鰓たエラー値を返す。次に`String`型の`s`を生成する。そしてファイルの中身を`s`に読みだす。`read_string`が成功したら関数は成功し、`s`に入っている`Sring`を返す。もし失敗したならエラー値を返す。

### `?`演算子
`?`演算子はエラー委譲のショートカットをしてくれる。`Result`値の足誤飲置かれた`?`は`Result`値を処理するために定義した`match`式とほぼ同じように動作する。`Result`値が`Ok`なら`Ok`の中身が帰ってくるが、値が`Err`なら、`return`キーワードを使った科のように関数全体から`Err`の中身が返ってくる。  
ただ、`match`式に夜委譲と`?`に夜委譲には違いがある。`?`を使ったエラー値は標準ライブラリの`From`トレイとで定義され、エラーの型を別のものに変換する`from`関数を通る。`?`演算素が`from`関数を呼び出すと、受け取ったエラー型が現在の関数の戻り値型で定義されているエラー型に変換される。
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```
さらに`?`メソッドを連結することでコードを短くすることができる。ただ、`?`演算子には注意することがある。それは
**`?`演算子は`Result`を返す関数でしか使用できない**ということである。すなわち
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```
このようには使えない。

## `panic!`すべきかそうでないか
### `panic`を使うべき場面
* 不変条件、契約、保証が破られた場合
* 呼び出し側が明確に間違った使い方をしている場合
* テスト、プロトタイピング  
不変条約とは、プログラム上で常に成立しなけらばならない条約のことである。データ構造やオブジェクトの状態に関する制約を表す。
```rust
struct Range {
    start: u32,
    end: u32,
}
```
`start<=end`であることを不変条約としたい場合、これは常に成り立つようにすべき。条件が破られたらバグの兆候とみなせる。  
契約とは、関数やモジュールの前提条件、事後条件、不変条件を明確にしどのように使うべきかを定めた約束である。
```rust
/// 分母 `denominator` は 0 であってはならない
fn divide(numerator: i32, denominator: i32) -> i32 {
    if denominator == 0 {
        panic!("Division by zero");
    }
    numerator / denominator
}
```
この場合前提条件は`denominator!=0`である。この前提条件が破られたときに`panic!`すべきである。  
保証とは、Rustコンパイラがこの条件は必ず満たされると保証する性能のこと。所有権、借用規則などが該当する。例としては、「可変と不変の借用は同時に存在しないこと」、「`Vec<T>`は内部的に連続したメモリ領域を保証する」などである。  
またテスト、プロトタイピングでは`unwrap`や`expect`を使って素早く実装し、テスト失敗時に即座にパニックして知らせることは有効である。
### `Result`を使うべき場面
* 関数の呼び出し元がエラーに対処できる場合
* 呼び出し側に選択肢を与えたい場合
`Result`を使うとより柔軟で健全なAPI設計をすることが出来る。そのため出来れば`Result`を使いたい。
### 独自の型を作る
独自の型を作ることでコードをより分かりやすく、堅牢にすることが出来る
```rust
use std::io;
fn main(){
  let g=Guess::new(3);
  println!("{}",g.value());
}

pub struct Guess{
  value:u32,
}

impl Guess{
  pub fn new(value:u32)->Guess{
    if value<1||value>100{
      panic!("Guess value must be between 1 and 100,got {}.",value);
    }
    Guess{
      value
    }
  }

  pub fn value(& self)->u32{
    self.value
  }
}
```
## ジェネリクス
ジェネリクスとは「型に依存しない汎用的なコードを書くための仕組み」である。  
### 関数定義
ベクタから最大値を取りたい。`char`と`i32`の最大値をそれぞれ取りたいときに異なる関数を定義する必要がある。
```rust
use std::io;
fn main(){
  let num_list=vec![33,43,22,46,100];
  let char_list=vec!['a','c','r','s','b'];

  println!("{}",largest_i32(&num_list));
  println!("{}",largest_char(&char_list));
}

fn largest_i32(list:&Vec<i32>)->i32{
  let mut largest=list[0];

  for item in list{
    if item>largest{
      largest=item;
    }
  }
  largest
}

fn largest_char(list:&Vec<char>)->char{
  let mut largest=list[0];
  for &item in list.iter(){
    if item>largest{
      largest=item;
    }
  }
  largest
}
```
ただ、これだとコードを書くのに時間がかかり、エラーも起こりやすい。そこで単独の関数にジェネリックな型引数を導入してこの重複を排除する。これから定義する新しい関数の型を引数にするには、ちょうど関数の値引数のように型引数に名前をつける必要がある。Rustでは慣習的にtypeの省略形である`T`を使用する。  
関数で使用する際は、コンパイラがその名前の意味を把握できるようにシグニチャでその引数名を宣言しなければならない。また、型引数名を関数シグニチャでしようする差異には、使用する前に型引数名を宣言しなければならない。
```rust
fn largest<T>(list:&Vec<T>)->T{
```
では次のように`largest`関数を定義するとどうなるか
```rust
fn largest<T>(list:&Vec<T>)->T{
  let mut largest=list[0];
  for &item in list.iter(){
    if item>largest{
      largest=item;
    }
  }
  largest
}
```
これはエラーになる
```
13 |     if item>largest{
   |        ----^------- T
   |        |
   |        T
   |
help: consider restricting type parameter `T` with trait `PartialOrd`
```
注釈は`std::comp::PartialOrd`に触れている。これは**トレイト**である。このエラーは`T`がなりうるすべての可能性のある型に対して動作しないと述べている。本体で型`T`の値を比較したいとき、値が順序付け可能な型のみしか使用できない。比較を可能にするために、標準ライブラリには型に実装できる`std::comp::PartialOrd`トレイトがある。
### 構造体定義
構造体でジェネリクスを使用することもできる
```rust
use std::io;
struct Point<T>{
  x:T,
  y:T,
}

fn main(){
  let integer=Point{x:5,y:10};
  let float=Point{x:1.0,y:4.2};

  println!("{}",&float.x);
}
```
ジェネリックな型を１つだけ使用して`Point<T>`を定義したため、この定義は`Point<T>`構造体が何らかの型`T`に関して、ジェネリックであると述べている。その型が何であれ、`x`と`y`のフィールドは両方その同じ型になっていることに注意したい。例えば、
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```
これはエラーになる。その場合は複数のジェネリックな型引数を使用できる。
```rust
use std::io;
struct Point<T,U>{
  x:T,
  y:U,
}

fn main(){
  let integer_float=Point{x:5,y:4.0};
}
```
### enum定義
今までに出てきた`Optin<T>`と`Result<T,E>`はジェネリックなデータ型を保持するenumの良い例である。
```rust
enum Option<T> {
    Some(T),
    None,
}
```
型`T`の値を保持する`Some`と、値を何も保持しない`None`である。
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
`Result`は2つの型`T`、`E`に対してジェネリックで、2つの列挙子がある。型`T`の値を保持する`Ok`と型`E`の値を保持する`Err`である。
### メソッド定義
メソッド内で型`<T>`を使用したい場合は`impl`の後に`T`を宣言しないといけない。
```rust
use std::io;
struct Point<T,U>{
  x:T,
  y:U,
}

fn main(){
  let integer=Point{x:5,y:2};
  let float=Point{x:5.0,y:4.0};

  println!("{}",float.distance_from_origin());
  println!("{}",integer.x());
  //println!("{}",integer.distance_from_origin());
}

impl Point<f32,f32>{
  fn distance_from_origin(&self)->f32{
    (self.x.powi(2)+self.y.powi(2)).sqrt()
  }
}

impl<T,U> Point<T,U>{
  fn x(&self)->&T{
    &self.x
  }
}
```
特定の型のみで関数を使用したい場合はジェネリクスを指定しない。コード上では`impl`の後に`<T>`を宣言しないで以前のように関数を定義する。  
構造体定義のジェネリックな型引数は、必ずしもその構造体のメソッドシグニチャで使用するものと同じにはならない。
```rust
use std::io;
#[derive(Debug)]
struct Point<T,U>{
  x:T,
  y:U,
}

fn main(){
  let p1=Point{x:5,y:1.2};
  let p2=Point{x:"Hello",y:'t'};
  let p3=p1.mixup(p2);

  println!("{:?}",p3);
  
}

impl<T,U> Point<T,U>{
  //所有権ムーブ
  fn mixup<V,W>(self,other:Point<V,W>)->Point<T,W>{
    Point{
      x:self.x,
      y:other.y,
    }
  }
}
```
### パフォーマンス
ジェネリックな型引数を使用すると、実行時にコストが発生するのかな、と思うかもしれない。Rustではジェネリクス
を使用しているコードの単相化をコンパイル時に行うことで達成している。**単相化**とは、コンパイル時に使用されている具体的な型を入れることで、ジェネリックなコードを特定のコードに変換する過程のこと。


## トレイト
型の振る舞いはその方に対して呼び出さるメソッドから構成される。異なる型は、それらの型すべてに対して同じメソッドを呼び出せるなら、同じ振る舞いを共有することになる。トレイトは、メソッドシグニチャをあるグループにまとめ、何らかの目的を達成するのに必要な一連の振る舞いを定義する手段である。C++でいうと、基底クラスに近い概念。振る舞いの共通化、ジェネリクスと組み合わせての型の制約、オブジェクト指向風の設計が用途である。
### 定義と実装
`Tweet`構造体と`NewsArticle`構造体にトレイトを実装する。トレイトの定義は`trait`と宣言することでできる。
```rust
pub trait Summary{
    fn summarize(&self)->String;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{},by {} ({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{}:{}",self.username,self.content)
    }
}
```
`impl`の後にトレイト名を置き、それから`for`キーワード、更にトレイトの実装対象の型の名前を指定することでトレイトを特定の型に実装することが出来る。トレイトを指定したら、型の中に絶対にトレイト内の関数をすべて持たないといけない。もし、この構造体、トレイトを`lib.rs`で定義した場合、`main`で呼び出す際は、トレイトも`use`で呼び出す必要がある。
```rust
use std::io;
use practice::{Tweet,NewsArticle,Summary};

fn main(){
  let tweet=Tweet{
    username:String::from("horse_ebooks"),
    content:String::from(
      "Hi"
    ),
    reply:false,
    retweet:false,
  };

  println!("{}",tweet.summarize());
}
```
`lib.rs`を`aggregator`と呼ばれるクレート専用にする。このクレート内のトレイトがどこまで有効なのかをみていく。ここで注意すべき制限の1つが**トレイトか対称の型が自分のクレートに固有であるときのみ、型に対してトレイトを実装できる**ということである。例えば、`Display`のような外部のクレートは`aggregator`クレートの機能の一部として、`Tweet`のような独自の型に実装できる。型`Tweet`が`aggregrator`追うレートに固有だからである。また、`Summary`を`aggregator`で標準ライブラリの`Vec<T>`に対して実装することもできる。  
しかし、外部のトレイトを外部の型に対して実装することは出来ない。例として、`aggregator`クレート内で`Vec<T>`に対して`Display`トレイトを実装することは出来ない。`Display`と`Vec<T>`は標準ライブラリで定義され、`aggregator`クレートに固有ではないからである。この制限は**孤児のルール**と呼ばれる、プログラム特性の一部である。このおかげで、他の人のコードが自分のコードを壊したりすることがないことを保証してくれる。

### デフォルト実装
ときとして、すべての型の全メソッドに対して実装を要求するのではなく、トレイトのすべて、あるいは一部のメソッドに対してデフォルトの振る舞いがあると有用である。特定の型にトレイトを実装する際、各メソッドのデフォルト実装を保持するかオーバーライドするか選べる。
```rust
pub trait Summary{
    fn summarize_author(&self)->String;

    fn summarize(&self)->String{
        format!("(Read more from {} ...)",self.summarize_author())
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{},by {} ({})",self.headline,self.author,self.location)
    }
    fn summarize_author(&self)->String{
        String::from("")
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("@{}",self.username)
    }
}
```
```rust
use std::io;
use practice::{Tweet,NewsArticle,Summary};

fn main(){
  let tweet=Tweet{
    username:String::from("horse_ebooks"),
    content:String::from(
      "Hi"
    ),
    reply:false,
    retweet:false,
  };

  let news=NewsArticle{
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
  };
  println!("{}",news.summarize());
  println!("{}",tweet.summarize());
}
```
```
Penguins win the Stanley Cup Championship!,by Iceburgh (Pittsburgh, PA, USA)
(Read more from @horse_ebooks ...)
```

### 引数としてのトレイト
トレイトを使っていろいろな種類の型を受け付ける関数を定義する方法を学ぶ。`item`の`summarize`メソッドを呼ぶ関数`notify`を定義する。この関数では、`Summary`をトレイトに持つどんな型でも引数に取ることが出来る。そして`Summary`トレイトの`summarize`を引き出すことが出来る。
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
### トレイト境界構文
実は`impl Trait`構文(上の例)は、より長い**トレイト境界(trait bound)**と呼ばれる姿の糖衣構文なのである。  
```rust
pub fn notify<T:Summary>(item:&T){
  println!("Braeking news! {}",item.summarize());
}
```
山かっこの中にジェネリックな型引数の宣言を書き、型引数の後ろにコロンをはさんでトレイト境界を置く。他にも
```rust
pub fn notify(item1:&impl Summary,item2: &impl Summary){
```
```rust
pub fn notify<T:Summary>(item1:&T,item2:&T){
```
は等価である。  
また、複数のトレイト境界を`+`構文で指定することもできる。例えば`notify`に`summarize`メソッドに加えて`item`の画面出力形式を使わせたいとする。このとき`notify`の定義に`item`は`Display`と`Summary`の両方を実装しなければならない。この時`+`構文を使うことができる。
```rust
pub fn notify(item:&(imple Summary+Display)){
```
```rust
pub fn notify<T:Summary+Display>(item:&T){
```
複数のジェネリック型の行き数を持ち、引数としてトレイトそ渡したい場合、関数のシグネチャが読みにくくなってしまう。
```rust
fn some_function<T:Display+Clone,U:Clone+Debug>(t:&T,u:&U)->i32{
```
代わりに`where`句を使いこのように書くことができる
```rust
fn some_function<T,U>(t:&T,u:&U)->i32
  where T:Display+Clone,
        U:Clone+Debug,
{
```
### トレイトを実装している型を返す
`impl Trait`構文を戻り値型のところで使うことにより、あるトレイトを実装する何らかの型を返すことが出来る。
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```
戻り値を`impl Summary`にすることにより、具体的な型が何かをゆうことなく、`returns_summarizable`関数は`Summary`トレイトを実装している何らかの型を返すのだ、と指定することが出来る。ただ、複数の型を指定することはできない。次の例はエラーになる。
```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

### Largest関数の修正
```rust
fn largest<T>(list:&Vec<T>)->T{
  let mut largest=list[0];

  for &item in list{
    if item>largest{
      largest=item;
    }
  }
  largest
}
```
この関数はジェネリクス型に指定できる`T`は、順序付けできない型も含んでしまう。
```
  --> src/main.rs:12:12
   |
12 |     if item>largest{
   |        ----^------- T
   |        |
   |        T
   |
help: consider restricting type parameter `T` with trait `PartialOrd`
   |
8  | fn largest<T: std::cmp::PartialOrd>(list:&Vec<T>)->T{
   |             ++++++++++++++++++++++
```
このようにエラーが出る。演算子`>`は標準ライブラリの`std::cmp::PartialOrd`デフォルトメソッドとして定義されているため`largest`関数が、比較できるあらゆる型のスライスに対して動くようにするには`T`のトレイト境界に`PartialOrd`を指定する必要がある。では
```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {
```
このように書くとどうなるか。
```
 --> src/main.rs:9:19
  |
9 |   let mut largest=list[0];
  |                   ^^^^^^^ move occurs because value has type `T`, which does not implement the `Copy` trait
  |
help: if `T` implemented `Clone`, you could clone the value
 --> src/main.rs:8:12
  |
8 | fn largest<T:PartialOrd>(list:&Vec<T>)->T{
  |            ^ consider constraining this type parameter with `Clone`
9 |   let mut largest=list[0];
  |                   ------- you could clone this value
help: consider borrowing here
  |
9 |   let mut largest=&list[0];
  |                   +

error[E0507]: cannot move out of a shared reference
  --> src/main.rs:11:16
   |
11 |   for &item in list{
   |        ----    ^^^^
   |        |
   |        data moved here
   |        move occurs because `item` has type `T`, which does not implement the `Copy` trait
   |
help: consider removing the borrow
   |
11 -   for &item in list{
11 +   for item in list{
```
このようなエラーが出る。ジェネリックでないバージョンの`largest`関数では、最大の`i32`か`char`を探そうとするだけだった。`i32`や`char`のようなサイズが既知の型はスタックに格納できるため、`Copy`トレイトを実装している。しかし、`largest`関数をジェネリックにすると、`list`引数が`Copy`トレイトを実装しない型を含む可能性が出てきた。結果として`list[0]`から値を`largest`にムーブできず、このエラーに陥った。
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
このように書けば`PartialOrd`と`Copy`を実装する、`largest`関数の完全なコードになる。
### トレイト境界を使用して、s￥メソッド実装を条件分けする
`Pair<T>`は、内部の型`T`が比較を可能にする`partialOrd`トレイトと出力を可能にする`Display`トレイトを実装しているときのみ、`comp_display`メソッドを実装する。
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
ジェネリクスを使うことで1つの関数定義で様々な型を使うことが出来る。トレイトは基底クラスみたいなイメージ。関数を定義して、様々な型に対して共通の動作を指定することが出来る。関数でトレイトを呼び出す場合は`impl Trait`やトレイト境界構文を使う必要がある。複数のジェネリクスを持つ関数を持つこともできるし、1つのジェネリクスに対して、複数のトレイトをつけることもできる。

## ライフタイム
ライフタイムとは、その参照が有効になるスコープのことである。多くの場合、型が推論されるようにライフタイムも暗示的に推論される。行く数の型の可能性があるときは、型を注釈しなければならない。同様に、参照のライフタイムがいくつか異なる方法で関係することがある場合には注釈しなければならない。  
ライフタイムの主な目的はダングリング参照を回避すること。例えば、以下のプログラムではダングリング参照が起こる。
```rust
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
```
この例では。スコープの外側で`r`が宣言され、スコープの内側で`x`が宣言される。その後、スコープの内側で`r`が`x`の参照をセットするが、スコープが終わると同時に、参照先である`x`は解放される。`r`は解放されたメモリを見ることになる。これは危険である。Rustでは借用チェッカーによってコンパイラがこのコードが無効であると決定してくれる。
### 借用チェッカー
スコープを比較してすべての借用が有効であるかを決定するきのうを**借用チェッカー**と呼ぶ。
```rust
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
```
`r`のライフタイムは`'a`と表され、`x`のライフタイムが`'b`と表される。`x`を参照する`r`は`x`のライフタイム`'b`よりも長生きである。この場合、借用チェッカーに引っかかってしまう。
### 関数のジェネリックなライフタイム
2角文字列スライスのうち、長い方を返す関数を書きたい。関数の引数に所有権を奪ってほしくないため、引数を参照にする。
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```
`longest`関数を次のように定義する
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
これではコンパイル出来ない。
```$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
(エラー[E0106]: ライフタイム指定子が不足しています)
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |                                 ^ expected lifetime parameter
  |                                   (ライフタイム引数があるべきです)
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
  (助言: この関数の戻り値型は借用された値を含んでいますが、
   シグニチャは、それが`x`と`y`どちらから借用されたものなのか宣言していません)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10`.

To learn more, run the command again with --verbose.
```
助言テキストは、戻り値の型はジェネリックなライフタイム引数である必要があると明かしている。コンパイラは、返している参照が`x`か`y`のとちらを参照しているか分からないからである。返す参照が常に有効であるかを決定したときのようにスコープを見ることも、参照の具体的なライフタイムが分からないから出来ないのである。
### ライフタイム注釈記法
シグニチャにジェネリックな型引数を指定された関数が、あらゆる型を受け取ることができるのと同様に、ジェネリックなライフタイム引数を指定された関数は、あらゆるライフタイムの参照を受け取ることができる。ライフタイム注釈は、ライフタイムに影響することなく、複数の参照のライフタイムのお互いの関係を記述する。
```rust
&i32        // a reference
            // (ただの)参照
&'a i32     // a reference with an explicit lifetime
            // 明示的なライフタイム付きの参照
&'a mut i32 // a mutable reference with an explicit lifetime
            // 明示的なライフタイム付きの可変参照
```

### 関数シグニチャにおけるライフタイム注釈
では実際に`longest`関数をライフタイム注釈を用いて修正する。
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
何らかのライフタイム`'a`に対して、関数は2つの引数をとり、どちらも少なくともライフタイム`'a`と同じだけ生きる文字列スライスであるとコンパイラに教えるようになった。また、この関数シグニチャは、関数から変える文字列スライスも少なくともライフタイム`'a`と同じだけ生きると、コンパイラに教えている。具体的な参照を`longest`に渡すと、`'a`に代入される具体的なライフタイムは、`x`のスコープの一部であって`y`のスコープと重なる部分になる。言い換えると、ジェネリックなライフタイム`'a`は、`x`と`y`のライフタイムの小さい方に等しい具体的な
ライフタイムになる。
```rust
fn main() {
    // 長い文字列は長い
    let string1 = String::from("long string is long");
    // （訳注：この言葉自体に深い意味はない。下の"xyz"より長いということだけが重要）

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // 一番長い文字列は{}
        println!("The longest string is {}", result);
    }
}
```
これはコンパイルが通るだろうか。答えはyesである。ジェネリックなライフタイム`'a`は、`string1`と`string2`の重なった部分のライフタイムになる。従って、`'a`は`string2`のライフタイムになる。`result`のライフタイムは`string2`のライフタイムに含まれるため、`longest`関数のライフタイム`'a`は矛盾しない。
```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
ではこれは。これはコンパイルエラーになる。先ほどと同様に、`longest`関数の引数のライフタイムは`string2`のライフタイムになる。戻り値のライフタイムは`result`のものになるが、これは`string2`のライフタイムの外側にある。そのため、ダングリング参照する恐れがあり、コンパイルはエラーをはく。  
ライフタイム引数を指定する必要があるかは、関数が行っていることに依存する。
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```
例えば、引数`x`と戻り値に対してライフタイム`'a`を指定したが、引数`y`には指定していない。これは`y`のライフタイムが`x`や戻り値のライフタイムとは何も関係がないからである。  
関数から参照を返す差異、戻り値のライフタイム引数は、引数のうちどれかのライフタイム引数を一致する必要がある。
```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    // 本当に長い文字列
    let result = String::from("really long string");
    result.as_str()
}
```
戻り値型にライフタイム引数`'a`を指定しても、戻り値のライフタイムは。引数のライフタイムと全く関係がないから、これはコンパイルできない。修正するには、参照ではなくデータ型を返せばよい。
### 構造体定義のライフタイム注釈
いままでは、所有された型を保持する構造体だけを定義してきた。構造体に参照を保持させることもできるが、その場合、構造体定義の全参照にライフタイム注釈を付け加える必要がある。
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    //                                                  "'.'が見つかりませんでした"
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
構造体に文字列スライスを保持するフィールド、`part`があり、これは参照である。ライフタイム`'a`を記述することで、構造体インスタンス`i`が参照した`&str`型の`first_sentence`よりも長生きであることを保証してくれる。
```rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  // 僕をイシュマエルとお呼び。何年か前・・・
  let novel = String::from("Call me Ishmael. Some years ago...");
  //                                                  "'.'が見つかりませんでした"
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  {
    let i = ImportantExcerpt {
      part: first_sentence,
    };
  }
}
```
これは`first_sentence`が構造体インスタンスの`ImportantExcerpt`よりも長生きしているため、エラー。

### ライフタイム省略
全参照にはライフタイムがあり、参照を使用する関数や構造体にはライフタイム引数を指定する必要があった。ただ
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
はライフタイム注釈なしでコンパイルできた。実は、Rust(ver1.0)以前では
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```
と書かれていた。このようにライフタイム注釈を入力されていられる部分がたくさん見られた。そこで、**ライフタイム省略規則**がコンパイラの参照解析に落とし込まれた。その規則に最後まで到達し、それでもライフタイムを割り出せない参照があった場合は、kンπらはエラーで停止する。引数のライフタイムは**入力ライフタイム**、戻り値のライフタイムは**出力ライフタイム**と称される。
1. 参照である各引数は、独自のライフタイム引数を得る。1引数の関数は、1つのライフタイム引数を得る。`fn foo<'a>(x:&'a i32);`2つの場合は。2つの個別のライフタイム引数を得る。`fn foo<'a.'b>(x:&a i32,y:'b i32)`
2. 1つだけ入力ライフタイム引数があるなら、そのライフタイムがすべての出力ライフタイム引数に代入される。`fn foo<'a>->&'a i32`
3. 複数の入力ライフタイム引数があるけど、メソッドナノでそのうちの一つが`&self`や`&mut self`だったら、`self`のライフタイムが全出力ライフタイム引数に代入される。  

コンパイラの立場になって`first_word`関数のライフタイムがどうなっているかを追跡する。まず関数のシグニチャは参照に紐づけられるライフタイムがない状態から始まる。
```rust
fn first_word(s: &str) -> &str {
```
1番目の規則が適用される。引数が1つであり、それは独自のライフタイム`'a`を得る。
```rust
fn first_word<'a>(s: &'a str) -> &str {
```
2番目の規則によって、1つの入力引数のライフタイムが出力引数に代入されるから次のようになる。
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```
すべての参照にライフタイムがついたから、コンパイラは、プログラマにこの関数シグニチャのライフタイムを注釈してもらう必要はない。  
では`longest`関数をみる。
```rust
fn longest(x: &str, y: &str) -> &str {
```
まず第1規則が適用される。
```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
2つ以上入力ライフタイムがあるため、2番目の規則は適用されない。3番目の規則も適用されない。この時点で、戻り値はライフタイムを指定されていないからエラーになる。
### メソッド定義におけるライフタイム注釈
構造体にライフタイムのあるメソッドを実装するとき、ジェネリックな型引数と同じ記法を使用する。`impl`ブロック内のメソッドシグニチャでは、参照は構造体のフィールドの参照のライフタイムに紐づいている可能性と、独立している可能性がある。
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        //       "お知らせします: {}"
        println!("Attention please: {}", announcement);
        self.part
    }
}
```
この場合、関数は2つの引数の参照をもつ。そして、戻り値にも参照をもつ。まず1番目の規則が適用され、`&self`と`announcement`に独自のライフタイムを与える。次に3つ目の規則が適用されうる。引数の1つが`&self`であるため、戻り値は`&self`のライフタイムをえる。

### 静的ライフタイム
特殊なライフタイムとして`'static`が存在する。これは、この参照がプログラムの全期間存在することを意味する。
```rust
// 僕は静的ライフタイムを持ってるよ
let s: &'static str = "I have a static lifetime.";
```
むやみに使うべきではない。値が長生きしても安全であることを明示したいときにつかう。
### ジェネリックな型引数、トレイト境界、ライフタイムを一度に
```rust
use std::io;
use std::thread;
use std::fmt::Display;

fn longest_with_ans_announcement<'a,T>(
  x:&'a str,
  y:&'a str,
  ann: T,
)->&'a str
where T:Display
{
  println!("{}",ann);
  if x.len()>y.len(){
    x
  }else{
    y
  }
}
fn main() {
  let s1=String::from("yes");
  let s2=String::from("no");
  let num:i64=89;

  let s1=longest_with_ans_announcement(&s1,&s2,num);

  println!("{}",s1)
}
``` 
ジェネリックな型引数、トレイト、トレイト境界、ジェネリックなライフタイム引き数により、多くの異なる場面で動くコードを繰り返すことなく各準部ができた。ジェネリックな型引数により、コードを異なる型に適用させてくれる。トレイトとトレイト境界は、型がジェネリックであっても、コードが必要とする振る舞いを持つことを保証する。ライフタイム注釈を活用することで、柔軟なコードにダングリング参照が存在しないことを保証する方法を学んだ。

## テスト
Rustにはプログラムの正当性に重きを置いて設計されているが、正当性は複雑で、単純に証明することはない。Rustの型システムはこの重荷の多くの部分を肩代わりするが、型システムはあらゆる種類の不当性を捕捉することはできない。Rustでは、言語内で自動化されたソフトウェアテストを書くことをサポートしている。
## テストの記述法
Rustのテストでは典型的には以下の3つの動作を行う
1. 必要なデータや状態をセットアップする
2. テスト対称のコードを走らせる
3. 結果が想定道理であることを断定する

### テスト関数の構成
Rustに置けるテストとは`test`属性で注釈されたな関数のことである。次のように`adder`プロジェクトをつくる。
```rust
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```
`src/lib.rs`の中身は
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
このようになっている。`fn`の行に`#[test]`注釈があることがわかる。これは`it_works`関数がテスト関数であることをしめすものである。`tests`モジュールは内部モジュールであるから、外部モジュール内のテストはいかにあるコードを内部モジュールのスコープに持っていく必要がある。`use super::*`と入れる必要がある。実際に`cargo test`でテストする。
```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running target/debug/deps/adder-92948b65e88960b4

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
`tests::it_works ... ok`は全テストが通ったことを意味する。`passed`、`failed`は通過、失敗したテストの数を合計している。`ignored`は無視すると指定されたテストの数。`measured`はパフォーマンスを測定するベンチマークテスト用である。  
テストを失敗してみる
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another(){
        panic!("Make this test fail");
    }
}
```
```$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running target/debug/deps/adder-92948b65e88960b4

running 2 tests
test tests::another ... FAILED
test tests::exploration ... ok

failures:

---- tests::another stdout ----
thread 'main' panicked at 'Make this test fail', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```
`test test::another`の行は`FAIlED`を表示し、失敗したことを示す。
### `assert!`
`assert!`マクロは、標準ライブラリで提供されているが、テスト内の何らかの条件が`true`と評価されることを確かめたいときに有効である。`assert!`マクロは、値が`true`ならテストは通るが、`false`なら`panic!`を呼び出し、テストは失敗する。

### `asser_eq!`と`assert_ne!`
コードの結果と期待される値が等しいときにテストを通す場合は`assert_eq!`を、異なっているときにテストを通す場合は`assert_ne!`を使う。
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```
比較対象の値は`PartialEq`と`Degub`トレイトを実装していなければならない。すべての組み込み型と、ほぼすべての標準ライブラリの型はこれらのトレイトを実装している。自分で定義した構造体やenumについては、その型の値が等しいか等しくないかをアサーションするために、`PartialEq`を実施する必要がある。また失敗したときにその値をプリントできるように`Debug`を追加する必要がある。定義に`#[derive(PartialEq,Debug)]`という注釈を追加すればよい。

### カスタムの失敗メッセージを追加する
例えば、このように`String`に特定の文字列が入っているかをテストする場合を考える
```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```
ただ、これではテストはアサーションが成功するかを示し、失敗した場合はどの行にアサーションがあるかを示すだけである。`greeting`が実際に度の値を持っているかを確認出来たらより有用になる。
```rust
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            //挨拶(greeting)は名前を含んでいません。その値は`{}`でした
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
```
そこでこのように書くことでエラーメッセージを受け取ることが出来る。

### `should_panic`でパニックを確認する
期待する正しい値をコードが返すことを確認することに加えて、想定通りにコードがエラー状態を扱っていることを確認することも重要である。テスト関数に`should_panic`という別の属性を追加することで、関数内のコードがパニックしたら、テストを通過させる。パニックしなかったら、テストは失敗する。
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            //予想値は1から100の間でなければなりませんが、{}でした。
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
この場合、`panic!`が起こるためテストは通る。`should_panic`を使用するテストは不正確なこともある。なぜならｍコードが何らかのパニックを起こしたことしか示さないからである。`should_panic`の正確性を期すために、`shouldd_panic`属性に`expected`引数を追加することもできる。`ecpected`引数と同じ`panic!`が発生したとき、テストは通る。そうでないとき、テストは通らない。
```rust
// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                //予想値は1以上でなければなりませんが、{}でした。
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                //予想値は100以下でなければなりませんが、{}でした。
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //予想値は100以下でなければなりません
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### `Result<T,E>`をテストで使う
`Result<T,E>`を使い、パニックする代わりに`Err`を返すように書き直す
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
テストが成功すれば`Ok(())`を、失敗すれば`Err`に`String`を入れて返すようにする。

## テストの実行のされ方を制御する
`cargo run`がコードをコンパイルし、出来上がったバイナリを走らせるのと同様に、`cargo test`はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行する。`cargo test`では標準では、スレッドを使用して並行にはしる。これは、テストが早く実行し終わり、コードが機能しているかに関わらず、反応をより早く得られることを意味する。ただ、これはデータの競合(data race)の元となる。例えば、各テストがディスクに`test_output.txt`というファイルを作成し、何らかのデータを書き込むコードを走らせるとする。そして、各テストはそのファイルのデータを読み込み、ファイルが特定の値を含んでいるとアサーションし、その値は各テストで異なる。テストが同時に走るため、あるテストが他のテストに書き込んだり読み込んだりする間にファイルを上書きするかもしれない。それから2番目のテストが失敗する。解決策は、各テストが異なるファイルに書き込むこと、または一度に一つのテストを実行することである。  
並行にテストを実施しない場合、`--test-threads`フラグと使用したいスレッド数をテストバイナリに送ることが出来る。  
```
cargo test -- --test-threads=1
```  
テストスレッド数を1にセットし、並列処理を行わない。時間はかかるが、お互いに邪魔することはない。

### 関数の出力を表示する
```rust
fn prints_and_returns_10(a: i32) -> i32 {
    //{}という値を得た
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

```
例えばこれらをテストする。すると
```
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
        I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
このような結果になり、`I got the value 8`のみえられる。テストに失敗した場合のみ、出力が得られるのである。テストを通った場合、この関数の出力はキャプチャされてしまう。これを無効にするには
```
cargo test -- --nocapture
```
とする。

### 名前でテストの一部を実行する
すべてテストをすると時間がかかってしまうことがある。特定の部分のコードしか対象にしていない場合、そのコードに関わるテストの実を走らせたいかもしれない。`cargo test`に走らせたい名前を引数として渡すことで、実行するテストを選ぶことが出来る。例えばこのような関数があるとする
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```
単独のテストを走らせる場合  
```
cargo test one_hondred
```
このように名前を指定する。  
特定の文字列をを含む関数をすべて実行する場合  
```
cargo test add
```
すると名前に`add`が入った関数のみテストされる。

### 特に要望のない場合テストを無視する。
時には時間のかかるテストを無視したい場合があるだろう。その場合には関数に`ignore`属性を付与するとよい。
```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 実行に1時間かかるコード
    // code that takes an hour to run
}
```
この場合、`expensive_test`関数のテストはスキップされえる。