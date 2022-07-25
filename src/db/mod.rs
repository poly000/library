use std::fmt::Display;

use library::sql;
use rusqlite::{params, Connection, Result};

pub trait Utils {
    fn query_keyword(&self, keyword: Vec<Keyword>) -> Result<Vec<Book>>;
    fn query_isbn(&self, isbn: u64) -> Result<Book>;
    fn insert(&self, book: Book) -> Result<()>;
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
                    isbn: row.get(0)?,
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
                isbn: isbn,
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
        Book {
            isbn,
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
                price integer not null
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
    pub isbn: u64,
    pub name: String,
    pub authors: String,
    pub tags: String,

    /// view http://www.lingoes.net/zh/translator/langcode.htm for language codes
    pub lang: String,

    /// 100x price, store 0.01 as 1 e.g.
    pub price: u64,
}

pub enum Keyword {
    Author(String),
    Name(String),
    Tag(String),
    Lang(String),
}

impl Book {
    pub fn new(isbn: u64, name: &str, authors: &str, tags: &str, lang: &str, price: u64) -> Self {
        Book {
            isbn,
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

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let isbn = self.isbn;
        let authors = &self.authors;
        let tags = &self.tags;
        let name = &self.name;
        let price = self.price as f64 / 100.0;

        f.write_fmt(format_args!(
            r#"Name: {name}
Authors: {authors}
ISBN: {isbn}
Tags: {tags}
Price: {price}"#
        ))?;

        Ok(())
    }
}
