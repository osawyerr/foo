use pgx::prelude::*;

pgx::pg_module_magic!();

extension_sql_file!("../sql/_bootstrap.sql", bootstrap);

#[pg_extern]
fn hello_foo() -> &'static str {
    "Hello, foo"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_foo() {
        assert_eq!("Hello, foo", crate::hello_foo());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
