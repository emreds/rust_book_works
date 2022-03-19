use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

trait MessageType {
    fn print_type(&self);
}

impl MessageType for Message {
    fn print_type(&self) {
        println!("This is our one {:?}", self);
    }
}

impl Message {
    fn call(&self) {
        println!("Das ist ein(e) {:?}", self);
    }

    fn is_quit(&self) -> bool {
        if let &Message::Quit = self {
            true
        }
        else {
            false
        }
    }
}

fn main() {

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));   
    
    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));

    let m = Message::Write(String::from("Hello world!"));
    m.call();
    m.is_quit();
    m.print_type();
}   
