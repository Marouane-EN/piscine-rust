use error_types::*;

#[test]
fn form_validation_flow() {
    let mut form_output = Form {
        name: "Lee".to_owned(),
        password: "qwqwsa1dty_".to_owned(),
    };

    assert!(form_output.validate().is_ok());

    form_output.name = "".to_owned();
    let err = form_output.validate().unwrap_err();
    assert_eq!(err.err, "Username is empty");
    assert_eq!(err.form_values.0, "name");
    assert_eq!(err.form_values.1, "");

    form_output.name = "as".to_owned();
    form_output.password = "dty_1".to_owned();
    let err = form_output.validate().unwrap_err();
    assert_eq!(err.err, "Password should be at least 8 characters long");
    assert_eq!(err.form_values.0, "password");
    assert_eq!(err.form_values.1, "as");

    form_output.password = "asdasASd(_".to_owned();
    let err = form_output.validate().unwrap_err();
    assert_eq!(err.err, "Password should be a combination of ASCII numbers, letters and symbols");
    assert_eq!(err.form_values.0, "password");
    assert_eq!(err.form_values.1, "as");

    form_output.password = "asdasASd123SA".to_owned();
    let err = form_output.validate().unwrap_err();
    assert_eq!(err.err, "Password should be a combination of ASCII numbers, letters and symbols");
    assert_eq!(err.form_values.0, "password");
    assert_eq!(err.form_values.1, "as");
}
