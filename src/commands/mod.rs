use std::env;
use std::fs;
use std::path::Path;

use crate::current_catalog::*;
use crate::current_catalog::{FileType, FilerFile};

#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandError {
    VoidDir,
    GetFile,
    DirEntry(String),
    FileTypeUnknow,
    FileNameInvalid,
    Other(String),
}

pub fn go_to_root() -> CurrentCatalog {
    CurrentCatalog::new(DirectoryMoveType::Root, None, None).unwrap()
}

pub fn go_to_home() -> CurrentCatalog {
    CurrentCatalog::new(DirectoryMoveType::Home, None, None).unwrap()
}

pub fn get_parent_path(path: String) -> String {
    if &path == "/" || &path.matches("/").count() == &1 {
        return String::from("/");
    }

    let last_index = match path.rfind("/") {
        Some(index) => index,
        None => panic!("Что-то не так в get_parent_path"),
    };

    path[0..last_index].to_string()
}

/*
 Functions for current_catalog
*/

pub fn entry_to_dir(current_catalog: &CurrentCatalog) -> Result<String, CommandError> {
    let files_list: Vec<&FilerFile> = if current_catalog.show_hidden_files {
        current_catalog
            .hidden_files
            .iter()
            .chain(current_catalog.unhidden_files.iter())
            .collect()
    } else {
        current_catalog.unhidden_files.iter().collect()
    };

    if files_list.is_empty() {
        return Err(CommandError::VoidDir);
    };

    let current_file = match files_list.get(current_catalog.pointer) {
        Some(file) => file,
        None => return Err(CommandError::GetFile),
    };

    match current_file.file_type {
        FileType::Catalog => {
            return Ok(format!("{}/{}", current_catalog.path, current_file.name));
        }
        FileType::File => return Ok(format!("{}", current_catalog.path)),
    };
}

pub fn get_home_dir() -> String {
    match env::home_dir() {
        Some(path) => path,
        None => panic!("Ошибка получения домашнего каталога!"),
    }
    .display()
    .to_string()
}

pub fn get_files(current_catalog: &Path) -> Result<(Vec<FilerFile>, Vec<FilerFile>), CommandError> {
    let catalog_read = match fs::read_dir(current_catalog) {
        Ok(directory) => directory,
        Err(_) => {
            return Err(CommandError::DirEntry(
                "get_files: Error reading directory".to_string(),
            ))
        }
    };

    let mut unhidden_files: Vec<FilerFile> = Vec::new();
    let mut hidden_files: Vec<FilerFile> = Vec::new();

    for file in catalog_read {
        let file = match file {
            Ok(file) => file,
            Err(_) => {
                return Err(CommandError::DirEntry(
                    "get_files: Iterator processing error".to_string(),
                ))
            }
        };

        let file_name = file
            .file_name()
            .into_string()
            .or(Err(CommandError::FileNameInvalid))?;

        let file_type: FileType = if let Ok(file_t) = file.file_type() {
            if file_t.is_file() {
                FileType::File
            } else {
                FileType::Catalog
            }
        } else {
            return Err(CommandError::FileTypeUnknow);
        };

        let file_struct: FilerFile = FilerFile {
            name: file_name.clone(),
            file_type: file_type,
        };

        if file_name.starts_with(".") {
            hidden_files.push(file_struct);
        } else {
            unhidden_files.push(file_struct);
        };
    }

    Ok((unhidden_files, hidden_files))
}
