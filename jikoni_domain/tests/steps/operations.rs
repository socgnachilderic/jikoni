use bdd::mult;
use cucumber::{given, when, then};

use crate::MyWorld;

#[derive(Debug,)]
pub enum Operations {
    Init,
    Input(i32, i32),
    Result(i32),
    Error,
}

#[given(expr = r#"the numbers "{int}" and "{int}""#)]
fn i_have_two_numbers(world: &mut MyWorld, num1: i32, num2: i32) {
    world.ops = Operations::Input(num1, num2);
}

#[when(expr = "the User multiply them")]
fn add_two_numbers(world: &mut MyWorld) {
    world.ops = match world.ops {
        Operations::Input(num1, num2) => {
            let ops_result = mult(num1, num2);
            Operations::Result(ops_result)
        }
        _ => Operations::Error,
    };
}

#[then(expr = "the User gets {int} as result")]
fn return_result(world: &mut MyWorld, expected_result: i32) {
    match world.ops {
        Operations::Result(result) => assert_eq!(expected_result, result),
        _ => panic!("this is error")
    };
}
