
pub enum ParameterType {
    Int(i32),
    String(String),
}

pub struct Parameter {
    pub value: Box<ParameterType>,
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

pub enum QueryOperator {
    And,
    Or,
}

pub struct Query {
    target_key: String,
    condition: QueryCondition,
    operator: QueryOperator,
    parameter: Parameter,
}

impl Query {
    pub fn from_int(target: &str,
                    cond: QueryCondition,
                    param: i32) -> Query {
        Query {
            target_key: String::from(target),
            condition: cond,
            operator: QueryOperator::And,
            parameter: Parameter {
                value: Box::new(ParameterType::Int(param)),
            },
        }
    }
}

pub struct Key {
    name: String,
    alias: String,
}

pub struct Table {
    name: String,
    keys: Vec<Key>,
    placeholder: String,
}

impl Table {
    pub fn new(name: &str) -> Table {
        Table {
            name: String::from(name),
            keys: Vec::new(),
            placeholder: "?".to_string(),
        }
    }

    pub fn add_key(&mut self, name: &str, alias: &str) -> &mut Table {
        self.keys.push(Key {
            name: String::from(name),
            alias: String::from(alias),
        });
        self
    }

    fn decode_cond(&self, key: &String, query: &Query) -> String {
        let cond = match query.condition {
            QueryCondition::Equals => "=",
            QueryCondition::Greater => ">",
            QueryCondition::Lesser => "<",
            QueryCondition::GreaterEqual => ">=",
            QueryCondition::LesserEqual => "<=",
        };
        format!("{} {} {}", key, cond, self.placeholder)
    }

    pub fn generate_where_clause(&self, queries: &Vec<Query>) -> Option<WhereClause> {
        let mut clauses: Vec<String> = Vec::new();
        let mut parameters: Vec<Parameter> = Vec::new();

        // scan target key
        for q in queries {
            if let Some(k) = self.keys.iter().find(|&x| x.name == q.target_key) {
                clauses.push(self.decode_cond(&k.alias, q));
                let param = Parameter {
                    value: match &*q.parameter.value {
                        ParameterType::Int(x)
                            => Box::new(ParameterType::Int(*x)),
                        ParameterType::String(x)
                            => Box::new(ParameterType::String(x.to_string())),
                    },
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
