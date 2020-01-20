pub mod sql_generator;

#[cfg(test)]
mod tests {
    use super::sql_generator::*;

    #[test]
    fn test_generate_where_clause() {
        let mut t = Table::new("test_table");
        t.add_key("key01", "key01")
            .add_key("foo", "key02");

        let qs1 = vec![
            Query::from_int("key01", QueryCondition::Equals, 100),
        ];
        if let Some(clause) = t.generate_where_clause(&qs1) {
            assert_eq!(clause.clause, "key01 = ?");
            if let ParameterType::Int(x) = *clause.parameters[0].value {
                assert_eq!(x, 100);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let qs2 = vec![
            Query::from_int("foo", QueryCondition::Greater, 1),
        ];
        if let Some(clause) = t.generate_where_clause(&qs2) {
            assert_eq!(clause.clause, "key02 > ?");
            if let ParameterType::Int(x) = *clause.parameters[0].value {
                assert_eq!(x, 1);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}
