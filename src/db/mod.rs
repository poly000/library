use library::sql;
use rusqlite::{params, Connection, Result};

pub trait Utils {
    fn query_keyword(&self, keyword: Vec<Keyword>) -> Result<Vec<Book>>;
    fn query_isbn(&self, isbn: u64) -> Result<Book>;
    fn insert(&self, isbn: u64, book: Book) -> Result<()>;
    fn init(&self) -> Result<()>;
}

impl Utils for Connection {
    fn query_keyword(&self, keywords: Vec<Keyword>) -> Result<Vec<Book>> {
        let keyword = keywords
            .into_iter()
            .map(|key| String::from(key))
            .reduce(|a, b| a + " and " + &b)
            .ok_or(rusqlite::Error::InvalidQuery)?;

        let sql: String = sql!(
            "--sql
            select * from books where "
        )
        .to_owned()
            + &keyword;
        let mut stmt = self.prepare(&sql)?;

        let mut books = Vec::new();
        for book in stmt.query_map([], |row| {
            Ok({
                let book = Book {
                    name: row.get(1)?,
                    authors: row.get(2)?,
                    tags: row.get(3)?,
                    lang: row.get(4)?,
                    price: row.get(5)?,
                };
                book
            })
        })? {
            books.push(book?);
        }

        Ok(books)
    }

    fn query_isbn(&self, isbn: u64) -> Result<Book, rusqlite::Error> {
        let mut stmt = self.prepare(sql!(
            "--sql
           select * from books where isbn = ? 
       "
        ))?;

        let mut rows = stmt.query(params![isbn])?;

        match rows.next()? {
            Some(row) => Ok(Book {
                name: row.get(1)?,
                authors: row.get(2)?,
                tags: row.get(3)?,
                lang: row.get(4)?,
                price: row.get(5)?,
            }),
            None => Err(rusqlite::Error::InvalidQuery),
        }
    }

    fn insert(
        &self,
        isbn: u64,
        Book {
            name,
            authors,
            tags,
            lang,
            price,
        }: Book,
    ) -> Result<()> {
        self.execute(
            sql!(
                "--sql
                insert into books (isbn, name, authors, tags, lang, price) Values (?1, ?2, ?3, ?4, ?5, ?6)
    "
            ),
            (isbn, name, authors, tags, lang, price),
        )?;
        Ok(())
    }

    fn init(&self) -> Result<()> {
        self.execute(
            sql!(
                "--sql
            create table if not exists books (
                isbn integer primary key,
                name text NOT NULL,
                authors text not null,
                tags text not null,
                lang text not null,
                price real not null
            )
        "
            ),
            (),
        )?;
        Ok(())
    }
}

#[derive(Debug)]
/// both of authors and tags are splitted by ","
pub struct Book {
    pub name: String,
    pub authors: String,
    pub tags: String,

    /// view http://www.lingoes.net/zh/translator/langcode.htm for language codes
    pub lang: String,

    pub price: f64,
}

pub enum Keyword {
    Author(String),
    Name(String),
    Tag(String),
    Lang(String),
}

impl Book {
    pub fn new(name: &str, authors: &str, tags: &str, lang: &str, price: f64) -> Self {
        Book {
            name: name.into(),
            authors: authors.into(),
            tags: tags.into(),
            lang: lang.into(),
            price,
        }
    }
}

impl From<Keyword> for String {
    fn from(key: Keyword) -> Self {
        match key {
            Keyword::Author(s) => format!("authors like '%{s}%'"),
            Keyword::Lang(lang) => format!("lang like '%{lang}%'"),
            Keyword::Name(name) => format!("name like '%{name}%'"),
            Keyword::Tag(tag) => format!("tags like '%{tag}%'"),
        }
    }
}
