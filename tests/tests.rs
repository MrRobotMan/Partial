use partial::partial;

fn output<F, T: std::fmt::Debug>(func: F) -> T
where
    F: Fn() -> T,
{
    println!("Calling func");

    let res = func();
    println!("result: {res:?}");
    res
}

fn mult(a: i32, b: i32) -> i32 {
    println!("Called mult");
    a * b
}

fn no_args() -> &'static str {
    "Hello"
}

#[test]
fn partial_with_static_inputs() {
    let res = partial!(mult, 3, 4);
    assert_eq!(12, res);
}

#[test]
fn partial_with_variable_inputs() {
    let res = 12;
    let res = partial!(mult, res, 4);
    assert_eq!(48, res);
}

#[test]
fn partial_with_no_args() {
    assert_eq!("Hello", partial!(no_args));
}

#[test]
fn example_use_case() {
    assert_eq!(-5, output(|| partial!(mult, -5, 1)));
}
