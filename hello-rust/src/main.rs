// #[derive(Debug)]

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let a = 123;
    // a = 456; // 不对，此时a是不可变变量
    // a = "asd"; // 类型不对
    let a = 456; // 变量的值可以重新绑定
    const c: i32 = 123;
    //const c = 546; // 但是常量的值不能重新绑定
    /*
        变量的值可以"重新绑定"，但在"重新绑定"以前不能私自被改变，
        这样可以确保在每一次"绑定"之后的区域里编译器可以充分的推理程序逻辑。
    */
    // println!("a={}", a);
    // println!("c={}", c);

    // mut
    // "可变"（mutable）
    let mut b = 123;
    b = 456;
    // println!("a={}", b);

    // 声明类型
    let a2: u64 = 123;

    // 重影（Shadowing）
    // 重影就是指变量的名称可以被重新使用的机制
    // 重影与可变变量的赋值不是一个概念
    // 重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。
    // 但可变变量赋值仅能发生值的变化。

    // 元组
    let tup: (i32, f32, u8) = (123, 3.14, 5);
    let (x, y, z) = tup;

    // 数组
    // 是 [] 包起来的，同类型的数据
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];
    let mut arr4 = [1, 2, 3];
    arr4[1] = 5;

    // fun1();
    // println!("add1(3, 4) = {}", add1(3, 4));

    // 变量与数据交互的方式_克隆();
    // test_impl();
    // 结构体打印();
    // 命令行输入();
    // 文件读取();
    // 文件写入();
    // vector1();
    test_Person();
}

fn say_fun() {
    use ferris_says::say; // from the previous step
    use std::io::{stdout, BufWriter};
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// fn 函数名 (参数) 函数体
// Rust不在乎您在何处定义函数，只需在某个地方定义它们即可，即在后面定义的也可以在前面使用

fn fun1() {
    println!("Hello")
}

// 如果需要具备参数必须声明参数名称和类型
// 函数体由一系列可以以表达式（Expression）结尾的语句（Statement）组成
// 可以在一个用 {} 包括的块里编写一个较为复杂的表达式
// 注：函数体表达式并不能等同于函数体，它不能使用 return 关键字
fn add1(a: i32, b: i32) -> i32 {
    // Rust 不支持自动返回值类型判断！
    // 如果没有明确声明函数返回值的类型，函数将被认为是"纯过程"，不允许产生返回值，
    // return 后面不能有返回值表达式。
    // 这样做的目的是为了让公开的函数能够形成可见的公报。
    return a + b;
}
fn add2(a: i32, b: i32) -> i32 {
    let res = a + b;
    res // 这个相当于返回值，注意没有分号
}

fn 条件语句() {
    let number = 3;
    // Rust 中的条件表达式必须是 bool 类型
    if number < 5 {
        // 不需要用小括号（可以有，但是没必要）
        // Rust 中的 if 不存在单语句不用加 {} 的规则，不允许使用一个语句代替一个块
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
}

fn while循环() {
    let mut number = 1;

    // 目前，还没有 do-while 的用法，但是 do 被规定为保留字，也许以后的版本中会用到
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
}

fn for循环1() {
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
}

/*
    点点
        ..y 等价于 0..y
        x.. 等价于位置 x 到数据结束
        .. 等价于位置 0 到结束
*/

fn for循环2() {
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
}

fn loop循环() {
    /*
        loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。
        这是一个十分巧妙的设计，
        因为 loop 这样的循环常被用来当作查找工具使用，
        如果找到了某个东西当然要将这个结果交出去
    */
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}

/*
所有权规则：
    1. Rust 中的每个值都有一个变量，称为其所有者。
    2. 一次只能有一个所有者。
    3. 当所有者不在程序运行范围时，该值将被删除。
*/

fn 变量范围() {
    // 变量范围是变量的一个属性，其代表变量的可行域，默认从声明变量开始有效直到变量所在域结束
    {
        // 在声明以前，变量 s 无效
        let s = "runoob";
        // 这里是变量 s 的可用范围
    }
    // 变量范围已经结束，变量 s 无效
}

// 堆：在程序运行时程序自己申请使用内存的机制
// 有分配就有释放，程序不能一直占用某个内存资源。因此决定资源是否浪费的关键因素就是资源有没有及时的释放。
/*
    Rust 之所以没有明示释放的步骤是因为在变量范围结束的时候，Rust 编译器自动添加了调用释放资源函数的步骤。
    这种机制看似很简单了：它不过是帮助程序员在适当的地方添加了一个释放资源的函数调用而已。
    但这种简单的机制可以有效地解决一个史上最令程序员头疼的编程问题。
*/

fn 变量与数据交互的方式_移动() {
    let x = 5;
    let y = x;
    /*
        这个程序将值 5 绑定到变量 x，然后将 x 的值复制并赋值给变量 y。
        现在栈中将有两个值 5。此情况中的数据是"基本数据"类型的数据，
        不需要存储到堆中，仅在栈中的数据的"移动"方式是直接复制，
        这不会花费更长的时间或更多的存储空间。
    */

    // 但如果发生交互的数据在堆中就是另外一种情况：
    // 第一步产生一个 String 对象，值为 "hello"。其中 "hello" 可以认为是类似于长度不确定的数据，需要在堆中存储。
    // 第二步的情况略有不同
    let s1 = String::from("hello");
    // 当变量超出范围时，Rust 自动调用释放资源函数并清理该变量的堆内存
    // 为了防止释放两次，保证安全，s1在赋值给s2时，就已经无效了（名存实亡）
    let s2 = s1;
}

// 如果需要将数据单纯的复制一份以供他用，可以使用克隆
fn 变量与数据交互的方式_克隆() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 引用
fn 引用() {
    let s1 = String::from("hello");
    // & 运算符可以取变量的"引用"
    // 当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);

    /*
        引用不会获得值的所有权。
        引用只能租借（Borrow）值的所有权。
        引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权
        租借的所有权不能修改所有者的值
    */
}

/*
    当然，也存在一种可变的租借方式，就像你租一个房子，如果物业规定房主可以修改房子结构，
    房主在租借时也在合同中声明赋予你这种权利，你是可以重新装修房子的
*/
fn 可变引用1() {
    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("oob");
    println!("{}", s2);
}

// 垂悬引用（Dangling References）
/*
    这是一个换了个名字的概念，如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针（注意，不一定是空指针，还有可能是已经释放的资源）。
    它们就像失去悬挂物体的绳子，所以叫"垂悬引用"。
    "垂悬引用"在 Rust 语言里不允许出现，如果有，编译器会发现它。
*/
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// 切片类型  slice
// 字符串进行切片操作
// 除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组
fn 切片类型_1() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    // 被切片引用的字符串禁止更改其值

    println!("{}={}+{}", s, part1, part2);
}

/*
在 Rust 中有两种常用的字符串类型：str 和 String。
str 是 Rust 核心语言类型，就是本章一直在讲的字符串切片（String Slice），常常以引用的形式出现（&str）。
凡是用双引号包括的字符串常量整体的类型性质都是 &str
String 类型是 Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。
String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
String 和 str 都支持切片，切片的结果是 &str 类型的数据。
注意：切片结果必须是引用类型，但开发者必须自己明示这一点
*/

fn String转str的一种方法() {
    let s1 = String::from("hello");
    let s2 = &s1[..];
}

// 结构体
/*
    在 Rust 里 struct 语句仅用来定义，不能声明实例，结尾不需要 ; 符号，而且每个字段定义之后用 , 分隔。
*/
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

// 使用类似JSON的 k-v 结构来定义结构体
fn 结构体_JSON写法() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };

    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob2 = Site {
        domain, // 等同于 domain : domain,
        name,   // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
    };
}

// 元组结构体
/*
元组结构体是一种形式是元组的结构体。
与元组的区别是它有名字和固定的类型格式。
它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据
*/
fn 元组结构体() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
}

/*
    结构体所有权：
        结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
        这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。
        但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn 结构体打印() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 类型 {属性和值}
    println!("rect1 is {:?}", rect1);
}

/*
结构体方法：
方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。
如果你学习过一些面向对象的语言，那你一定很清楚函数一般放在类定义里并在函数中用 this 表示所操作的实例。
Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。
结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
*/
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn test_impl() {
    // 计算一个矩形的面积
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 在调用结构体方法的时候不需要填写 self ，这是出于对使用方便性的考虑
    println!("rect1's area is {}", rect1.area());
}

// 结构体关联函数
/*
之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。
这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。
*/
impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn create(name: String, age: u32) -> Person {
        Person { name, age }
    }
    fn speak(&self) {
        println!("name={}, age={}", self.name, self.age);
    }
}

fn test_Person() {
    let p = Person::create;
    println!("{:?}", p);
}

// 单元结构体
// 结构体可以只作为一种象征而无需任何成员
// 称这种没有身体的结构体为单元结构体（Unit Struct）
struct UnitStruct;

// 枚举类
#[derive(Debug)]
enum Book1 {
    Papery,
    Electronic,
}

fn 枚举类1() {
    let book = Book1::Papery;
    println!("{:?}", book);
}

#[derive(Debug)]
enum Book2 {
    // 可以为枚举类成员添加元组属性描述
    Papery(u32),
    Electronic(String),
}
fn 枚举类2() {
    let book = Book2::Papery(1001);
    let ebook = Book2::Electronic(String::from("url://..."));
}

#[derive(Debug)]
enum Book3 {
    // 为属性命名，可以用结构体语法
    // 虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。
    // 访问的方法在 match 语法中。
    Papery { index: u32 },
    Electronic { url: String },
}

fn 枚举类3() {
    let book = Book3::Papery { index: 1001 };
}

/*
match语法：
枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。
基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。
switch 语法很经典，但在 Rust 中并不支持，
很多语言摒弃 switch 的原因都是因为 switch 容易存在因忘记添加 break 而产生的串接运行问题，
Java 和 C# 这类语言通过安全检查杜绝这种情况出现
*/

fn match语法() {
    enum Book {
        Papery { index: u32 },
        Electronic { url: String },
    }

    let book = Book::Papery { index: 1001 };
    let ebook = Book::Electronic {
        url: String::from("url..."),
    };
    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}

/*
match 块也可以当作函数表达式来对待，它也是可以有返回值的
但是所有返回值表达式的类型必须一样！
如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字
    match 枚举类实例 {
        分类1 => 返回值表达式,
        分类2 => 返回值表达式,
        ...
    }

match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。
其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。
对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示
*/
fn match语法2() {
    enum Book {
        Papery(u32),
        Electronic { url: String },
    }
    let book = Book::Papery(1001);
    match book {
        Book::Papery(i) => {
            println!("{}", i);
        }
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }
}

fn match语法3() {
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {}
    }
}

/*
Option枚举类：
Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
许多语言支持 null 的存在（C/C++、Java），这样很方便，但也制造了极大的问题，null 的发明者也承认这一点，"一个方便的想法造成累计 10 亿美元的损失"。
null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：毕竟只要出现一个这样的错误，程序的运行就要彻底终止。
为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出现（常在类型前面用 ? 符号修饰）。
Java 默认支持 null，但可以通过 @NotNull 注解限制出现 null，这是一种应付的办法。
Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类
    enum Option<T> {
        Some(T),
        None,
    }

如果你想定义一个可以为空值的类，你可以这样
    let opt = Option::Some("Hello");

如果你想针对 opt 执行某些操作，你必须先判断它是否是 Option::None：
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？
所以初始值为空的 Option 必须明确类型：
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()。
Option 是一种特殊的枚举类，它可以含值分支选择：
    let t = Some(64);
    match t {
            Some(64) => println!("Yes"),
            _ => println!("No"),
    }
*/

/*
if let语法：
    if let 匹配值 = 源变量 {
        语句块
    }
可以在之后添加一个 else 块来处理例外情况
if let 语法可以认为是只区分两种情况的 match 语句的"语法糖"（语法糖指的是某种语法的原理相同的便捷替代品）。
*/

/*
泛型
*/
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

/*
impl <特性名> for <所实现的类型名>

同一个类可以实现多个特性，每个 impl 块只能实现一个
*/

/*
特性与接口的不同：
接口只能规范方法而不能定义方法，但特性可以定义方法作为默认方法，
因为是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法
*/

// 命令行输入
fn 命令行输入() {
    use std::io::stdin;
    let mut str_buf = String::new();

    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}

fn 文件读取() {
    use std::fs;
    let text = fs::read_to_string("e:/my-projects/rust-demos/hello-rust/src/main.rs").unwrap();
    println!("{}", text);
}

fn 文件写入() {
    use std::fs;
    fs::write("e:\\temp\\text.txt", "FROM RUST PROGRAM")
        .unwrap();
}


fn vector1() {
    let mut v1 = vec![1, 2, 4, 8];
    let mut v2 = vec![2, 2, 2, 2];
    v1.append(&mut v2);
    println!("{:?}", v1);

    /*
        因为向量的长度无法从逻辑上推断，get 方法无法保证一定取到值，
        所以 get 方法的返回值是 Option 枚举类，有可能为空。
        这是一种安全的取值方法，但是书写起来有些麻烦。
        如果你能够保证取值的下标不会超出向量下标取值范围，你也可以使用数组取值语法：v1[2]
    */
}

mod second;
use second::ClassName;
fn 使用second() {
    let object = ClassName::new(1024);
    object.public_method();
}














