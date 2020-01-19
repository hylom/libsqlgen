pub mod sql_generator;

#[cfg(test)]
mod tests {
    use super::sql_generator::*;

    #[test]
    fn test_generate_where_clause() {
        let mut t = Table::new("test_table");
        t.add_key("key01")
            .add_key("key02");

        let qs1 = vec![
            Query::from_int("key01", QueryCondition::Equals, 100),
        ];
        if let Some(clause) = t.generate_where_clause(&qs1) {
            assert_eq!(clause.clause, "key01 = ?");
            assert_eq!(clause.parameters[0].int_value, 100);
            assert_eq!(clause.parameters[0].parameter_type, ParameterType::Int);
        } else {
            assert!(false);
        }

        // let qs2 = vec![
        //     Query::forInt("hogehoge", QueryCondition::Greater, 1),
        // ];
        // assert_eq!(t.generate_where_clause(&qs2), None);
    }
}
