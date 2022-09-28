enum QueryType {
    CREATE
    SELECT
    INSERT
    UPDATE
    DELETE
}

impl FromStr for QueryType {
    type Err = String;

    fn from_str(input: &str) -> Result<QueryType, Self.Err> {
        match input {
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
