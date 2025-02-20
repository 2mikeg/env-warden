use crate::db::Manager;
use clap::{Parser, Subcommand};
use inquire::{MultiSelect, Text};
use std::fmt::Display;

#[derive(Parser, Debug)]
#[command(
    name = "env-warden",
    version = "alpha",
    about = "Ejemplo de CLI con subcomandos"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[clap(skip)]
    manager: Manager,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add,
    Get,
    Prune,
    List,
}

pub struct SearchOptType {
    pub name: String,
    pub value: String,
}

impl Display for SearchOptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

impl SearchOptType {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
    pub fn repr_str(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

impl Cli {
    pub fn execute(&mut self, m: Manager) {
        self.manager = m;

        match &self.command {
            Commands::Get => {
                let s = self.manager.search().unwrap();
                let ops: Vec<SearchOptType> = s
                    .iter()
                    .map(|(_, vex)| SearchOptType::new(vex.0.clone(), vex.1.clone()))
                    .collect();

                let o = MultiSelect::new("Select:", ops).prompt().unwrap();

                let opt_str = o
                    .iter()
                    .map(|o| o.repr_str())
                    .collect::<Vec<String>>()
                    .join("\n");

                eprint!("{}", opt_str);
            }
            Commands::Add => {
                let var_name = Text::new("Variable name:").prompt().unwrap();
                let var_value = Text::new("Variable value:").prompt().unwrap();

                self.manager.insert(&var_name, &var_value).unwrap();
            }
            Commands::Prune => {
                let y_n = Text::new("Are you sure? (y/n):").prompt().unwrap();
                if y_n == "y" {
                    self.manager.prune().unwrap();
                } else {
                    eprint!("Aborted");
                }
            }
            Commands::List => {
                let s = self.manager.search().unwrap();
                for (k, v) in s {
                    println!("{}: {} = {}", k, v.0, v.1);
                }
            }
        }
    }
}
