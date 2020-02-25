use std::thread;
use std::sync::mpsc::channel;
use demo::Test;

static NUM: i32 = 18;

fn coerce_static() -> i32 {
    NUM
}

// FnOnce() 只可以调用一次哦
fn tt<F>(f: F)
    where F: FnOnce() {
    f();
    // f(); 只可以调用一次，重复调用会报错
}


// Fn() 可以重复的调用，且状态不可的重复调用
fn apply<F>(f: F) 
    where F: Fn() {
    f();
    f();
}

// 所谓的FnMut是状态是闭包函数里面的状态有变化，所有是mut可变的
fn do_twice<F>(mut func: F)
    where F: FnMut() {
    func();
    func();
}

fn test() {
    println!("test hello world");
}

fn main() {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        sender.send("hello world").unwrap();
    });
    for s in receiver {
        println!("{}", s);
    }

    let t = Test {
        name: 2
    };

    apply(test);
    tt(test);

    let mut x = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }
    println!("x = {}", x);
    assert_eq!(5, x);

    println!("{}", t.name);
    println!("{}", coerce_static());
    println!("Hello, world!");
}
