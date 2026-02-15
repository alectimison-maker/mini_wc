/// CountMode：选择输出模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountMode {
    All,
    Lines,
    Words,
    Bytes,
    Chars,
}

/// FileStats：统计结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStats {
    pub lines: usize, // 行数
    pub words: usize, // 词数（按空白分割）
    pub bytes: usize, // 字节数（u8 数组长度）
    pub chars: usize, // 字符数（Unicode 标量数量）
}

impl FileStats {
    /// from_bytes：从文件内容的字节切片生成统计
    pub fn from_bytes(bytes: &[u8]) -> Self {
        // bytes: &[u8]：只借用，不拿走 Vec<u8> 的所有权

        let byte_count = bytes.len();
        // len()：切片长度
        // 这就是“字节数”

        // 这里把 bytes 尝试当成 UTF-8 文本
        // 如果不是 UTF-8（比如二进制），我们用 lossy 转换，保证不 panic
        let text = String::from_utf8_lossy(bytes);
        // from_utf8_lossy：返回 Cow<str>（写时拷贝）
        // 好处：如果本来就是有效 UTF-8，就不复制；无效则替换并生成新字符串

        let line_count = text.lines().count();
        // lines()：按 '\n' 分割迭代器（不包含换行符）
        // count()：迭代器消费并计数

        let word_count = text.split_whitespace().count();
        // split_whitespace：按空白分割
        // count：计数

        let char_count = text.chars().count();
        // chars()：按 Unicode 标量迭代
        // 注意：不是“字节”，也不是“用户可见字形簇”，但对学习足够

        //返回值
        Self {
            lines: line_count,
            words: word_count,
            bytes: byte_count,
            chars: char_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_basic() {
        let input = b"hello rust\nrust world\n";
        // b"..."：字节串字面量，类型是 &[u8; N]

        let s = FileStats::from_bytes(input);
        assert_eq!(s.lines, 2);
        assert_eq!(s.words, 4);
        assert_eq!(s.bytes, input.len());
        assert_eq!(s.chars, String::from_utf8_lossy(input).chars().count());
    }

    #[test]
    fn mode_is_copy() {
        let m1 = CountMode::Lines;
        let m2 = m1; // Copy：不 move
        assert_eq!(m1, m2);
    }
}
