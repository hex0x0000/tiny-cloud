// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

use std::f64::consts::PI;

use crate::*;

#[test]
fn arg_name_equality() {
    let flag = ArgName::both('o', "out");
    let flag1 = ArgName::short('o');
    let flag2 = ArgName::long("out");
    assert_eq!(flag, flag);
    assert_eq!(flag1, flag1);
    assert_eq!(flag2, flag2);
    assert_eq!(flag, flag1);
    assert_eq!(flag, flag2);
    assert_ne!(flag1, flag2);
}

#[test]
fn test_arg_macro() {
    assert_eq!(arg! { -'h', --help }, ArgName::both('h', "help"));
    assert_eq!(arg! { -'h', --long-help }, ArgName::long("long-help"));
    assert_eq!(
        arg! { -'h', --long-long-long-help },
        ArgName::long("long-long-long-help")
    );
    assert_eq!(arg!(-'h'), ArgName::short('h'));
    assert_eq!(arg!(--help), ArgName::long("help"));
    assert_eq!(arg! { --long-help }, ArgName::long("long-help"));
    assert_eq!(
        arg! { --long-long-long-help },
        ArgName::long("long-long-long-help")
    );
}

#[test]
fn test_value_macro() {
    assert_eq!(value!(), ArgValue::Flag);
    assert_eq!(value!(string), ArgValue::String(None));
    assert_eq!(value!(num), ArgValue::Num(None));
    assert_eq!(value!(float), ArgValue::Float(None));
    assert_eq!(value!(path), ArgValue::Path(None));
    assert_eq!(
        value!(string, "a b c"),
        ArgValue::String(Some("a b c".into()))
    );
    assert_eq!(value!(num, 2), ArgValue::Num(Some(2)));
    assert_eq!(value!(float, 2.3), ArgValue::Float(Some(2.3)));
    assert_eq!(value!(path, "/path"), ArgValue::Path(Some("/path".into())));
}

#[test]
#[should_panic]
fn test_command_fail() {
    Command::create("test", "A really cool failing test")
        .arg(arg!(-'V'), value!(), "Program's version")
        .arg(arg!(--path), value!(path), "Insert an example path")
        .arg(arg!(-'h', --help), value!(), "Show this help")
        .arg(arg!(--help), value!(), "Oopsie");
}

fn test_command() -> Command {
    Command::create("test", "A really cool test")
        .arg(arg!(-'V'), value!(), "Program's version")
        .arg(arg!(--path), value!(path, "/default/path"), "Insert a path")
        .arg(arg!(--num), value!(num, 3), "Insert a number")
        .arg(arg!(--float), value!(float), "Insert a float")
        .arg(arg!(--idk), value!(string), "Just insert something")
        .arg(
            arg!(--idk2),
            value!(string, "default value"),
            "Just insert something again",
        )
        .arg(arg!(-'h', --help), value!(), "Show this help")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .license(env!("CARGO_PKG_LICENSE"))
}

#[test]
#[should_panic]
fn test_subcmd_fail() {
    Command::create("testception", "A really failed test inception")
        .arg(arg!(--idk), value!(string), "Just insert something")
        .subcommand(test_command())
        .subcommand(test_command())
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .license(env!("CARGO_PKG_LICENSE"));
}

fn mkargs(vec: &[&str]) -> Vec<String> {
    vec.iter().map(|&s| s.into()).collect()
}

#[test]
fn test_subcmd() {
    let input = mkargs(&["test-program", "test", "--idk2", "a b c"]);
    let parsed = Command::create("testception", "A really good test inception")
        .arg(
            arg!(--idk2),
            value!(string, "a b c d"),
            "Just insert something",
        )
        .subcommand(test_command().color(false))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .license(env!("CARGO_PKG_LICENSE"))
        .parse_from(input)
        .unwrap();
    println!("{}", parsed.help);
    assert_eq!(
        parsed.args.get(ArgName::long("idk2")).string().unwrap(),
        "a b c"
    );
    assert_eq!(
        parsed.args.get(arg!(--idk2)).description,
        "Just insert something again"
    );
    assert!(parsed.help.starts_with("testception test"));
}

#[test]
fn test_inputs() {
    let input = mkargs(&[
        "test-program",
        "--num",
        "6",
        "--float",
        &PI.to_string(),
        "-V",
        "--path",
        "/some/path",
        "-h",
        "-h",
        "--idk",
        "hiii hello",
    ]);
    let parsed = test_command().parse_from(input).unwrap();
    println!("{}", parsed.help);
    assert_eq!(parsed.args.get(arg!(--num)).num().unwrap(), 6);
    assert_eq!(parsed.args.get(arg!(--float)).float().unwrap(), PI);
    assert_eq!(
        parsed.args.get(arg!(--path)).path().unwrap().clone(),
        PathBuf::from("/some/path")
    );
    assert_eq!(parsed.args.get(arg!(--idk)).string().unwrap(), "hiii hello");
    assert_eq!(parsed.args.count(arg!(-'V')), 1);
    assert_eq!(parsed.args.count(arg!(-'h')), 2);
    assert!(parsed.args.try_get(arg!(-'a')).is_none());
}

#[test]
fn test_multiple_flags() {
    let input = mkargs(&[
        "test-program",
        "-h",
        "-h",
        "-h",
        "--path",
        "/path/a",
        "--path",
        "/path/b",
    ]);
    let parsed = test_command().parse_from(input).unwrap();
    assert_eq!(parsed.args.count(arg!(-'h')), 3);
    assert_eq!(parsed.args.count(arg!(--path)), 2);
    assert_eq!(
        parsed.args.get(arg!(--path)).path().unwrap().clone(),
        PathBuf::from("/path/b")
    );
}

#[test]
fn test_single_arg1() {
    let input = mkargs(&["test-program", "--help"]);
    let parsed = test_command().parse_from(input).unwrap();
    assert_eq!(parsed.args.count(arg!(--help)), 1);
    assert_eq!(parsed.args.count(arg!(-'h')), 1);
}

#[test]
fn test_single_arg2() {
    let input = mkargs(&["test-program", "--float", &PI.to_string()]);
    let parsed = test_command().parse_from(input).unwrap();
    assert_eq!(parsed.args.get(arg!(--float)).float().unwrap(), PI);
}

#[test]
#[should_panic]
fn test_bad_input1() {
    let input = mkargs(&["test-program", "--num", "6", "6"]);
    test_command().parse_from(input).unwrap();
}

#[test]
#[should_panic]
fn test_bad_input2() {
    let input = mkargs(&["test-program", "-num", "6"]);
    test_command().parse_from(input).unwrap();
}

#[test]
#[should_panic]
fn test_bad_input3() {
    let input = mkargs(&["test-program", "num", "6"]);
    test_command().parse_from(input).unwrap();
}

#[test]
fn test_tabbing() {
    let cmd = Command::create("tabbing", "Tests tabbing")
        .arg(arg!(--aaaaaaa), value!(), "testestetstestestest")
        .arg(arg!(--aaaaaaaaaaa), value!(), "testestetstestestest")
        .arg(arg!(--aaaaaaaaaaaaaaa), value!(), "testestetstestestest")
        .arg(
            arg!(--aaaaaaaaaaaaaaaaaaa),
            value!(),
            "testestetstestestest",
        )
        .arg(
            arg!(--aaaaaaaaaaaaaaaaaaaaaaaa),
            value!(),
            "testestetstestestest",
        )
        .subcommand(Command::create("testsub", "testsub"))
        .subcommand(Command::create("testsubanan", "testsub"))
        .subcommand(Command::create("testsubaaaaaaaa", "testsub"))
        .subcommand(Command::create("testsubaaaaaaaaaaaaa", "testsub"))
        .parse_from(vec!["test".into()])
        .unwrap();
    println!("{}", cmd.help);
}
