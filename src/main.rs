use inquire::ui::{IndexPrefix, RenderConfig};
use inquire::Select;
use itertools::Itertools;
use std::process::Command;

fn filter_with_index(filter: &str, _: &&str, string_value: &str, index: usize) -> bool {
    let filter = filter.to_lowercase();

    (index + 1).to_string().contains(&filter) || string_value.to_lowercase().contains(&filter)
}

fn main() {
    // Configure inquire crate
    let render_config = RenderConfig::default().with_option_index_prefix(IndexPrefix::SpacePadded);
    inquire::set_global_render_config(render_config);

    // Select the context
    let output = Command::new("kubectx").output().unwrap();
    let content = String::from_utf8(output.stdout).unwrap();
    let content = content.trim().split('\n').collect_vec();
    let context = Select::new("Select the cluster:", content)
        .with_filter(&filter_with_index)
        .prompt()
        .expect("There was an error, please try again");

    // Select the namespace
    let content = vec!["domestika", "frontend"];
    let namespace = Select::new("Select the namespace:", content)
        .with_filter(&filter_with_index)
        .prompt()
        .expect("There was an error, please try again");

    // Select the reviewapp
    let output = Command::new("kubectl")
        .arg("--context")
        .arg(context)
        .arg("get")
        .arg("pods")
        .arg("-n")
        .arg(namespace)
        .arg("-o")
        .arg("jsonpath='{.items[*].metadata.labels.branch_slug}'")
        .output()
        .unwrap();

    let content = String::from_utf8(output.stdout).unwrap().replace('\'', "");
    let content = content.trim().split(' ').unique().collect_vec();
    let reviewapp = Select::new("Select the review app:", content)
        .with_filter(&filter_with_index)
        .with_page_size(15)
        .prompt()
        .expect("There was an error, please try again");

    // Select the container
    let output = Command::new("kubectl")
        .arg("--context")
        .arg(context)
        .arg("get")
        .arg("pods")
        .arg("-l")
        .arg(format!("branch_slug={}", reviewapp))
        .arg("-n")
        .arg(namespace)
        .arg("-o")
        .arg("name")
        .output()
        .unwrap();

    let content = String::from_utf8(output.stdout)
        .unwrap()
        .replace("pod/", "");
    let content = content.trim().split('\n').collect_vec();
    let container = Select::new("Select the container:", content)
        .with_filter(&filter_with_index)
        .with_page_size(10)
        .prompt()
        .expect("There was an error, please try again");

    // Enter the container
    println!("Connecting...");

    Command::new("kubectl")
        .arg("--context")
        .arg(context)
        .arg("exec")
        .arg("-it")
        .arg("-n")
        .arg(namespace)
        .arg(container)
        .arg("--")
        .arg("bash")
        .arg("-l")
        .spawn()
        .expect("Command failed to start")
        .wait()
        .expect("Something wrong happened while waiting for command to finninsh");
}
