use rssr::bun;

#[test]
fn it_checks_that_bun_is_in_the_environment() {
    let bun_version_option = bun::check_for_bun_in_env();
    let hardcoded = "1.1.38".to_string();
    assert_eq!(bun_version_option.unwrap(), hardcoded);
}
