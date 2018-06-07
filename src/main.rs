use std::io::prelude::*; // 没有这一行，下边编译不过
use std::net::TcpStream;

fn main() {
    let mut socket = TcpStream::connect("localhost:19890").unwrap();//连接baidu
    let _ = socket.write(b"GET / HTTP/1.0\n\n");  //获取发送网页

    let mut data = [0;128];
    let resp = socket.read(&mut data); //得到结果
    println!("{:?}",resp);


   for i in data.iter() {
     print!("{}",i);
   }
}
