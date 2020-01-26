pub mod sql_generator;

#[cfg(test)]
mod tests {
    use super::sql_generator::*;

    #[test]
    fn test_generate_where_clause() {
        let mut t = Table::new("test_table");
        t.add_key_and_alias("key01", "key01")
            .add_key_and_alias("key02", "foo");

        // test for '=' operator
        let qs1 = vec![
            Query::from_int("key01", QueryCondition::Equals, 100),
        ];
        if let Ok(clause) = t.generate_where_clause(&qs1) {
            assert_eq!(clause.clause, "key01 = ?");
            assert_eq!(*clause.parameters[0].value, ParameterType::Int(100));
        } else {
            assert!(false);
        }

        // test for '>' operator
        let qs2 = vec![
            Query::from_int("foo", QueryCondition::Greater, 1),
        ];
        if let Ok(clause) = t.generate_where_clause(&qs2) {
            assert_eq!(clause.clause, "key02 > ?");
            assert_eq!(*clause.parameters[0].value, ParameterType::Int(1));
        } else {
            assert!(false);
        }

        // test for invalid condition
        let qs3 = vec![
            Query::from_int("foo", QueryCondition::None, 1),
        ];
        if let Err(err) = t.generate_where_clause(&qs3) {
            assert_eq!(err, SqlGenerateError::InvalidCondition);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_generate_limit_clause() {
        let mut t = Table::new("test_table");
        t.add_key_and_alias("key01", "key01")
            .add_key_and_alias("key02", "foo");

        let qs1 = vec![
            Query::from_int("limit", QueryCondition::Equals, 10),
            Query::from_int("offset", QueryCondition::Equals, 4),
        ];
        if let Ok(clause) = t.generate_limit_clause(&qs1) {
            assert_eq!(clause.clause, "LIMIT ? OFFSET ?");
            assert_eq!(*clause.parameters[0].value, ParameterType::Int(10));
            assert_eq!(*clause.parameters[1].value, ParameterType::Int(4));
        } else {
            assert!(false);
        }

    }
    #[test]
    fn test_generate_order_by_clause() {
        let mut t = Table::new("test_table");
        t.add_key_and_alias("key01", "key01")
            .add_key_and_alias("key02", "foo");

        // test for 'ASC' order
        let qs1 = vec![
            Query::from_str("order_by", QueryCondition::Asc, "key01"),
        ];
        if let Ok(clause) = t.generate_order_by_clause(&qs1) {
            assert_eq!(clause.clause, "key01 ASC");
        } else {
            assert!(false);
        }

        // test for 'ASC' order
        let qs2 = vec![
            Query::from_str("order_by", QueryCondition::Desc, "foo"),
        ];
        if let Ok(clause) = t.generate_order_by_clause(&qs2) {
            assert_eq!(clause.clause, "key02 DESC");
        } else {
            assert!(false);
        }
    }
}
