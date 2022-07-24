mod db;

use rusqlite::Connection;
use db::Utils;

use db::Book;
use db::Keyword;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Connection::open("library.db")?;

    db.init()?;
    _insert(&db);
    
    Ok(())
}

fn _insert(db: &Connection) {
    db.insert(9787111606420, Book::new("深入浅出Rust", "范长春,F001", "计算机科学,编程语言,rust","zh-CN",89.0)).unwrap();
    db.insert(9787115390592, Book::new("C Primer Plus 6th", "Stephen Prata,姜佑", "计算机科学,编程语言,c","zh-CN", 77.4)).unwrap();
}