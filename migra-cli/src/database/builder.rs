use super::prelude::*;

pub(crate) fn merge_query_with_params(query: &str, params: ToSqlParams) -> String {
    params
        .iter()
        .enumerate()
        .fold(query.to_string(), |acc, (i, p)| {
            str::replace(&acc, &format!("${}", i + 1), &p.to_sql())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_one_param_in_query() {
        assert_eq!(
            merge_query_with_params("SELECT $1", &[&"foo"]),
            "SELECT 'foo'"
        );
    }

    #[test]
    fn replace_two_params_in_query() {
        assert_eq!(
            merge_query_with_params("SELECT $1, $2", &[&"foo", &"bar"]),
            "SELECT 'foo', 'bar'"
        );
    }

    #[test]
    fn replace_all_bonds_in_query_with_first_param() {
        assert_eq!(
            merge_query_with_params("SELECT $1, $1", &[&"foo"]),
            "SELECT 'foo', 'foo'"
        );
    }
}
