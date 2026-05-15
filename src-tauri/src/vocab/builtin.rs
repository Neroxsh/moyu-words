pub static BUILTIN_BOOKS: &[BuiltinBook] = &[
    BuiltinBook {
        title: "初中词书",
        filename: "1 初中-乱序.txt",
        remote_path: "1%20%E5%88%9D%E4%B8%AD-%E4%B9%B1%E5%BA%8F.txt",
    },
    BuiltinBook {
        title: "高中词书",
        filename: "2 高中-乱序.txt",
        remote_path: "2%20%E9%AB%98%E4%B8%AD-%E4%B9%B1%E5%BA%8F.txt",
    },
    BuiltinBook {
        title: "四级词书",
        filename: "3 四级-乱序.txt",
        remote_path: "3%20%E5%9B%9B%E7%BA%A7-%E4%B9%B1%E5%BA%8F.txt",
    },
    BuiltinBook {
        title: "六级词书",
        filename: "4 六级-乱序.txt",
        remote_path: "4%20%E5%85%AD%E7%BA%A7-%E4%B9%B1%E5%BA%8F.txt",
    },
    BuiltinBook {
        title: "考研词书",
        filename: "5 考研-乱序.txt",
        remote_path: "5%20%E8%80%83%E7%A0%94-%E4%B9%B1%E5%BA%8F.txt",
    },
    BuiltinBook {
        title: "托福词书",
        filename: "6 托福-乱序.txt",
        remote_path: "6%20%E6%89%98%E7%A6%8F-%E4%B9%B1%E5%BA%8F.txt",
    },
    BuiltinBook {
        title: "SAT词书",
        filename: "7 SAT-乱序.txt",
        remote_path: "7%20SAT-%E4%B9%B1%E5%BA%8F.txt",
    },
];

pub struct BuiltinBook {
    pub title: &'static str,
    pub filename: &'static str,
    pub remote_path: &'static str,
}

pub fn builtin_book_url(book: &BuiltinBook) -> String {
    let base = std::env::var("MOYU_VOCAB_BASE")
        .unwrap_or_else(|_| {
            std::env::var("MOYU_GITHUB_RAW_BASE")
                .unwrap_or_else(|_| "https://raw.githubusercontent.com/KyleBing/english-vocabulary/master".to_string())
        })
        .trim_end_matches('/')
        .to_string();
    format!("{}/{}", base, book.remote_path.trim_start_matches('/'))
}