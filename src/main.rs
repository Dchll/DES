#![feature(ascii_char)]

use crate::ip::ip;

mod ip;

fn main() {
    println!("DES 加密");
    // 64位密钥
    let mut key = Vec::<u8>::with_capacity(64);
    // 64位数据
    let mut data = Vec::<u8>::with_capacity(64);

    // 置换
    // let ip_data = ip(&data);

    // 16轮运算

    // 逆置换
    
    
    println!("{:?}",'9'.to_digit(2));
}
