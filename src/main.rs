mod db;

use db::Utils;
use rusqlite::Connection;

use db::Book;
use db::Keyword;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Connection::open("library.db")?;

    db.init()?;
    // _insert(&db);
    let results = db.query_keyword(vec![Keyword::Tag("编程".into())])?;
    for result in results {
        println!("{result}\n");
    }

    Ok(())
}

fn _insert(db: &Connection) {
    db.insert(Book::new(
        9787111606420,
        "深入浅出Rust",
        "范长春,F001",
        "计算机科学,编程语言,rust",
        "zh-CN",
        8900,
    ))
    .unwrap();
    db.insert(Book::new(
        9787115390592,
        "C Primer Plus 6th",
        "Stephen Prata,姜佑",
        "计算机科学,编程语言,c",
        "zh-CN",
        7740,
    ))
    .unwrap();
}
