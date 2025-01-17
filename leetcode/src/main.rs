use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
    vec,
};

use clap::{Parser, Subcommand};
use leetcode::api::{self, MetaData, Problem, StatWithStatus};
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
    /// Fetch problems via API (space-separated)
    Fetch { problem_ids: Vec<u32> },
    /// Archive solutions to solved (space-separated)
    Archive { problem_ids: Vec<u32> },
    /// Fetch daily question
    Daily,
    /// Fetch a random problem via API
    Random,
}

fn main() {
    let cli = Cli::parse();

    let problems = api::list_problems().unwrap().stat_status_pairs;

    let problem_ids = match &cli.command {
        CliCommands::Fetch { problem_ids } | CliCommands::Archive { problem_ids } => {
            problem_ids.clone()
        }
        CliCommands::Random => {
            let mut rng = rand::thread_rng();
            vec![
                problems
                    .iter()
                    .choose(&mut rng)
                    .unwrap()
                    .stat
                    .frontend_question_id,
            ]
        }
        CliCommands::Daily => {
            let problem_id = api::get_daily_problem_id().expect("Daily question unavailable");
            vec![problem_id]
        }
    };

    problems
        .iter()
        .filter(|x| problem_ids.contains(&x.stat.frontend_question_id))
        .for_each(|problem_stat| match cli.command {
            CliCommands::Archive { problem_ids: _ } => archive_problem(problem_stat),
            _ => fetch_problem(problem_stat),
        });
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

    let mut source_mod_modified = io::BufReader::new(fs::File::open(source_mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| *x != format!("pub mod p{id_title};"))
        .collect::<Vec<_>>();

    source_mod_modified.sort_by_key(|x| x[9..13].parse::<u32>().unwrap());
    source_mod_modified.dedup();

    let _ = fs::write(source_mod_path, source_mod_modified.join("\n"));

    let dest_mod_path = format!("{DEST_ROOT}/mod.rs");
    let dest_mod_file = Path::new(&dest_mod_path);

    let mut dest_mod_modified = io::BufReader::new(fs::File::open(dest_mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    dest_mod_modified.push(format!("pub mod s{id_title};"));
    dest_mod_modified.sort_by_key(|x| x[9..13].parse::<u32>().unwrap());
    dest_mod_modified.dedup();

    let _ = fs::write(dest_mod_path, dest_mod_modified.join("\n"));
}

fn fetch_problem(problem_stat: &StatWithStatus) {
    let problem = api::get_problem(problem_stat).unwrap();

    let id_title = format!(
        "{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );

    let problem_path = format!("{SOURCE_ROOT}/p{id_title}.rs");
    let problem_file = Path::new(&problem_path);

    if problem_file.exists() {
        panic!("Problem \"{}\" already exists", problem.title);
    }

    let problem_content = html2md::parse_html(&problem.content).replace(" ", " ");

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

    let template = fs::read_to_string("./leetcode/problem_template").unwrap();

    let mut source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace(
            "__PROBLEM_DESC__",
            &problem_content
                .split("\n")
                .map(|x| format!("/// {x}"))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
        .replace("__PROBLEM_LINK__", &parse_problem_link(&problem))
        .replace("__DISCUSS_LINK__", &parse_discuss_link(&problem));

    source = if let Some(true) = problem.meta_data.systemdesign {
        source
            .replace(
                "__PROBLEM_DEFAULT_CODE__",
                &insert_return_in_code_systemdesign(&problem.meta_data, &default_code),
            )
            .replace(
                "__PROBLEM_TEST_CODE__",
                &create_test_code_systemdesign(&problem_content, &problem.meta_data),
            )
    } else {
        source
            .replace(
                "__PROBLEM_DEFAULT_CODE__",
                &insert_return_in_code(
                    &problem.meta_data.return_.as_ref().unwrap().type_,
                    &default_code,
                ),
            )
            .replace(
                "__PROBLEM_TEST_CODE__",
                &create_test_code(&problem_content, &problem.meta_data),
            )
    };

    let mut fp = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(problem_file)
        .unwrap();

    fp.write_all(source.as_bytes()).unwrap();
    drop(fp);

    let mod_path = format!("{SOURCE_ROOT}/mod.rs");
    let mod_file = Path::new(&mod_path);
    let mut mod_modified = io::BufReader::new(fs::File::open(mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    mod_modified.push(format!("pub mod p{id_title};"));
    mod_modified.sort_by_key(|x| x[9..13].parse::<u32>().unwrap());
    mod_modified.dedup();

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

fn default_return_value(return_type: &str) -> &str {
    match return_type {
        x if x.ends_with("[]") || x.starts_with("list<") => "vec![]",
        "ListNode" => "Some(Box::new(ListNode::new(0)))",
        "TreeNode" => "Some(Rc::new(RefCell::new(TreeNode::new(0)))",
        "boolean" => "false",
        "character" => "'0'",
        "double" => "0.",
        "integer" | "long" => "0",
        "string" => "String::new()",
        _ => "",
    }
}

fn insert_return_in_code(return_type: &str, code: &str) -> String {
    let re = Regex::new(r"\{[ \n]+}").unwrap();

    let return_value = default_return_value(return_type);

    re.replace(code, format!("{{\n        {return_value}\n    }}"))
        .to_string()
}

fn insert_return_in_code_systemdesign(meta_data: &MetaData, code: &str) -> String {
    let mut result = code.to_string();

    let mut replacements = vec![("new".to_string(), "Self {}")];
    for method in meta_data.methods.as_ref().unwrap() {
        replacements.push((
            to_snake_case(&method.name),
            default_return_value(&method.return_.type_),
        ));
    }

    for (method_name, replacement) in &replacements {
        let re = Regex::new(&format!("(fn {method_name}(?:.+))\\{{[ \n]+?}}")).unwrap();

        result = re
            .replace(&result, format!("$1{{\n        {replacement}\n    }}"))
            .to_string();
    }

    result
}

fn to_snake_case(s: &str) -> String {
    s.chars()
        .map(|ch| {
            if ch.is_uppercase() {
                format!("_{}", ch.to_lowercase())
            } else {
                ch.to_string()
            }
        })
        .collect::<String>()
}

fn create_test_code(problem_content: &str, meta_data: &MetaData) -> String {
    let problem_content = problem_content
        .split("\n")
        .map(|x| x.trim())
        .collect::<Vec<_>>()
        .join("");

    let examples_re =
        Regex::new(r"```\s*Input:(.+?)Output:(.+?)\s*(?:Explanation.+?)?```").unwrap();

    let examples = examples_re
        .captures_iter(&problem_content)
        .map(|x| {
            let extracted = x.extract::<2>().1;
            let (inputs_str, expected) = (extracted[0], extracted[1]);

            let inputs_str_len = inputs_str.chars().count();

            let mut inputs = vec![];
            let mut is_searching_equal = true;
            let mut right = inputs_str_len;
            let mut rvalue = "";

            for (i, ch) in inputs_str.chars().rev().enumerate() {
                let i = inputs_str_len - i - 1;

                if is_searching_equal && ch == '=' {
                    rvalue = inputs_str[(i + 1)..right].trim();
                    right = i;

                    is_searching_equal = !is_searching_equal;
                } else if !is_searching_equal && ch == ',' {
                    let lvalue = inputs_str[(i + 1)..right].trim();
                    right = i;

                    inputs.push((lvalue, rvalue));

                    is_searching_equal = !is_searching_equal;
                }
            }

            inputs.push((inputs_str[..right].trim(), rvalue));
            inputs.reverse();

            (inputs, expected.trim())
        })
        .collect::<Vec<_>>();
    let mut test_code = vec![];

    for (inputs, expected) in examples {
        let mut function_inputs = vec![];

        for (input, param) in inputs.iter().zip(meta_data.params.as_ref().unwrap()) {
            let lvalue = to_snake_case(&param.name);
            let rvalue = format_value_type(input.1, &param.type_);
            test_code.push(format!("        let {lvalue} = {rvalue};"));
            function_inputs.push(lvalue);
        }

        let rvalue = format_value_type(expected, &meta_data.return_.as_ref().unwrap().type_);
        test_code.push(format!("        let expected = {rvalue};"));

        let function_name = to_snake_case(meta_data.name.as_ref().unwrap());
        test_code.push(format!(
            "        assert_eq!(Solution::{function_name}({}), expected);",
            function_inputs.join(", ")
        ));
    }

    test_code.join("\n")
}

fn create_test_code_systemdesign(problem_content: &str, meta_data: &MetaData) -> String {
    let problem_content = problem_content
        .split("\n")
        .map(|x| x.trim())
        .collect::<Vec<_>>()
        .join("");

    let examples_re = Regex::new(
        r"```\s*Input(\[[^\n]+\])\s*(\[[^\n]+\])\s*Output\s*(\[[^\n]+\])\s*(?:Explanation.+?)?```",
    )
    .unwrap();

    let classname = meta_data.classname.clone().unwrap();
    let constructor_param_types = &meta_data.constructor.as_ref().unwrap().params;

    examples_re
        .captures_iter(&problem_content)
        .flat_map(|x| {
            let extracted = x.extract::<3>().1;
            let (method_names, method_params, expecteds) =
                (extracted[0], extracted[1], extracted[2]);

            let method_names = method_names[1..method_names.len() - 1]
                .split(",")
                .map(|x| x.trim().trim_matches('"'));

            let method_params_re = Regex::new(r"\[([^\[\]]*)\]").unwrap();

            let method_params = method_params_re
                .captures_iter(method_params)
                .map(|x| x.extract::<1>().1[0]);

            let expecteds = expecteds[1..expecteds.len() - 1]
                .split(",")
                .map(|x| x.trim());

            method_names
                .zip(method_params)
                .zip(expecteds)
                .map(|((method_name, method_param), expected)| {
                    let params = method_param.split(",").map(|x| x.trim());

                    if method_name == classname {
                        format!(
                            "        let obj = {}::new({});",
                            meta_data.classname.clone().unwrap(),
                            params
                                .zip(constructor_param_types)
                                .map(|(param, param_def)| {
                                    format_value_type(param, &param_def.type_)
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    } else {
                        let method = meta_data
                            .methods
                            .as_ref()
                            .unwrap()
                            .iter()
                            .find(|x| x.name == method_name)
                            .unwrap();

                        if method.return_.type_ == "void" {
                            format!(
                                "        obj.{}({});",
                                to_snake_case(method_name),
                                params
                                    .zip(&method.params)
                                    .map(|(param, param_def)| {
                                        format_value_type(param, &param_def.type_)
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", "),
                            )
                        } else if method.return_.type_ == "boolean" {
                            format!(
                                "        assert!({}obj.{}({}));",
                                if expected == "false" { "!" } else { "" },
                                to_snake_case(method_name),
                                params
                                    .zip(&method.params)
                                    .map(|(param, param_def)| {
                                        format_value_type(param, &param_def.type_)
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", "),
                            )
                        } else {
                            format!(
                                "        assert_eq!(obj.{}({}), {});",
                                to_snake_case(method_name),
                                params
                                    .zip(&method.params)
                                    .map(|(param, param_def)| {
                                        format_value_type(param, &param_def.type_)
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", "),
                                expected
                            )
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_value_type(value: &str, value_type: &str) -> String {
    if !value_type.ends_with("[]") && !value_type.starts_with("list<") {
        return match value_type {
            "ListNode" => format!("linked!{value}"),
            "TreeNode" => format!("tree!{value}"),
            "string" => format!("{value}.to_owned()"),
            _ => value.to_owned(),
        };
    }

    let mut vec_count = 0;
    let mut value_type = value_type;

    while value_type.ends_with("[]") {
        vec_count += 1;
        value_type = &value_type[..(value_type.len() - 2)];
    }

    while value_type.starts_with("list<") {
        vec_count += 1;
        value_type = &value_type[5..(value_type.len() - 1)];
    }

    let nd = if vec_count > 1 { "nd_" } else { "" };

    match value_type {
        "string" => format!("{nd}vec_string!{value}"),
        _ => format!("{nd}vec!{value}"),
    }
}
