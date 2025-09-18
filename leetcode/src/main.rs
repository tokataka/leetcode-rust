use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead, Write},
    path::Path,
    vec,
};

use clap::{Parser, Subcommand, ValueEnum};
use leetcode::api::{self, MetaData, Problem, StatWithStatus};
use regex::Regex;
use scraper::{Html, Selector};

const ATTEMPTING_ROOT: &str = "./solutions/src/attempting";
const ARCHIVED_ROOT: &str = "./solutions/src/archived";

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand, Debug)]
enum CliCommands {
    /// Fetch problems with problem id
    Fetch {
        #[clap(required = true)]
        problem_ids: Vec<u32>,
    },
    /// Archive solutions to archived (empty to archive all)
    Archive {
        problem_ids: Vec<u32>,
        #[arg(long)]
        exclude: Vec<u32>,
    },
    /// Fetch daily question
    Daily,
    /// Fetch a random problem via API
    Random {
        #[arg(short, long)]
        difficulty: Vec<ProblemDifficulty>,
    },
}

#[derive(ValueEnum, Debug, Clone)]
enum ProblemDifficulty {
    Easy,
    Medium,
    Hard,
}

impl ProblemDifficulty {
    fn to_str(&self) -> &str {
        match &self {
            ProblemDifficulty::Easy => "EASY",
            ProblemDifficulty::Medium => "MEDIUM",
            ProblemDifficulty::Hard => "HARD",
        }
    }
}

fn main() {
    let mut cli = Cli::parse();

    let problems = api::list_problems().unwrap().stat_status_pairs;

    let existing_attempting_problems =
        io::BufReader::new(fs::File::open(format!("{ATTEMPTING_ROOT}/mod.rs")).unwrap())
            .lines()
            .filter_map(|x| x.unwrap()[9..13].parse::<u32>().ok())
            .filter(|&x| x > 0)
            .collect::<HashSet<_>>();

    let existing_archived_problems =
        io::BufReader::new(fs::File::open(format!("{ARCHIVED_ROOT}/mod.rs")).unwrap())
            .lines()
            .filter_map(|x| x.unwrap()[9..13].parse::<u32>().ok())
            .collect::<HashSet<_>>();

    let problem_ids = match &mut cli.command {
        CliCommands::Fetch { problem_ids } => std::mem::take(problem_ids),
        CliCommands::Archive {
            problem_ids,
            exclude,
        } => if problem_ids.is_empty() {
            existing_attempting_problems.iter().cloned().collect()
        } else {
            std::mem::take(problem_ids)
        }
        .into_iter()
        .filter(|x| !exclude.contains(x))
        .collect(),
        CliCommands::Random { difficulty } => {
            let difficulties = difficulty.iter().map(|d| d.to_str()).collect::<Vec<_>>();

            loop {
                let problem_id = api::get_random_problem_title_slug(&difficulties)
                    .expect("Random question failed");

                if !existing_attempting_problems.contains(&problem_id)
                    && !existing_archived_problems.contains(&problem_id)
                {
                    break vec![problem_id];
                }
            }
        }
        CliCommands::Daily => {
            let problem_id = api::get_daily_problem_id().expect("Daily question unavailable");
            vec![problem_id]
        }
    };

    problems
        .iter()
        .filter(|x| problem_ids.contains(&x.stat.frontend_question_id))
        .for_each(|problem_stat| {
            let problem_id = problem_stat.stat.frontend_question_id;
            let problem_title = problem_stat.stat.question_title.clone().unwrap();

            let problem_string = format!("Problem #{}: {}", problem_id, &problem_title);

            match &cli.command {
                CliCommands::Archive {
                    problem_ids: _,
                    exclude: _,
                } => {
                    // if problem_ids.is_empty() {
                    //     problem_ids.extend(&existing_attempting_problems);
                    // }

                    if !existing_attempting_problems.contains(&problem_id) {
                        println!("{} not found", &problem_string);
                        return;
                    }

                    if existing_archived_problems.contains(&problem_id) {
                        println!("{} already archived", &problem_string);
                        return;
                    }

                    archive_problem(problem_stat);

                    println!("Archived {}", &problem_string);
                }
                _ => {
                    if existing_attempting_problems.contains(&problem_id)
                        || existing_archived_problems.contains(&problem_id)
                    {
                        println!("{} already exists", &problem_string);
                        return;
                    }

                    fetch_problem(problem_stat);

                    println!("Fetched {}", &problem_string,)
                }
            };
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
            .replace('-', "_")
    );

    let source_file = Path::new(ATTEMPTING_ROOT).join(format!("p{id_title}.rs"));
    let dest_file = Path::new(ARCHIVED_ROOT).join(format!("p{id_title}.rs"));

    let _ = fs::rename(source_file, dest_file);

    let source_mod_path = format!("{ATTEMPTING_ROOT}/mod.rs");
    let source_mod_file = Path::new(&source_mod_path);

    let mut source_mod_modified = io::BufReader::new(fs::File::open(source_mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| *x != format!("pub mod p{id_title};"))
        .collect::<Vec<_>>();

    source_mod_modified.sort_by_key(|x| x[9..13].parse::<u32>().unwrap());
    source_mod_modified.dedup();

    let _ = fs::write(source_mod_path, source_mod_modified.join("\n"));

    let dest_mod_path = format!("{ARCHIVED_ROOT}/mod.rs");
    let dest_mod_file = Path::new(&dest_mod_path);

    let mut dest_mod_modified = io::BufReader::new(fs::File::open(dest_mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    dest_mod_modified.push(format!("pub mod p{id_title};"));
    dest_mod_modified.sort_by_key(|x| x[9..13].parse::<u32>().unwrap());
    dest_mod_modified.dedup();

    let _ = fs::write(dest_mod_path, dest_mod_modified.join("\n"));
}

fn fetch_problem(problem_stat: &StatWithStatus) {
    let problem = api::get_problem(problem_stat).unwrap();

    let id_title = format!(
        "{:04}_{}",
        problem.question_id,
        problem.title_slug.replace('-', "_")
    );

    let problem_path = format!("{ATTEMPTING_ROOT}/p{id_title}.rs");
    let problem_file = Path::new(&problem_path);

    let problem_markdown = html2md::parse_html(&problem.content).replace(' ', " ");
    let problem_dom = Html::parse_fragment(&problem.content);

    let code = problem
        .code_definition
        .iter()
        .find(|x| x.value == "rust")
        .expect("Rust code submission is not allowed for this problem.");

    let default_code = code
        .default_code
        .split('\n')
        .filter(|x| !x.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    let template = fs::read_to_string("./leetcode/problem_template").unwrap();

    let mut source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace(
            "__PROBLEM_DESC__",
            &problem_markdown
                .split('\n')
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
                &create_test_code_systemdesign(&problem_dom, &problem.meta_data),
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
                &create_test_code(&problem_dom, &problem.meta_data),
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

    let mod_path = format!("{ATTEMPTING_ROOT}/mod.rs");
    let mod_file = Path::new(&mod_path);
    let mut mod_modified = io::BufReader::new(fs::File::open(mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    mod_modified.push(format!("pub mod p{id_title};"));
    mod_modified.sort_by_key(|x| x[9..13].parse::<u32>().unwrap());
    mod_modified.dedup();

    let _ = fs::write(mod_path, mod_modified.join("\n"));

    if std::env::var("TERM_PROGRAM").is_ok_and(|v| v == "vscode") {
        std::process::Command::new("code")
            .arg(&problem_path)
            .status()
            .expect("'code' is not found");
    }
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
        "TreeNode" => "Some(Rc::new(RefCell::new(TreeNode::new(0))))",
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
    let mut result = vec![];
    let mut in_comment = false;

    for line in code.split('\n') {
        if line.trim().starts_with("//") {
            continue;
        }

        if line.trim().starts_with("/**") {
            in_comment = true;
        }

        if !in_comment {
            result.push(line);
        }

        if line.trim().ends_with("*/") {
            in_comment = false;
        }
    }

    let mut result = result.join("\n");

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

fn create_test_code(problem_dom: &Html, meta_data: &MetaData) -> String {
    enum TextMode {
        Input,
        Expected,
        Explanation,
    }

    let mut examples = vec![];

    let selector_pre = Selector::parse("pre").unwrap();
    let selector_example_block = Selector::parse("div.example-block").unwrap();

    for selector in [selector_pre, selector_example_block] {
        for example in problem_dom.select(&selector) {
            let mut text_mode = TextMode::Input;
            let mut input_str = vec![];
            let mut expected = vec![];

            for text in example.text() {
                match text.trim() {
                    "" => (),
                    _x if _x.starts_with("Input") => text_mode = TextMode::Input,
                    _x if _x.starts_with("Output") => text_mode = TextMode::Expected,
                    _x if _x.starts_with("Explanation") => text_mode = TextMode::Explanation,
                    x => match text_mode {
                        TextMode::Input => input_str.push(x),
                        TextMode::Expected => expected.push(x),
                        TextMode::Explanation => break,
                    },
                }
            }

            examples.push((input_str.join(" "), expected.join(" ")));
        }
    }

    let examples = examples.into_iter().map(|(inputs_str, expected)| {
        let mut inputs = vec![];
        let inputs_str_len = inputs_str.chars().count();
        let mut is_searching_equal = true;
        let mut right = inputs_str_len;
        let mut rvalue = "".to_owned();

        for (i, ch) in inputs_str.chars().rev().enumerate() {
            let i = inputs_str_len - i - 1;

            if is_searching_equal && ch == '=' {
                inputs_str[(i + 1)..right].trim().clone_into(&mut rvalue);
                right = i;

                is_searching_equal = !is_searching_equal;
            } else if !is_searching_equal && ch == ',' {
                let lvalue = inputs_str[(i + 1)..right].trim().to_owned();
                right = i;

                inputs.push((lvalue, rvalue.clone()));
                rvalue.clear();

                is_searching_equal = !is_searching_equal;
            }
        }

        inputs.push((inputs_str[..right].trim().to_owned(), rvalue));
        inputs.reverse();

        (inputs, expected)
    });

    let mut test_code = vec![];

    for (inputs, expected) in examples {
        let mut function_inputs = vec![];

        for (input, param) in inputs.iter().zip(meta_data.params.as_ref().unwrap()) {
            let lvalue = to_snake_case(&param.name);
            let rvalue = format_value_type(&input.1, &param.type_);
            test_code.push(format!("        let {lvalue} = {rvalue};"));
            function_inputs.push(lvalue);
        }

        let rvalue = format_value_type(&expected, &meta_data.return_.as_ref().unwrap().type_);
        test_code.push(format!("        let expected = {rvalue};"));

        let function_name = to_snake_case(meta_data.name.as_ref().unwrap());
        test_code.push(format!(
            "        assert_eq!(Solution::{function_name}({}), expected);",
            function_inputs.join(", ")
        ));
    }

    test_code.join("\n")
}

fn create_test_code_systemdesign(problem_dom: &Html, meta_data: &MetaData) -> String {
    // let problem_content = problem_content
    //     .split('\n')
    //     .map(|x| x.trim())
    //     .collect::<Vec<_>>()
    //     .join("");

    // let examples_re = Regex::new(
    //     r"```\s*Input(\[[^\n]+\])\s*(\[[^\n]+\])\s*Output\s*(\[[^\n]+\])\s*(?:Explanation.+?)?```",
    // )
    // .unwrap();

    let classname = meta_data.classname.clone().unwrap();
    let constructor_param_types = &meta_data.constructor.as_ref().unwrap().params;

    enum TextMode {
        Input,
        Expected,
        Explanation,
    }

    let mut examples = vec![];

    let selector_pre = Selector::parse("pre").unwrap();
    let selector_example_block = Selector::parse("div.example-block").unwrap();

    for selector in [selector_pre, selector_example_block] {
        for example in problem_dom.select(&selector) {
            let mut text_mode = TextMode::Input;
            let mut input_str = vec![];
            let mut expected = vec![];

            for text in example.text() {
                match text.trim() {
                    "" => (),
                    _x if _x.starts_with("Input") => text_mode = TextMode::Input,
                    _x if _x.starts_with("Output") => text_mode = TextMode::Expected,
                    _x if _x.starts_with("Explanation") => text_mode = TextMode::Explanation,
                    x => match text_mode {
                        TextMode::Input => input_str.push(x),
                        TextMode::Expected => expected.push(x),
                        TextMode::Explanation => break,
                    },
                }
            }

            examples.push((input_str.join("\n"), expected.join(" ")));
        }
    }

    examples
        .into_iter()
        .flat_map(|(input_str, expecteds)| {
            let (method_names, method_params) = input_str.split_once('\n').unwrap();

            let method_names = split_list_str(method_names);
            let method_names = method_names.iter().map(|x| x.trim_matches('"'));

            let method_params = split_list_str(method_params);

            let expecteds = split_list_str(&expecteds);

            method_names
                .zip(method_params)
                .zip(expecteds)
                .map(|((method_name, method_param), expected)| {
                    let params = split_list_str(method_param);

                    if method_name == classname {
                        format!(
                            "        let mut obj = {}::new({});",
                            meta_data.classname.clone().unwrap(),
                            params
                                .into_iter()
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
                                    .into_iter()
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
                                    .into_iter()
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
                                    .into_iter()
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

fn split_list_str(s: &str) -> Vec<&str> {
    let s = &s[1..s.len() - 1];

    let mut start_idx = 0;
    let mut brace_count = 0;

    let mut result = vec![];

    for cur_idx in 0..s.len() {
        match &s[cur_idx..cur_idx + 1] {
            "[" => brace_count += 1,
            "]" => brace_count -= 1,
            "," => {
                if brace_count == 0 {
                    result.push(s[start_idx..cur_idx].trim());
                    start_idx = cur_idx + 1;
                }
            }
            _ => (),
        }
    }

    result.push(s[start_idx..].trim());

    result
}

fn format_value_type(value: &str, value_type: &str) -> String {
    if !value_type.ends_with("[]") && !value_type.starts_with("list<") {
        return match value_type {
            "ListNode" => format!("linked!{value}"),
            "TreeNode" => format!("tree!{value}"),
            "string" => format!("{value}.to_owned()"),
            "character" => value.replace("\"", "'"),
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
