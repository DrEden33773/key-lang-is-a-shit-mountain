//! 映射表

/// 返回符号优先级
pub const fn prec(x: &[u8]) -> u8 {
  match x {
    b"-." | b"-:" => 16,
    b"::" | b"." => 15,
    b"(" | b"[" => 14, // 代指调用和索引
    // unary => 13
    b"*" | b"%" | b"/" => 12,
    b"+" | b"-" => 11,
    b"<<" | b">>" => 10,
    b"&" => 9,
    b"^" => 8,
    b"|" => 7,
    b"==" | b"!=" | b"<" | b">" | b"<=" | b">=" => 6,
    b"is" => 5,
    b"&&" => 4,
    b"||" => 3,
    b"=" | b"+=" | b"-=" | b"*=" | b"/=" | b"%=" | b"&=" | b"|=" | b"^=" | b"<<=" | b">>=" => 2,
    b"|>" => 1, // 管道运算符应当最靠后计算
    _ => 0,
  }
}
pub const PREC_UNARY: u8 = 13;

/// 转义符表
pub const fn escape(c: u8) -> u8 {
  match c {
    b'n' => b'\n',
    b'r' => b'\r',
    b't' => b'\t',
    b'\\' => b'\\',
    b'0' => 0,
    b'`' => b'`',
    b'{' => b'{',
    _ => 255,
  }
}

/// 将一个u8字符解析为u8
pub const fn char_to_u8(c: u8) -> Option<u8> {
  match c {
    b'0'..=b'9' => Some(c - b'0'),
    b'a'..=b'f' => Some(c - b'a' + 10),
    b'A'..=b'F' => Some(c - b'A' + 10),
    _ => None,
  }
}
