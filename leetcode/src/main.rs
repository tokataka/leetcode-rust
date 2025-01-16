use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

use clap::{Parser, Subcommand};
use leetcode::api::{self, get_problem, Problem, StatWithStatus};
use rand::prelude::*;
use regex::Regex;

const SOURCE_ROOT: &str = "./solutions/src/attempting";
const DEST_ROOT: &str = "./solutions/src/solved";

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand, Debug)]
enum CliCommands {
    /// Fetch problem via API
    Fetch { problem_id: u32 },
    /// Archive solution to solved
    Archive { problem_id: u32 },
    /// Fetch a random problem via API
    Random,
}

fn main() {
    let cli = Cli::parse();

    let problems = api::list_problems().unwrap().stat_status_pairs;

    let problem_stat = match cli.command {
        CliCommands::Fetch { problem_id } => problems
            .iter()
            .find(|x| x.stat.frontend_question_id == problem_id)
            .expect("Invalid Problem ID"),
        CliCommands::Random => {
            let mut rng = rand::thread_rng();
            problems.iter().choose(&mut rng).unwrap()
        }
        CliCommands::Archive { problem_id } => {
            let problem_stat = problems
                .iter()
                .find(|x| x.stat.frontend_question_id == problem_id)
                .expect("Invalid Problem ID");

            archive_problem(problem_stat);

            return;
        }
    };

    let problem = get_problem(problem_stat).unwrap();

    fetch_problem(&problem);
}

fn archive_problem(problem_stat: &StatWithStatus) {
    let id_title = format!(
        "{:04}_{}",
        problem_stat.stat.frontend_question_id,
        problem_stat
            .stat
            .question_title_slug
            .as_ref()
            .unwrap()
            .replace("-", "_")
    );

    let source_file = Path::new(SOURCE_ROOT).join(format!("p{id_title}.rs"));
    let dest_file = Path::new(DEST_ROOT).join(format!("s{id_title}.rs"));

    if !source_file.exists() {
        panic!("Problem not found");
    }

    if dest_file.exists() {
        panic!("Archived solution already exists");
    }

    let _ = fs::rename(source_file, dest_file);

    let source_mod_path = format!("{SOURCE_ROOT}/mod.rs");
    let source_mod_file = Path::new(&source_mod_path);

    let source_mod_modified = io::BufReader::new(fs::File::open(source_mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| *x != format!("pub mod p{id_title};"))
        .collect::<Vec<_>>();

    let _ = fs::write(source_mod_path, source_mod_modified.join("\n"));

    let dest_mod_path = format!("{DEST_ROOT}/mod.rs");
    let dest_mod_file = Path::new(&dest_mod_path);

    let mut dest_mod_modified = io::BufReader::new(fs::File::open(dest_mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    dest_mod_modified.push(format!("pub mod s{id_title};"));
    dest_mod_modified.sort_by_key(|x| x[5..9].parse::<u32>().unwrap());

    let _ = fs::write(dest_mod_path, dest_mod_modified.join("\n"));
}

fn fetch_problem(problem: &Problem) {
    let problem_content = html2md::parse_html(&problem.content).replace(" ", " ");

    let examples_re =
        Regex::new(r"```\s*Input:(.+?)Output:(.+?)\s*(?:Explanation.+?)?```").unwrap();
    let inputs_re = Regex::new(r"(\w+) = (?:(\[.+?\])|([^\[\]]+?))(?:,\s*|$)").unwrap();

    let problem_content_joined = problem_content
        .split("\n")
        .map(|x| x.trim())
        .collect::<Vec<_>>()
        .join("");

    let examples = examples_re
        .captures_iter(&problem_content_joined)
        .map(|x| {
            let extracted = x.extract::<2>().1;
            let (inputs, expected) = (extracted[0], extracted[1]);

            let inputs = inputs_re
                .captures_iter(inputs.trim())
                .map(|x| {
                    let extracted = x.extract::<2>().1;
                    (extracted[0], extracted[1])
                })
                .collect::<Vec<_>>();

            (inputs, expected.trim())
        })
        .collect::<Vec<_>>();

    let code = problem
        .code_definition
        .iter()
        .find(|x| x.value == "rust")
        .expect("Rust code submission is not allowed for this problem.");

    let default_code = code
        .default_code
        .split("\n")
        .filter(|x| !x.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    let id_title = format!(
        "{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );

    let template = fs::read_to_string("./leetcode/problem_template").unwrap();

    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace(
            "__PROBLEM_DESC__",
            &problem_content
                .split("\n")
                .map(|x| format!("/// {x}"))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .replace(
            "__PROBLEM_DEFAULT_CODE__",
            &insert_return_in_code(&problem.return_type, &default_code),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
        .replace("__PROBLEM_LINK__", &parse_problem_link(problem))
        .replace("__DISCUSS_LINK__", &parse_discuss_link(problem))
        .replace(
            "__PROBLEM_TEST_CODE__",
            &create_test_code(&examples, &default_code),
        );

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{SOURCE_ROOT}/p{id_title}.rs"))
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    let mod_path = format!("{SOURCE_ROOT}/mod.rs");
    let mod_file = Path::new(&mod_path);
    let mut mod_modified = io::BufReader::new(fs::File::open(mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    mod_modified.push(format!("pub mod p{id_title};"));
    mod_modified.sort_by_key(|x| x[5..9].parse::<u32>().unwrap());

    let _ = fs::write(mod_path, mod_modified.join("\n"));
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse crate::util::linked_list::ListNode;")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse crate::util::tree::TreeNode;")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse crate::util::point::Point;")
    }
    extra_use_line
}

fn parse_problem_link(problem: &Problem) -> String {
    format!("https://leetcode.com/problems/{}/", problem.title_slug)
}

fn parse_discuss_link(problem: &Problem) -> String {
    format!(
        "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
        problem.title_slug
    )
}

fn insert_return_in_code(return_type: &str, code: &str) -> String {
    let re = Regex::new(r"\{[ \n]+}").unwrap();
    match return_type {
        "ListNode" => re
            .replace(code, "{\n        Some(Box::new(ListNode::new(0)))\n    }")
            .to_string(),
        "ListNode[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "TreeNode" => re
            .replace(
                code,
                "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n    }",
            )
            .to_string(),
        "boolean" => re.replace(code, "{\n        false\n    }").to_string(),
        "character" => re.replace(code, "{\n        '0'\n    }").to_string(),
        "character[][]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "double" => re.replace(code, "{\n        0f64\n    }").to_string(),
        "double[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "int[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "integer" => re.replace(code, "{\n        0\n    }").to_string(),
        "integer[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "integer[][]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<String>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<TreeNode>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<boolean>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<double>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<integer>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<list<integer>>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<list<string>>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<string>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "null" => code.to_string(),
        "string" => re
            .replace(code, "{\n        String::new()\n    }")
            .to_string(),
        "string[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "void" => code.to_string(),
        "NestedInteger" => code.to_string(),
        "Node" => code.to_string(),
        _ => code.to_string(),
    }
}

fn create_test_code(examples: &Vec<(Vec<(&str, &str)>, &str)>, code: &str) -> String {
    let function_re = Regex::new(r"fn (.+)\((.+)\) -> (.+) \{").unwrap();
    let function_matched = function_re.captures(code).unwrap().extract::<3>().1;

    let (function_name, input_types, output_type) = (
        function_matched[0],
        function_matched[1]
            .split(", ")
            .map(|x| x.split_once(": ").unwrap())
            .collect::<Vec<_>>(),
        function_matched[2],
    );

    let mut test_code = vec![];

    for (inputs, expected) in examples {
        let mut function_inputs = vec![];

        for (input, input_type) in inputs.iter().zip(&input_types) {
            let lvalue = input_type.0;
            let rvalue = format_value_type(input.1, input_type.1);
            test_code.push(format!("        let {lvalue} = {rvalue};"));
            function_inputs.push(lvalue.to_owned());
        }

        let rvalue = format_value_type(expected, output_type);
        test_code.push(format!("        let expected = {rvalue};"));
        test_code.push(format!(
            "        assert_eq!(Solution::{function_name}({}), expected);",
            function_inputs.join(", ")
        ));
    }

    test_code.join("\n")
}

fn format_value_type(value: &str, value_type: &str) -> String {
    match value_type {
        "Option<Box<ListNode>>" => format!("linked!{value}"),
        "Option<Rc<RefCell<TreeNode>>>" => format!("tree!{value}"),
        "String" => format!("{value}.to_string()"),
        x if x.starts_with("Vec<Vec<") => format!("nd_vec!{value}"),
        x if x.starts_with("Vec<") => {
            if x.contains("String") {
                format!("vec_string!{value}")
            } else if value.contains("Option") {
                format!("option_vec!{value}")
            } else {
                format!("vec!{value}")
            }
        }
        _ => value.to_owned(),
    }
}
