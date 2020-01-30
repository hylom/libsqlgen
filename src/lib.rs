pub mod sql_generator;
use sql_generator::*;
use std::os::raw::{c_int, c_char};
use std::ptr;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn new_table(name: &str) -> *mut Table {
    let ptr = Box::new(Table::new(name));
    Box::into_raw(ptr)
}

#[no_mangle]
pub extern "C" fn free_table(table_ptr: *mut Table) {
    unsafe { Box::from_raw(table_ptr) };
}

#[no_mangle]
pub extern "C" fn add_key_and_alias_to_table(table_ptr: *mut Table,
                                             name_ptr: *const c_char,
                                             alias_ptr: *const c_char) {
    unsafe {
        let name = CStr::from_ptr(name_ptr);
        let alias = CStr::from_ptr(alias_ptr);
        (*table_ptr).add_key_and_alias(name.to_str().unwrap(),
                                       alias.to_str().unwrap());
    };
}

#[no_mangle]
pub extern "C" fn new_queries() -> *mut Vec<Query> {
    let queries: Vec<Query> = Vec::new();
    let ptr = Box::new(queries);
    Box::into_raw(ptr)
}

#[no_mangle]
pub extern "C" fn free_queries(queries_ptr: *mut Vec<Query>) {
    unsafe { Box::from_raw(queries_ptr) };
}

#[no_mangle]
pub extern "C" fn add_query_from_int(vec_ptr: *mut Vec<Query>,
                                     target_ptr: *const c_char,
                                     condition_ptr: *const c_char,
                                     param: c_int) {
    unsafe {
        let target = CStr::from_ptr(target_ptr);
        let condition = CStr::from_ptr(condition_ptr);

        let cond = match condition.to_str() {
            Ok("eq") => QueryCondition::Equals,
            Ok("gt") => QueryCondition::Greater,
            Ok("lt") => QueryCondition::Lesser,
            Ok("ge") => QueryCondition::GreaterEqual,
            Ok("le") => QueryCondition::LesserEqual,
            Ok("desc") => QueryCondition::Desc,
            Ok("asc") => QueryCondition::Asc,
            _ => QueryCondition::None,
        };
        let query = Query::from_int(target.to_str().unwrap(), cond, param);
        (*vec_ptr).push(query);
    }
}

#[no_mangle]
pub extern "C" fn genereate_where_clause(table_ptr: *mut Table,
                                         queries_ptr: *mut Vec<Query>)
                                         -> *mut Clause {
    unsafe {
        if let Ok(clause) = (*table_ptr).generate_where_clause(&*queries_ptr) {
            let ptr = Box::new(clause);
            Box::into_raw(ptr)
        } else {
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn free_clause(clause_ptr: *mut Clause) {
    unsafe { Box::from_raw(clause_ptr) };
}

#[no_mangle]
pub extern "C" fn show_clause(clause_ptr: *mut Clause) {
    println!("{}", unsafe { &(*clause_ptr).clause });
}


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
