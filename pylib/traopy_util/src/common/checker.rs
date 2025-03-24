use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.common")]
pub fn normal_judge_checker(expected: String, actual: String) -> bool {
    let expected = parse_whitespace_and_newline(&expected);
    let actual = parse_whitespace_and_newline(&actual);
    expected == actual
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.common")]
pub fn parse_whitespace_and_newline(s: &str) -> String {
    s.replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
