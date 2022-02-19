use std::{process::{Command, Stdio}, io::{self, Read}, fs::File};
use lazy_static::{lazy_static, __Deref};
use regex::{Regex, Captures};
use std::collections::HashMap;

use crate::{format::{Step, VarValue}, console};

fn run_command(cmd: &String) -> Result<(), io::Error> {
    console::log_command(&cmd);
    Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
    Ok(())
}

fn run_command_output(cmd: &String) -> Result<String, io::Error> {
    Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output().and_then(|val| Ok(String::from_utf8(val.stdout).expect("Could not convert command output to string")))
}

pub fn run(step: Step, state: HashMap<String, String>) -> HashMap<String, String> {
    lazy_static! {
        static ref VAR_REGEX: Regex = Regex::new(r"%\((?P<name>.*?)\)").unwrap();
    }
    let mut mut_state = state.clone();
    let replace_fn = |cap: &Captures| match state.get(&cap.name("name").map(|val| val.as_str()).unwrap_or("").to_string()) {
        Some(value) => value.to_owned(),
        None => "".to_string()
    };
    let replace_vars = |text: &String| VAR_REGEX.replace_all(text, replace_fn).deref().to_string();
    match step {
        Step::Info(message) => console::log_info(replace_vars(&message).as_str()),
        Step::Run(command) => run_command(&replace_vars(&command)).expect(format!("Failed to run command {}", command.as_str()).as_str()),
        Step::Confirm(prompt, if_yes, if_no) => {
            let result = console::question(prompt.as_str());
            match result.as_str() {
                    "y" => run_all_state(if_yes, &mut_state),
                    "n" => run_all_state(if_no, &mut_state),
                    &_ => run_all_state(if_no, &mut_state)
            };
        },
        Step::ConfirmFile(name, if_exists, if_not_exists) => match &mut File::open(name) {
            Ok(file) => {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => {
                        mut_state.insert("file_content".to_string(), content);
                        run_all_state(if_exists, &mut_state);
                        mut_state.remove("file_content");
                    },
                    Err(err) => {
                        console::log_err(err.to_string().as_str());
                        run_all_state(if_not_exists, &mut_state);
                    }
                }
            },
            Err(_e) => run_all(if_not_exists)
        },
        Step::Variable(name, value) => {mut_state.insert(name, match value {
            VarValue::Literal(val) => val,
            VarValue::CommandResult(cmd) => match run_command_output(&cmd) {
                Ok(str) => str,
                Err(_) => "".to_string()
            },
            VarValue::Cores() => match run_command_output(&String::from("nproc")) {
                Ok(str) => str,
                Err(_) => "".to_string()
            },
        });}
    };
    mut_state.to_owned()
}

pub fn run_all_state(steps: Vec<Step>, initial_state: &HashMap<String, String>) -> HashMap<String, String> {
    let mut state = initial_state.to_owned();
    for step in steps {
        state = run(step, state);
    }
    state.to_owned()
}

pub fn run_all(steps: Vec<Step>) {
    run_all_state(steps, &HashMap::new());
}