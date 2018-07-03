fn main() {
    let input: Vec<_> = std::fs::read_to_string("tests/lexer.txt").unwrap()
        .chars().collect();
    let items = lexer::lex(&input[..]);
    for item in items.unwrap().into_iter() {
        println!("{}", lexer::format_item(&input[..], item));
    }
}

mod lexer {
    #[derive(Debug)]
    enum ItemKind {
        Symbol,
        Ident,
        Quote,
    }

    pub struct Item {
        line: usize,
        col:  usize,
        kind: ItemKind,
        head: usize,
        tail: usize,
    }

    pub fn format_item(file: &[char], item: Item) -> String {
        let slice: String = file[item.head..item.tail].iter().collect();
        format!("{} - {:?} line: {}", slice, item.kind, item.line)
    }

    pub fn lex(file: &[char]) -> Result<Vec<Item>, String> {
        let mut items = vec![];
        let mut pos = 0;
        let (mut line, mut col) = (0, 0);
        while let Some(&cc) = file.get(pos) {
            match cc {
                '\'' | '"' => {
                    let start = pos;
                    let first_char = file[pos];
                    let mut escaped = false;
                    pos += 1;
                    while escaped || file[pos] != first_char {
                        escaped = !escaped && file[pos] == '\\';
                        pos += 1;
                        if pos >= file.len() {
                            return Err("Unexpected EOF; expected end of quote.".to_string());
                        }
                    }
                    items.push(Item{
                        kind: ItemKind::Quote,
                        head: start, tail: pos,
                        line: line, col: col,
                    });
                    pos += 1;
                },
                _ if cc.is_alphanumeric() => {
                    let start = pos;
                    while pos < file.len() && file[pos].is_alphanumeric() {
                        pos += 1;
                    }
                    items.push(Item{
                        kind: ItemKind::Ident,
                        head: start, tail: pos,
                        line: line, col: col,
                    });
                },
                _ if !cc.is_whitespace() => {
                    items.push(Item{
                        kind: ItemKind::Symbol,
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
}
