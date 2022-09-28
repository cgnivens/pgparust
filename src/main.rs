use std::str::FromStr;

pub fn main() {
    println!("hello world");
}

enum QueryType {
    CREATE,
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}

impl FromStr for QueryType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() { // not case-sensitive, so force case
            "CREATE" => Ok(QueryType::CREATE),
            "SELECT" => Ok(QueryType::SELECT),
            "INSERT" => Ok(QueryType::INSERT),
            "UPDATE" => Ok(QueryType::UPDATE),
            "DELETE" => Ok(QueryType::DELETE),
            _ => Err("Unknown query type".into())
        }
    }
}

struct Query {
    type: QueryType,
    table_name: String,
    conditions: Vec<Condition>
}
