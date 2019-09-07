use crate::Category;
use itertools::Itertools;
use snafu::Snafu;
use std::io::prelude::*;

#[derive(Debug, Snafu)]
pub enum GetCategoriesError {
    #[snafu(display("Io error {}", err))]
    IoError { err: std::io::Error },
    #[snafu(display("Error reading file: {}", err))]
    RetrieveFile { err: ReadFileError },
}

impl From<ReadFileError> for GetCategoriesError {
    fn from(err: ReadFileError) -> Self {
        Self::RetrieveFile { err }
    }
}
impl From<std::io::Error> for GetCategoriesError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError { err }
    }
}

#[derive(Debug)]
pub enum InputMethod {
    Lines,
    Stdin,
}

#[derive(Debug, Snafu)]
pub enum InputMethodParseError {
    #[snafu(display("Invalid input method: {}", method))]
    UnknownMethod { method: String },
}

impl std::str::FromStr for InputMethod {
    type Err = InputMethodParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "stdin" => Ok(Self::Stdin),
            "file" => Ok(Self::Lines),
            s => Err(InputMethodParseError::UnknownMethod {
                method: s.to_owned(),
            }),
        }
    }
}

impl InputMethod {
    pub fn get_categories(
        &self,
        file: &std::path::Path,
    ) -> Result<Vec<Category>, GetCategoriesError> {
        match self {
            Self::Lines => get_categories_from_lines(file).map_err(GetCategoriesError::from),
            Self::Stdin => get_categories_from_stdin().map_err(GetCategoriesError::from),
        }
    }
}

fn get_categories_from_stdin() -> Result<Vec<Category>, std::io::Error> {
    print!("Category count: ");
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    let category_count: usize = loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer)?;
        match buffer.trim().parse() {
            Err(e) => {
                print!("This number is invalid ({}), try again: ", e);
                std::io::stdout().flush()?;
            }
            Ok(n) => break n,
        }
    };
    let mut categories = Vec::with_capacity(category_count);
    for i in 0..category_count {
        let mut name = String::new();
        println!("Category {}", i);
        print!("\tCategory name: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut name)?;
        if name.ends_with('\n') {
            name.pop();
        };
        print!("\tCategory size: ");
        std::io::stdout().flush()?;
        let size: u64 = loop {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer)?;
            match buffer.trim().parse() {
                Err(e) => {
                    print!("\tThis number is invalid ({}), try again: ", e);
                    std::io::stdout().flush()?;
                }
                Ok(n) => break n,
            }
        };
        categories.push(Category { name, size })
    }
    Ok(categories)
}

#[derive(Debug, Snafu)]
pub enum ReadFileError {
    #[snafu(display("Io error: {}", err))]
    FileError { err: std::io::Error },
    #[snafu(display("File has invalid format"))]
    InvalidFormat,
    #[snafu(display("Error parsing number: {}", err))]
    InvalidNumber { err: std::num::ParseIntError },
}

impl From<std::io::Error> for ReadFileError {
    fn from(err: std::io::Error) -> Self {
        Self::FileError { err }
    }
}
impl From<std::num::ParseIntError> for ReadFileError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::InvalidNumber { err }
    }
}

fn get_categories_from_lines(path: &std::path::Path) -> Result<Vec<Category>, ReadFileError> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut categories = Vec::new();
    for mut chunk in reader.lines().chunks(2).into_iter() {
        let name = match chunk.next() {
            Some(n) => n?,
            None => return Err(ReadFileError::InvalidFormat),
        };
        let size = match chunk.next() {
            Some(a) => a?.parse()?,
            None => return Err(ReadFileError::InvalidFormat),
        };
        categories.push(Category { name, size });
    }
    Ok(categories)
}
