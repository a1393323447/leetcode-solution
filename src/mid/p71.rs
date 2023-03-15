struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path = path.as_bytes();
        let mut res = vec![];
        let mut token = String::new();

        let mut stream = path.iter().peekable();

        while let Some(&ch) = stream.next() {
            let ch = ch as char;
            match ch {
                '/' => {
                    token.push('/');
                    while let Some(&&ch) = stream.peek() {
                        if ch == b'/' {
                            stream.next();
                        } else {
                            break;
                        }
                    }
                    match token.as_str() {
                        "./" => token.clear(),
                        "../" => {
                            if let Some(s) = res.pop() {
                                if s == "/" {
                                    res.push(s);
                                }
                            }
                            token.clear();
                        }
                        _ => {
                            res.push(token.clone());
                            token.clear();
                        }
                    }
                }
                _ => token.push(ch),
            }
        }

        if token == ".." {
            if let Some(s) = res.pop() {
                if s == "/" {
                    res.push(s);
                }
            }
        } else if token == "." {
        } else if !token.is_empty() {
            res.push(token.clone());
        }

        if let Some(token) = res.last_mut() {
            if token.ends_with("/") {
                token.pop();
                if token.is_empty() {
                    res.pop();
                }
            }
        }

        if res.is_empty() {
            "/".into()
        } else {
            res.join("")
        }
    }

    pub fn simplify_path2(path: String) -> String {
        let mut stk = vec![];
        path.split('/').for_each(|s| match s {
            "." | "" => (),
            ".." => {
                stk.pop();
            }
            _ => stk.push(s),
        });
        "/".to_string() + &stk.join("/")
    }
}
