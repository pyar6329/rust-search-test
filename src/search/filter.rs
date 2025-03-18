pub fn foobar() {
    // let x = r#"" OR true AND foo = ""#;
    let x = r#"abc"def\\ghi"#;

    let escaped_x = espcape_filter(x);

    let query = format!(
        r#"
        id = {} AND name = "{}"
    "#,
        10, escaped_x
    );
    println!("{}", query);
}

fn espcape_filter(s: &str) -> String {
    // " -> \"
    let replaced_slash = s.replace(r#"\"#, r#"\\"#);
    replaced_slash.replace(r#"""#, r#"\""#)
}
