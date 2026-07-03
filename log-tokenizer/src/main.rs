// 1.3 — Lifetime Mechanics & Elision Rules
// Exercise: Zero-Copy Audit-Log Tokenizer
// Spec: see §4 of "1.3 Lifetime mechanics & elision rules.md" in the notes vault.

struct Token<'line> {
    word: &'line str,
    kind: TokenKind,
}

#[derive(Copy, Clone)]
enum TokenKind {
    IpAddress,
    Email,
    Generic,
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut token_vec: Vec<Token> = vec![];
    for word in line.split_whitespace() {
        if word.contains('@') {
            token_vec.push(Token {
                word,
                kind: TokenKind::Email,
            });
        } else if is_ip_addr(word) {
            token_vec.push(Token {
                word,
                kind: TokenKind::IpAddress,
            });
        } else {
            token_vec.push(Token {
                word,
                kind: TokenKind::Generic,
            });
        }
    }

    token_vec
}

fn is_ip_addr(s: &str) -> bool {
    let mut count = 0;
    for part in s.split('.') {
        if part.is_empty() || !part.chars().all(|v| v.is_ascii_digit()) {
            return false;
        }
        count += 1;
    }

    if count != 4 {
        return false;
    }

    true
}

#[derive(Debug)]
struct Match<'line, 'rule> {
    word: &'line str,
    rule: &'rule str,
}

fn flag_sensitive<'line, 'rule>(
    tokens: &[Token<'line>],
    ruleset: &[&'rule str],
) -> Vec<Match<'line, 'rule>> {
    let mut matcher: Vec<Match> = vec![];

    for token in tokens {
        for rule in ruleset {
            if token.word.contains(rule) {
                matcher.push(Match {
                    word: token.word,
                    rule,
                });
            }
        }
    }

    matcher
}

fn main() {
    let ruleset = vec!["root", "admin"];
    let mut extracted_str;

    {
        let line = String::from("user admin src 10.0.0.5 contact root@internal");
        let sensitives_in_line = flag_sensitive(&tokenize(&line), &ruleset);

        // rule with 'static lives while word with lifetime 'line dies due to scope

        extracted_str = sensitives_in_line[0].rule; // works. wouldnt work if i merged output to a
        // forced 'a lifetime, that would bring down 'static lifetime of rule to shorter lived
        // lifetime of 'line

        // extracted_str = sensitives_in_line[0].word; // error[E0597]: `line` does not live long enough
    }
    println!("{extracted_str}");
}
