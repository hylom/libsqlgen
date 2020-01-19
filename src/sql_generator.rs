#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterType {
    Int,
    String,
}

pub struct Parameter {
    pub int_value: i32,
    pub str_value: String,
    pub parameter_type: ParameterType,
}

pub struct WhereClause {
    pub clause: String,
    pub parameters: Vec<Parameter>,
}

pub enum QueryCondition {
    Equals,
    Greater,
    Lesser,
    GreaterEqual,
    LesserEqual,
}

pub struct Query {
    target_key: String,
    condition: QueryCondition,
    parameter: Parameter,
}

impl Query {
    pub fn from_int(target: &str,
                    cond: QueryCondition,
                    param: i32) -> Query {
        Query {
            target_key: String::from(target),
            condition: cond,
            parameter: Parameter {
                int_value: param,
                str_value: "".to_string(),
                parameter_type: ParameterType::Int,
            },
        }
    }
}


pub struct Table {
    name: String,
    keys: Vec<String>,
}

impl Table {
    pub fn new(name: &str) -> Table {
        Table {
            name: String::from(name),
            keys: Vec::new(),
        }
    }

    pub fn add_key(&mut self, key_name: &str) -> &mut Table {
        self.keys.push(String::from(key_name));
        self
    }

    pub fn generate_where_clause(&self, queries: &Vec<Query>) -> Option<WhereClause> {
        let mut clauses: Vec<String> = Vec::new();
        let mut parameters: Vec<Parameter> = Vec::new();

        // scan target key
        for q in queries {
            if let Some(k) = self.keys.iter().find(|&x| x == &q.target_key) {
                let cond = match q.condition {
                    QueryCondition::Equals => "=",
                    QueryCondition::Greater => ">",
                    QueryCondition::Lesser => "<",
                    QueryCondition::GreaterEqual => ">=",
                    QueryCondition::LesserEqual => "<=",
                };
                clauses.push(format!("{} {} ?", k, cond));
                let param = Parameter {
                    int_value: q.parameter.int_value,
                    str_value: String::from(&q.parameter.str_value),
                    parameter_type: q.parameter.parameter_type,
                };
                parameters.push(param);
            } else {
                continue;
            }
        }

        if clauses.len() == 0 {
            return None;
        }
        let result = WhereClause {
            clause: clauses.join(", "),
            parameters: parameters
        };
        Some(result)
    }
}
