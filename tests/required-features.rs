extern crate cargotest;
extern crate hamcrest;

use cargotest::is_nightly;
use cargotest::install::{cargo_home, has_installed_exe};
use cargotest::support::{project, execs};
use hamcrest::{assert_that, existing_file, not};

#[test]
fn build_bin_default_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a"]
            a = []

            [[bin]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("build"),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());

    assert_that(p.cargo_process("build").arg("--no-default-features"),
                execs().with_status(0));
    assert_that(&p.bin("foo"), not(existing_file()));

    assert_that(p.cargo_process("build").arg("--bin=foo"),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());

    assert_that(p.cargo_process("build").arg("--bin=foo").arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
error: target `foo` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
"));
    assert_that(&p.bin("foo"), not(existing_file()));
}

#[test]
fn build_bin_arg_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            a = []

            [[bin]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("build").arg("--features").arg("a"),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
}

#[test]
fn build_bin_multiple_required_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a", "b"]
            a = []
            b = ["a"]
            c = []

            [[bin]]
            name = "foo_1"
            path = "src/foo_1.rs"
            required-features = ["b", "c"]

            [[bin]]
            name = "foo_2"
            path = "src/foo_2.rs"
            required-features = ["a"]
        "#)
        .file("src/foo_1.rs", "fn main() {}")
        .file("src/foo_2.rs", "fn main() {}");

    assert_that(p.cargo_process("build"),
                execs().with_status(0));

    assert_that(&p.bin("foo_1"), not(existing_file()));
    assert_that(&p.bin("foo_2"), existing_file());

    assert_that(p.cargo_process("build").arg("--features").arg("c"),
                execs().with_status(0));

    assert_that(&p.bin("foo_1"), existing_file());
    assert_that(&p.bin("foo_2"), existing_file());

    assert_that(p.cargo_process("build").arg("--no-default-features"),
                execs().with_status(0));

    assert_that(&p.bin("foo_1"), not(existing_file()));
    assert_that(&p.bin("foo_2"), not(existing_file()));
}

#[test]
fn build_example_default_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a"]
            a = []

            [[example]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("examples/foo.rs", "fn main() {}");

    assert_that(p.cargo_process("build").arg("--example=foo"),
                execs().with_status(0));
    assert_that(&p.bin("examples/foo"), existing_file());

    assert_that(p.cargo_process("build").arg("--example=foo").arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
error: target `foo` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
"));
    assert_that(&p.bin("examples/foo"), not(existing_file()));
}

#[test]
fn build_example_arg_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            a = []

            [[example]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("examples/foo.rs", "fn main() {}");

    assert_that(p.cargo_process("build").arg("--example=foo").arg("--features").arg("a"),
                execs().with_status(0));
    assert_that(&p.bin("examples/foo"), existing_file());
}

#[test]
fn build_example_multiple_required_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a", "b"]
            a = []
            b = ["a"]
            c = []

            [[example]]
            name = "foo_1"
            required-features = ["b", "c"]

            [[example]]
            name = "foo_2"
            required-features = ["a"]
        "#)
        .file("examples/foo_1.rs", "fn main() {}")
        .file("examples/foo_2.rs", "fn main() {}");

    assert_that(p.cargo_process("build").arg("--example=foo_1"),
                execs().with_status(101).with_stderr("\
error: target `foo_1` requires the features: `b`, `c`
Consider enabling them by passing e.g. `--features=\"b c\"`
"));
    assert_that(p.cargo("build").arg("--example=foo_2"),
                execs().with_status(0));

    assert_that(&p.bin("examples/foo_1"), not(existing_file()));
    assert_that(&p.bin("examples/foo_2"), existing_file());

    assert_that(p.cargo_process("build").arg("--example=foo_1")
                .arg("--features").arg("c"),
                execs().with_status(0));
    assert_that(p.cargo("build").arg("--example=foo_2")
                .arg("--features").arg("c"),
                execs().with_status(0));

    assert_that(&p.bin("examples/foo_1"), existing_file());
    assert_that(&p.bin("examples/foo_2"), existing_file());

    assert_that(p.cargo_process("build").arg("--example=foo_1")
                .arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
error: target `foo_1` requires the features: `b`, `c`
Consider enabling them by passing e.g. `--features=\"b c\"`
"));
    assert_that(p.cargo("build").arg("--example=foo_2")
                .arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
error: target `foo_2` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
"));

    assert_that(&p.bin("examples/foo_1"), not(existing_file()));
    assert_that(&p.bin("examples/foo_2"), not(existing_file()));
}

#[test]
fn test_default_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a"]
            a = []

            [[test]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("tests/foo.rs", "#[test]\nfn test() {}");

    assert_that(p.cargo_process("test"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] target[/]debug[/]deps[/]foo-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

"));

    assert_that(p.cargo_process("test").arg("--no-default-features"),
                execs().with_status(0).with_stderr(format!("\
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]"))
                .with_stdout(""));

    assert_that(p.cargo_process("test").arg("--test=foo"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] target[/]debug[/]deps[/]foo-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

"));

    assert_that(p.cargo_process("test").arg("--test=foo").arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
error: target `foo` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
"));
}

#[test]
fn test_arg_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            a = []

            [[test]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("tests/foo.rs", "#[test]\nfn test() {}");

    assert_that(p.cargo_process("test").arg("--features").arg("a"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] target[/]debug[/]deps[/]foo-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

"));
}

#[test]
fn test_multiple_required_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a", "b"]
            a = []
            b = ["a"]
            c = []

            [[test]]
            name = "foo_1"
            required-features = ["b", "c"]

            [[test]]
            name = "foo_2"
            required-features = ["a"]
        "#)
        .file("tests/foo_1.rs", "#[test]\nfn test() {}")
        .file("tests/foo_2.rs", "#[test]\nfn test() {}");

    assert_that(p.cargo_process("test"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] target[/]debug[/]deps[/]foo_2-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

"));

    assert_that(p.cargo_process("test").arg("--features").arg("c"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] target[/]debug[/]deps[/]foo_1-[..][EXE]
[RUNNING] target[/]debug[/]deps[/]foo_2-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured


running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

"));

    assert_that(p.cargo_process("test").arg("--no-default-features"),
                execs().with_status(0).with_stderr(format!("\
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]"))
                .with_stdout(""));
}

#[test]
fn bench_default_features() {
    if !is_nightly() {
        return;
    }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a"]
            a = []

            [[bench]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("benches/foo.rs", r#"
            #![feature(test)]
            extern crate test;

            #[bench]
            fn bench(_: &mut test::Bencher) {
            }"#);

    assert_that(p.cargo_process("bench"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test bench ... bench: [..] 0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

"));

    assert_that(p.cargo_process("bench").arg("--no-default-features"),
                execs().with_status(0).with_stderr(format!("\
[FINISHED] release [optimized] target(s) in [..]"))
                .with_stdout(""));

    assert_that(p.cargo_process("bench").arg("--bench=foo"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test bench ... bench: [..] 0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

"));

    assert_that(p.cargo_process("bench").arg("--bench=foo").arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
error: target `foo` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
"));
}

#[test]
fn bench_arg_features() {
    if !is_nightly() {
        return;
    }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            a = []

            [[bench]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("benches/foo.rs", r#"
            #![feature(test)]
            extern crate test;

            #[bench]
            fn bench(_: &mut test::Bencher) {
            }"#);

    assert_that(p.cargo_process("bench").arg("--features").arg("a"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test bench ... bench: [..] 0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

"));
}

#[test]
fn bench_multiple_required_features() {
    if !is_nightly() {
        return;
    }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a", "b"]
            a = []
            b = ["a"]
            c = []

            [[bench]]
            name = "foo_1"
            required-features = ["b", "c"]

            [[bench]]
            name = "foo_2"
            required-features = ["a"]
        "#)
        .file("benches/foo_1.rs", r#"
            #![feature(test)]
            extern crate test;

            #[bench]
            fn bench(_: &mut test::Bencher) {
            }"#)
        .file("benches/foo_2.rs", r#"
            #![feature(test)]
            extern crate test;

            #[bench]
            fn bench(_: &mut test::Bencher) {
            }"#);

    assert_that(p.cargo_process("bench"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo_2-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test bench ... bench: [..] 0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

"));

    assert_that(p.cargo_process("bench").arg("--features").arg("c"),
                execs().with_status(0).with_stderr(format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo_1-[..][EXE]
[RUNNING] target[/]release[/]deps[/]foo_2-[..][EXE]", p.url()))
                .with_stdout("
running 1 test
test bench ... bench: [..] 0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured


running 1 test
test bench ... bench: [..] 0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

"));

    assert_that(p.cargo_process("bench").arg("--no-default-features"),
                execs().with_status(0).with_stderr(format!("\
[FINISHED] release [optimized] target(s) in [..]"))
                .with_stdout(""));
}

#[test]
fn install_default_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a"]
            a = []

            [[bin]]
            name = "foo"
            required-features = ["a"]

            [[example]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("examples/foo.rs", "fn main() {}");

    assert_that(p.cargo_process("install"),
                execs().with_status(0));
    assert_that(cargo_home(), has_installed_exe("foo"));
    assert_that(p.cargo_process("uninstall").arg("foo"),
                execs().with_status(0));

    assert_that(p.cargo_process("install").arg("--no-default-features"),
                execs().with_status(101).with_stderr(format!("\
[INSTALLING] foo v0.0.1 ([..])
[FINISHED] release [optimized] target(s) in [..]
[ERROR] no binaries are available for install using the selected features
")));
    assert_that(cargo_home(), not(has_installed_exe("foo")));

    assert_that(p.cargo_process("install").arg("--bin=foo"),
                execs().with_status(0));
    assert_that(cargo_home(), has_installed_exe("foo"));
    assert_that(p.cargo_process("uninstall").arg("foo"),
                execs().with_status(0));

    assert_that(p.cargo_process("install").arg("--bin=foo").arg("--no-default-features"),
                execs().with_status(101).with_stderr(format!("\
[INSTALLING] foo v0.0.1 ([..])
[ERROR] failed to compile `foo v0.0.1 ([..])`, intermediate artifacts can be found at \
    `[..]target`

Caused by:
  target `foo` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
")));
    assert_that(cargo_home(), not(has_installed_exe("foo")));

    assert_that(p.cargo_process("install").arg("--example=foo"),
                execs().with_status(0));
    assert_that(cargo_home(), has_installed_exe("foo"));
    assert_that(p.cargo_process("uninstall").arg("foo"),
                execs().with_status(0));

    assert_that(p.cargo_process("install").arg("--example=foo").arg("--no-default-features"),
                execs().with_status(101).with_stderr(format!("\
[INSTALLING] foo v0.0.1 ([..])
[ERROR] failed to compile `foo v0.0.1 ([..])`, intermediate artifacts can be found at \
    `[..]target`

Caused by:
  target `foo` requires the features: `a`
Consider enabling them by passing e.g. `--features=\"a\"`
")));
    assert_that(cargo_home(), not(has_installed_exe("foo")));
}

#[test]
fn install_arg_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            a = []

            [[bin]]
            name = "foo"
            required-features = ["a"]
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("install").arg("--features").arg("a"),
                execs().with_status(0));
    assert_that(cargo_home(), has_installed_exe("foo"));
    assert_that(p.cargo_process("uninstall").arg("foo"),
                execs().with_status(0));
}

#[test]
fn install_multiple_required_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [features]
            default = ["a", "b"]
            a = []
            b = ["a"]
            c = []

            [[bin]]
            name = "foo_1"
            path = "src/foo_1.rs"
            required-features = ["b", "c"]

            [[bin]]
            name = "foo_2"
            path = "src/foo_2.rs"
            required-features = ["a"]
        "#)
        .file("src/foo_1.rs", "fn main() {}")
        .file("src/foo_2.rs", "fn main() {}");

    assert_that(p.cargo_process("install"),
                execs().with_status(0));
    assert_that(cargo_home(), not(has_installed_exe("foo_1")));
    assert_that(cargo_home(), has_installed_exe("foo_2"));
    assert_that(p.cargo_process("uninstall").arg("foo"),
                execs().with_status(0));

    assert_that(p.cargo_process("install").arg("--features").arg("c"),
                execs().with_status(0));
    assert_that(cargo_home(), has_installed_exe("foo_1"));
    assert_that(cargo_home(), has_installed_exe("foo_2"));
    assert_that(p.cargo_process("uninstall").arg("foo"),
                execs().with_status(0));

    assert_that(p.cargo_process("install").arg("--no-default-features"),
                execs().with_status(101).with_stderr("\
[INSTALLING] foo v0.0.1 ([..])
[FINISHED] release [optimized] target(s) in [..]
[ERROR] no binaries are available for install using the selected features
"));
    assert_that(cargo_home(), not(has_installed_exe("foo_1")));
    assert_that(cargo_home(), not(has_installed_exe("foo_2")));
}
