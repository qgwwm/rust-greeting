/// rust learn
use std::fs;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::error::Error;

mod util;

use util::util::say;

/// 添加数据
fn add_hello(x: i32, y: i32) {
    println!("the add result is {}", x + y);
}

/// obtain os size
fn obtain_size() -> i32 {
    return 32;
}

/// learn if
fn test_if() -> i32 {
    let a: i32 = 1;
    let b: i32 = 2;
    let max = if a > b { a } else { b };
    return max;
}

/// learn while
fn test_while(x: i32) {
    let mut y: i32 = 1;
    while x > y {
        println!("temp y is {}", y);
        y += 1;
    }
}

/// learn iter
fn test_iter() {
    let a = [10, 12, 23, 23, 45];
    for i in a.iter() {
        println!("the iter is {}", i);
    }
}

/// learn loop
fn test_loop(x: i32) {
    let mut y: i32 = 1;
    loop {
        if y < x {
            println!("loop index is {}", y);
            y += 1;
        } else {
            println!("loop index is {},end", y);
            break;
        }
    }
}

/// learn owner
fn test_owner() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    s1.push_str("_world");
    let s2 = &s1;
    println!("owner test {}", s2);
}

/// test struct
///
#[derive(Debug)]

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn obtain_area(&self) -> i32 {
        return self.width * self.height;
    }

    fn obtain_grith(&self) -> i32 {
        return self.width * 2 + self.height * 2;
    }

    fn create(width: i32, height: i32) -> Rectangle {
        return Rectangle {
            width: width,
            height: height,
        };
    }
}

/// test trait
trait Describe {
    fn describe(&self) {
        println!("trait impl");
    }
}

impl Describe for Rectangle {
    fn describe(&self) {
        println!("trait impl {}-{}", self.width, self.height);
    }
}

fn obtain_area(obj: impl Describe) {
    obj.describe();
}

/// read file
fn read_file() {
    let path =
        String::from("/mnt/d/develop/workspcace/vscode/rust/rust-greeting/greeting/src/main.rs");
    let mut buff = [0u8; 20];
    let mut file = fs::File::open(&path).unwrap();
    file.read(&mut buff).unwrap();
    println!("{:?}", buff);
    file.read(&mut buff).unwrap();
    println!("{:?}", buff);
    let content = fs::read_to_string(&path).unwrap();
    println!("context :{}", content)
}

// read file bu link
fn read_file_link() -> Result<String,io::Error>{
    let path =
        String::from("/mnt/d/develop/workspcace/vscode/rust/rust-greeting/greeting/src/main.rs");
    let mut content = String::new();
    let result = fs::File::open(&path)?.read_to_string(&mut content)?;
    Ok(content)
}

/// test thread
fn test_thread() {
    for i in 1..4 {
        println!("other thread print :{}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    let price: i64 = 78;
    let name: &str = "漆国武";
    let names = [30, 10];
    add_hello(12, 23);
    obtain_size();
    println!("Hello, world! {},{}", name, names[1]);
    println!("the {} price is {}", "apple", price);
    println!("the max number is {}", test_if());
    test_while(4);
    test_iter();
    test_loop(4);
    test_owner();

    let rect = Rectangle {
        width: 12,
        height: 23,
    };
    println!("the Rectangle is {:#?}", rect);
    println!("the Rectangle area is {}", rect.obtain_area());
    println!("the Rectangle grith {}", rect.obtain_grith());
    let rect1 = Rectangle::create(34, 65);
    println!("the Rectangle is {:#?}", rect1);
    rect1.describe();
    obtain_area(rect1);
    read_file();

    thread::spawn(test_thread);
    let num = 23;
    let (tx, rx) = mpsc::channel();
    let inc = thread::spawn(move || {
        for i in 0..10 {
            println!("inc thread print :{}-{}", i, num);
            thread::sleep(Duration::from_millis(1));
            tx.send(1).unwrap();
        }
    });

    for i in 1..4 {
        println!("main thread print :{}", i);
        thread::sleep(Duration::from_millis(1));
    }
    let received = rx.recv().unwrap();
    println!("received data is :{}", received);

    say();

    inc.join().unwrap();

    let linkFile = read_file_link();
    println!("read linked file : {:#?}", linkFile);
    match linkFile {
        Ok(fileString) => {
            println!("read linked file : {:#?}", fileString);
            ()   
        },
        _ => (),
    }
}
