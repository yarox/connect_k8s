use core::fmt;
use itertools::Itertools;
use std::io;
use std::{collections::HashMap, process::Command};

struct CommandResultChoice {
    choices: HashMap<String, String>,
}

impl fmt::Display for CommandResultChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_choices = self.choices.len();
        let width = num_choices.to_string().len();

        for (i, line) in self
            .choices
            .iter()
            .sorted_by_key(|x| x.0.parse::<u32>().unwrap())
        {
            writeln!(f, "{:>width$}. {}", i, line, width = width)
                .expect("Something went wrong while displaying CommandResultChoice");
        }

        Ok(())
    }
}

impl CommandResultChoice {
    fn new(content: &[&str]) -> Self {
        let mut choices = HashMap::new();

        for (i, line) in content.iter().enumerate() {
            choices.insert((i + 1).to_string(), String::from(*line));
        }

        CommandResultChoice { choices }
    }

    fn select(&self, prompt: &str) -> String {
        println!("{}", self);

        loop {
            println!("{}", prompt);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("unable to read input");

            let input = input.trim();

            match self.choices.get(input) {
                Some(value) => {
                    println!(">>> {}\n", value);
                    return value.to_string();
                }
                None => {
                    println!("Invalid selection. Try again.");
                    continue;
                }
            };
        }
    }
}

fn main() {
    // Select the context
    let output = Command::new("kubectx").output().unwrap();
    let content = String::from_utf8(output.stdout).unwrap();
    let content = content.trim().split('\n').collect_vec();

    let context_list = CommandResultChoice::new(&content);
    let context = context_list.select("Select the cluster to connect to: ");

    // Select the namespace
    let content = vec!["domestika", "frontend"];
    let namespace_list = CommandResultChoice::new(&content);
    let namespace = namespace_list.select("Select the namespace to connect to: ");

    // Select the reviewapp
    let output = Command::new("kubectl")
        .arg("--context")
        .arg(&context)
        .arg("get")
        .arg("pods")
        .arg("-n")
        .arg(&namespace)
        .arg("-o")
        .arg("jsonpath='{.items[*].metadata.labels.branch_slug}'")
        .output()
        .unwrap();

    let content = String::from_utf8(output.stdout).unwrap().replace('\'', "");
    let content = content.trim().split(' ').unique().collect_vec();

    let reviewapp_list = CommandResultChoice::new(&content);
    let reviewapp = reviewapp_list.select("Select the review app to connect to: ");

    // Select the container
    let output = Command::new("kubectl")
        .arg("--context")
        .arg(&context)
        .arg("get")
        .arg("pods")
        .arg("-l")
        .arg(format!("branch_slug={}", reviewapp))
        .arg("-n")
        .arg(&namespace)
        .arg("-o")
        .arg("name")
        .output()
        .unwrap();

    let content = String::from_utf8(output.stdout)
        .unwrap()
        .replace("pod/", "");
    let content = content.trim().split('\n').collect_vec();

    let container_list = CommandResultChoice::new(&content);
    let container = container_list.select("Select the container to connect to: ");

    // Enter the container
    Command::new("kubectl")
        .arg("--context")
        .arg(&context)
        .arg("exec")
        .arg("-it")
        .arg("-n")
        .arg(&namespace)
        .arg(&container)
        .arg("--")
        .arg("bash")
        .arg("-l")
        .spawn()
        .expect("Command failed to start")
        .wait()
        .expect("Something wrong happened while waiting for command to finninsh");
}
