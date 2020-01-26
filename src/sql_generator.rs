use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParameterType {
    Int(i32),
    String(String),
}

pub struct Parameter {
    pub value: Box<ParameterType>,
}

impl Clone for Parameter {
    fn clone(&self) -> Parameter {
        Parameter {
            value: match &*self.value {
                ParameterType::Int(x)
                    => Box::new(ParameterType::Int(*x)),
                ParameterType::String(x)
                    => Box::new(ParameterType::String(x.to_string())),
            },
        }
    }
}

pub struct Clause {
    pub clause: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub enum QueryCondition {
    Equals,
    Greater,
    Lesser,
    GreaterEqual,
    LesserEqual,
    Desc,
    Asc,
    None,
}

pub enum QueryOperator {
    And,
    Or,
}

pub struct Query {
    target: String,
    condition: QueryCondition,
    operator: QueryOperator,
    parameter: Parameter,
}

impl Query {
    pub fn from_int(target: &str,
                    cond: QueryCondition,
                    param: i32) -> Query {
        Query {
            target: String::from(target),
            condition: cond,
            operator: QueryOperator::And,
            parameter: Parameter {
                value: Box::new(ParameterType::Int(param)),
            },
        }
    }

    pub fn from_str(target: &str,
                    cond: QueryCondition,
                    param: &str) -> Query {
        Query {
            target: String::from(target),
            condition: cond,
            operator: QueryOperator::And,
            parameter: Parameter {
                value: Box::new(ParameterType::String(param.to_string())),
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

#[derive(Debug, PartialEq)]
pub enum SqlGenerateError {
    InvalidCondition,
}

impl fmt::Display for SqlGenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SqlGenerateError::InvalidCondition
                => write!(f, "invalid condition"),
        }
    }
}

impl Error for SqlGenerateError {
    fn description(&self) -> &str {
        match *self {
            SqlGenerateError::InvalidCondition
                => "invalid condition",
        }
    }
    
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SqlGenerateError::InvalidCondition => None,
        }
    }
}

impl Table {
    pub fn new(name: &str) -> Table {
        Table {
            name: String::from(name),
            keys: Vec::new(),
            placeholder: "?".to_string(),
        }
    }

    pub fn add_key_and_alias(&mut self, name: &str, alias: &str) -> &mut Table {
        self.keys.push(Key {
            name: String::from(name),
            alias: String::from(alias),
        });
        self
    }

    fn decode_cond(&self, key: &String, query: &Query) -> Result<String, SqlGenerateError> {
        let cond = match query.condition {
            QueryCondition::Equals => "=",
            QueryCondition::Greater => ">",
            QueryCondition::Lesser => "<",
            QueryCondition::GreaterEqual => ">=",
            QueryCondition::LesserEqual => "<=",
            _ => return Result::Err(SqlGenerateError::InvalidCondition),
        };
        Result::Ok(format!("{} {} {}", key, cond, self.placeholder))
    }

    pub fn generate_where_clause(&self, queries: &Vec<Query>)
                                 -> Result<Clause, SqlGenerateError> {
        let mut clauses: Vec<String> = Vec::new();
        let mut parameters: Vec<Parameter> = Vec::new();

        // scan target key
        for q in queries {
            if let Some(k) = self.keys.iter().find(|&x| x.alias == q.target) {
                match self.decode_cond(&k.name, q) {
                    Ok(clause) => clauses.push(clause),
                    Err(err) => return Err(err),
                }
                parameters.push(q.parameter.clone());
            } else {
                continue;
            }
        }

        let result = Clause {
            clause: clauses.join(", "),
            parameters: parameters
        };
        Ok(result)
    }

    pub fn generate_limit_clause(&self, queries: &Vec<Query>)
                                 -> Result<Clause, SqlGenerateError> {
        let mut clauses: Vec<String> = Vec::new();
        let mut parameters: Vec<Parameter> = Vec::new();

        // scan target key
        for q in queries {
            if q.target == "limit" {
                clauses.push(format!("LIMIT {}", self.placeholder));
                parameters.push(q.parameter.clone());
                continue;
            }
            if q.target == "offset" {
                clauses.push(format!("OFFSET {}", self.placeholder));
                parameters.push(q.parameter.clone());
                continue;
            }
        }

        let result = Clause {
            clause: clauses.join(" "),
            parameters: parameters
        };
        Ok(result)
    }

    pub fn generate_order_by_clause(&self, queries: &Vec<Query>)
                                 -> Result<Clause, SqlGenerateError> {
        let mut clauses: Vec<String> = Vec::new();
        let parameters: Vec<Parameter> = Vec::new();

        // scan target key
        for q in queries {
            if q.target == "order_by" {
                if let ParameterType::String(key) = &*q.parameter.value {
                    if let Some(k) = self.keys.iter().find(|x| &x.alias == key) {
                        let order = match q.condition {
                            QueryCondition::Asc => "ASC",
                            QueryCondition::Desc => "DESC",
                            _ => return Result::Err(SqlGenerateError::InvalidCondition),
                        };
                        clauses.push(format!("{} {}", k.name, order));
                        continue;
                    }
                }
            }
        }

        let result = Clause {
            clause: clauses.join(", "),
            parameters: parameters
        };
        Ok(result)
    }
}

