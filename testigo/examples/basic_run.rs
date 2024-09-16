use testigo::prelude::*;

fn main() {
    let test_suit = TestigoBuilder::<()>::new().build::<String>();

    test_suit.execute();
}

#[testigo(name = "panicing test")]
fn panicing_test(_: &()) -> Result<(), String> {
    assert!(false);
    Ok(())
}

#[testigo(name = "test1")]
fn test1(_: &()) -> Result<(), String> {
    Ok(())
}

#[testigo]
fn test_is_func_name(_: &()) -> Result<(), String> {
    Ok(())
}
