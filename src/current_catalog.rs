// use std::env;
// use std::fs;
use std::path::Path;

use crate::commands;
use crate::commands::CommandError;

#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    Catalog,
    File,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FilerFile {
    pub name: String,
    pub file_type: FileType,
}

pub enum DirectoryMoveType {
    Prev,
    Entry,
    Home,
    Root,
}

#[derive(Debug)]
pub struct CurrentCatalog {
    pub path: String,
    pub show_hidden_files: bool,
    pub pointer: usize,
    pub unhidden_files_count: usize,
    pub hidden_files_count: usize,
    pub unhidden_files: Vec<FilerFile>,
    pub hidden_files: Vec<FilerFile>,
}

impl CurrentCatalog {
    pub fn new(
        dir_set: DirectoryMoveType,
        old_path: Option<String>,
        old_catalog: Option<CurrentCatalog>,
    ) -> Result<Self, CommandError> {
        let path: String = match dir_set {
            DirectoryMoveType::Home => commands::get_home_dir(),
            DirectoryMoveType::Root => String::from("/"),
            DirectoryMoveType::Entry => {
                if let Some(value) = old_catalog {
                    match commands::entry_to_dir(&value) {
                        Ok(new_path) => new_path,
                        Err(entry_error) => match entry_error {
                            CommandError::VoidDir => return Ok(value),
                            CommandError::GetFile => {
                                eprintln!("Error: entry_to_dir -> GetFile");
                                return Ok(value);
                            }
                            // CommandError::DirEntry(..) => return value,
                            other => {
                                eprintln!("{:?}", other);
                                return Ok(value);
                            }
                        },
                    }
                } else {
                    return Err(CommandError::Other(
                        "Error: entry_to_dir -> old_catalog not provided".to_string(),
                    ));
                }
            }
            DirectoryMoveType::Prev => {
                if let Some(value) = old_path {
                    commands::get_parent_path(value)
                } else {
                    panic!("Ошибка обработки Prev");
                }
            }
        };

        let pointer: usize = 0;

        let (unhidden_files, hidden_files) = commands::get_files(&Path::new(&path))?;

        let (unhidden_files_count, hidden_files_count) = (unhidden_files.len(), hidden_files.len());

        Ok(Self {
            path: path,
            show_hidden_files: true,
            pointer: pointer,
            unhidden_files_count,
            hidden_files_count,
            unhidden_files,
            hidden_files,
        })
    }
}

impl CurrentCatalog {
    pub fn next(&mut self) {
        if self.show_hidden_files {
            if self.pointer < (self.unhidden_files_count + self.hidden_files_count) - 1 {
                self.pointer = self.pointer + 1;
            }
        } else {
            if self.pointer < self.unhidden_files_count - 1 {
                self.pointer = self.pointer + 1
            };
        }
    }

    pub fn prev(&mut self) {
        if self.pointer > 0 {
            self.pointer = self.pointer - 1;
        }
    }

    pub fn set_hidden_flag(&mut self) {
        if self.show_hidden_files {
            self.show_hidden_files = false;
        } else {
            self.show_hidden_files = true;
        };
        self.pointer = 0;
    }
}

impl CurrentCatalog {
    pub fn print_files(&self) -> String {
        let file_list: Vec<&FilerFile> = if self.show_hidden_files {
            self.hidden_files
                .iter()
                .chain(self.unhidden_files.iter())
                .collect()
        } else {
            self.unhidden_files.iter().collect()
        };

        let mut print_files: String = String::new();

        for (index, file) in file_list.iter().enumerate() {
            let file_name: String = if self.pointer == index {
                format!("> {}\r\n", file.name)
            } else {
                format!("{}\r\n", file.name)
            };
            print_files.push_str(&file_name);
        }

        return print_files;
    }
}
