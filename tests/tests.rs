use std::process::Command;
use std::process::Output;

#[cfg(test)]
mod tests {
    use super::*;

    const SNIPPET_PATH: &str = "tests/snippets/";

    // runs an instance of rustic with the given file name and constprop flag.
    // returns the output of the process.
    fn run_program(file_name: &str, constprop: bool) -> Output {
        let mut args = vec![format!("{}{}", SNIPPET_PATH, file_name)];

        if constprop {
            args.push("--constprop".to_string());
        }

        Command::new(env!("CARGO_BIN_EXE_rustic"))
            .args(&args) // Pass the arguments to the program
            .output()
            .expect("Failed to execute process")
    }

    // Check that a rustic execution runs successfully and that the output contains the expected strings.
    fn runs_ok(file_name: &str, constprop: bool, expected: &[&str]) {
        let output = run_program(file_name, constprop);
        assert!(output.status.success(), "Failed: {:?}", output);
        for line in expected {
            assert!(String::from_utf8(output.stdout.clone()).unwrap().contains(line));
        }
    }

    // Check that a rustic execution fails and that the output contains the expected strings.
    fn runs_err(file_name: &str, constprop: bool, expected: &[&str]) {
        let output = run_program(file_name, constprop);
        assert!(!output.status.success(), "Failed: {:?}", output);
        for line in expected {
            assert!(String::from_utf8(output.stderr.clone()).unwrap().contains(line));
        }
    }

    #[test]
    fn test_all_grammar_feats() {
        runs_ok("parser/all_grammar_feats.rs", false, &vec![]);
    }

    #[test]
   fn test_binops() {
        runs_ok("parser/binops.rs", false, &vec![
            "let a = 1u8 + 1u8;",
            "let b = 1u8 - 1u8;",
            "let c = 1u8 * 1u8;",
            "let d = 1u8 / 1u8;"
        ]);
    }

    #[test]
    fn test_binops_constprop() {
        runs_ok("parser/binops.rs", true, &vec![
            "let a = 2u8;",
            "let b = 0u8;",
            "let c = 1u8;",
            "let d = 1u8;"
        ]);
    }

    #[test]
    fn test_comments() {
        runs_ok("parser/comments.rs", false, &vec![]);
    }

    #[test]
    fn test_inputs() {
        runs_ok("parser/inputs.rs", false, &vec![]);
    }

    #[test]
    fn test_paren() {
        runs_ok("parser/paren.rs", false, &vec![]);
    }

    #[test]
    fn single_def() {
        runs_ok("parser/single_def.rs", false, &vec![]);
    }

    #[test]
    fn test_redecl() {
        runs_err("errors/redecl.rs", false, &vec![
            "Error: Redefinition of variable 'a'."
        ]);
    }

    #[test]
    fn test_input_redecl() {
        runs_err("errors/input_redecl.rs", false, &vec![
            "Error: Redefinition of input variable 'a'."
        ]);
    }

    #[test]
    fn test_input_redecl_2() {
        runs_err("errors/input_redecl_2.rs", false, &vec![
            "Error: Redefinition of variable 'a'."
        ]);
    }

    #[test]
    fn test_out_of_order() {
        runs_err("errors/out_of_order.rs", false, &vec![
            "Error: Use of undefined variable 'a'."
        ]);
    }

    #[test]
    fn test_undefined() {
        runs_err("errors/undefined.rs", false, &vec![
            "Error: Use of undefined variable 'b'."
        ]);
    }

    #[test]
    fn test_constprop_unfoldable() {
        runs_ok("constprop/unfoldable.rs", true, &vec![
            "let c = a + 2u8;",
            "let d = b * 3u8;"
        ]);
    }

    #[test]
    fn test_constprop_nested_2() {
        runs_ok("constprop/nested_2.rs", true, &vec![
            "let result = ((((i) + 3u8)) * ((j) + 2u8)) - 8u8;"
        ]);
    }

    #[test]
    fn test_constprop_nested_3() {
        runs_ok("constprop/nested_3.rs", true, &vec![
            "let x = 1u8;",
            "let y = 29u8;"
        ]);
    }

    #[test]
    fn test_constprop_mixed() {
        runs_ok("constprop/mixed.rs", true, &vec![
            "let m = ((x) + 5u8) - 2u8;",
            "let n = 3u8 + x"
        ]);
    }

    #[test]
    fn test_constprop_paren() {
        runs_ok("constprop/paren.rs", true, &vec![
            "let m = 21u8;",
            "let n = 2u8;"
        ]);
    }

    #[test]
    fn test_constprop_overflow() {
        runs_err("constprop/overflow.rs", true, &vec![
            "Error: Constant evaluation resulted in value greater than 255: 250 + 10"
        ]);
    }

    #[test]
    fn test_constprop_overflow_2() {
        runs_err("constprop/overflow_2.rs", true, &vec![
            "Error: Constant evaluation resulted in value greater than 255: 40 * 7"
        ]);
    }

    #[test]
    fn test_constprop_complex_2() {
        runs_ok("constprop/complex_2.rs", true, &vec![
            "let x = 2u8 + a;",
            "let y = 12u8 + b;",
            "let z = (((x) * y)) + 3u8;"
        ]);
    }

    #[test]
    fn test_constprop_div_non_exact() {
        runs_err("constprop/div_non_exact.rs", true, &vec![
            "Error: Constant evaluation resulted in non-integer division: 10 / 3"
        ]);
    }

    #[test]
    fn test_constprop_div0() {
        runs_err("constprop/div0.rs", true, &vec![
            "Error: Constant evaluation resulted in division by zero: 10 / 0"
        ]);
    }

    #[test]
    fn test_constprop_complex() {
        runs_ok("constprop/complex.rs", true, &vec![
            "let p = 14u8;",
            "let q = 3u8;"
        ]);
    }

    #[test]
    fn test_constprop_mixed_2() {
        runs_ok("constprop/mixed_2.rs", true, &vec![
            "let a = 7u8;",
            "let b = 7u8 + x;",
            "let c = y * 2u8;",
            "let d = c - 1u8;"
        ]);
    }

    #[test]
    fn test_constprop_nested() {
        runs_ok("constprop/nested.rs", true, &vec![
            "let x = 7u8;",
            "let y = 20u8;"
        ]);
    }

    #[test]
    fn test_constprop_unfoldable_2() {
        runs_ok("constprop/unfoldable_2.rs", true, &vec![
            "let x = (a + 1u8) * 2u8;",
            "let y = (b - 1u8) / 2u8;",
            "let z = (a + 2u8) * (b - 1u8) + 3u8;"
        ]);
    }

    #[test]
    fn test_constprop_underflow() {
        runs_err("constprop/underflow.rs", true, &vec![
            "Error: Constant evaluation resulted in negative value: 5 - 10"
        ]);
    }

    #[test]
    fn test_constprop_idents() {
        runs_ok("constprop/idents.rs", true, &vec![
            "let a = 10u8;",
            "let b = 5u8;",
            "let c = 15u8;",
            "let d = 10u8;"
        ]);
    }
}