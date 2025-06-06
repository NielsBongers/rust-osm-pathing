pub enum PathExamples {
    Weert,
}

pub fn get_path_example(path_example: &PathExamples) -> (u64, u64) {
    match path_example {
        PathExamples::Weert => (1946539965, 42254288),
    }
}
