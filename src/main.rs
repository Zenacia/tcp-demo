use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

/*
  处理TCP连接
 */
fn handle_client(mut stream: TcpStream) {
    // 创建50字节的buffer
    let mut data = [0 as u8; 50];
    // 将stream里的数据复制到data中，每次50字节，直到读完。使用模式匹配处理异常
    while match stream.read(&mut data) {
        Ok(_size) => {
            // 将客服端发来的信息打印
            println!("Got message: {}", from_utf8(&data[..]).unwrap());

            let mut echo = [0 as u8; 50];
            // 将数据复制到新的内存echo中，避免下面write时造成死循环
            echo.copy_from_slice(&data[0..]);
            // 向客户端返回原文
            stream.write(&echo).unwrap();
            true
        },
        // 错误处理
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            // 关闭出错的连接
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // 创建TCP服务端，监听来自3333端口所有IP的请求
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    // 调用listener.incoming得到一个永不结束的迭代器，每当有新的连接时便触发一次迭代，值不为None
    for stream in listener.incoming() {
        // 可能抛出异常，模式匹配
        match stream {
            // 处理正确逻辑
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                // 创建新的线程，处理逻辑在闭包中进行
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            // 处理stream连接断开等异常
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    // 关闭TCP服务端
    drop(listener);
}