#[derive(Debug)]
pub struct Token {
    line: usize,
    col:  usize,
    head: usize,
    tail: usize,
    quote: bool,
}

pub fn format_item(file: &[char], item: Token) -> String {
    let slice = file[item.head..item.tail].iter().collect::<String>()
        .replace("\n", "\\n");
    let quote = if item.quote { "<quote>" } else { "" };
    format!("{:3},{:3}: {:20} {}",
            item.line, item.col, slice, quote)
}

pub fn lex(file: &[char]) -> Result<Vec<Token>, String> {
    let mut items = vec![];
    let mut pos = 0;
    let (mut line, mut col) = (0, 0);
    while let Some(&cc) = file.get(pos) {
        match cc {
            ';' => {
                while file[pos] != '\n' && pos < file.len() {
                    pos += 1;
                }
            }
            '\'' | '"' => {
                let quote_char = file[pos];
                let mut escaped = false;
                pos += 1;
                let start = pos;
                while escaped || file[pos] != quote_char {
                    escaped = !escaped && file[pos] == '\\';
                    pos += 1;
                    if pos >= file.len() {
                        return Err("Unexpected EOF; expected end of quote.".to_string());
                    }
                }
                items.push(Token{
                    quote: true,
                    head: start, tail: pos,
                    line: line, col: col,
                });
                pos += 1;
            }
            _ if cc.is_alphanumeric() => {
                let start = pos;
                while pos < file.len() && file[pos].is_alphanumeric() {
                    pos += 1;
                }
                items.push(Token{
                    quote: false,
                    head: start, tail: pos,
                    line: line, col: col,
                });
            }
            _ if !cc.is_whitespace() || cc == '\n' => {
                items.push(Token{
                    quote: false,
                    head: pos, tail: pos + 1,
                    line: line, col: col,
                });
                pos += 1;
            }
            _ => pos += 1,
        }
        if cc == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    Ok(items)
}
