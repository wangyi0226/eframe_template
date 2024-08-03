pub struct AutoComplete {
    keywords: std::collections::BTreeSet<&'static str>,
}

impl AutoComplete {
    pub fn lua() -> Self {
        Self {
            keywords: [
                "and",
                "break",
                "do",
                "else",
                "elseif",
                "end",
                "false",
                "for",
                "function",
                "goto",
                "if",
                "in",
                "local",
                "nil",
                "not",
                "or",
                "repeat",
                "return",
                "then",
                "true",
                "until",
                "while",
              ]
            .into_iter()
            .collect(),
        }
    }
    pub fn auto_complete(&self, text: &mut String, index: usize) -> usize {
        let mut words: std::collections::BTreeSet<&str> = self.keywords.clone();
        let mut index_s = None;
        for (ci, (bi, ch)) in text.char_indices().enumerate() {
            if ci == index {
                if let Some(index_s) = index_s {
                    let word = &text[index_s..bi];
                    let mut replace = word;
                    let mut m = false;
                    for w in &words {
                        if w.starts_with(word) {
                            if m {
                                //匹配多个不自动补全
                                return 0;
                            }
                            replace = w;
                            m = true;
                        }
                    }
                    let p = &text[..index_s];
                    let ni = replace.char_indices().count() + p.char_indices().count();
                    *text = format!("{}{}{}", p, replace, &text[bi..]);
                    return ni;
                } else {
                    return 0;
                }
            } else if !ch.is_ascii_alphanumeric() && ch != '_' {
                if let Some(index_s) = index_s {
                    words.insert(&text[index_s..bi]);
                }
                index_s = None;
            } else if index_s.is_none() {
                index_s = Some(bi)
            }
        }
        0
    }
}
