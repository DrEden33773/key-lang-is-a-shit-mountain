//! 映射表

/// 返回符号优先级
pub fn prec(x:&[u8])-> u8 {
  match x {
    b"*" | b"%" | b"/" => 11, 
    b"+" | b"-" => 10, 
    b"<<"|b">>" => 9,
    b"&" => 8,
    b"^" => 7,
    b"|" => 6,
    b"=="|b"!="|b"<"|b">"|b"<="|b">=" => 5,
    b"&&" => 4,
    b"||" => 3,
    b"="|b"+="|b"-="|b"*="|b"/="|b"%="|b"&="|b"|="|b"^="|b"<<="|b">>=" => 2,
    b"," => 1, 
    _=> 0
  }
}


/// 转义符表
pub fn escape(c:u8)-> u8 {
  match c {
    b'n'=> b'\n',
    b'r'=> b'\r',
    b't'=> b'\t',
    b'\\'=> b'\\',
    b'0'=> 0,
    _=> 255
  }
}


/// 将ks声明的类型映射给Rust
/// 
/// 只是使用类型对比并不使用数值，因此使用空指针是安全的
pub fn kstype(s:&[u8])-> crate::ast::KsType {
  use crate::ast::{
    KsType,Litr::*
  };
  use std::ptr::null_mut as null;
  let t = match s {
    b"Uint"=> Uint(0),
    b"Int"=> Int(0),
    b"Float"=> Float(0.0),
    b"Bool"=> Bool(false),
    b"Str"=> Str(null()),
    b"Array"=> Array(null()),
    b"Buffer"=> Buffer(null()),
    b"Func"=> Func(null()),
    _=> {
      return KsType::Custom(crate::intern(s));
    }
  };
  KsType::Primitive(std::mem::discriminant(&t))
}
