use std::collections::HashSet;
// use std::str::FromStr;

pub fn is_reserved(token: String) -> bool {
    let reserved = populate_hashset();
    reserved.contains(&token)
}

// impl FromStr for TokenType {
//     type Err = String;
//
//     fn from_str(other: &str) -> Result<TokenType, String> {
//
//
//         if reserved.contains(other) {
//             Ok(TokenType::Reserved(other.to_string()))
//         } else {
//             Ok(TokenType::Other)
//         }
//     }
// }
//
//
// impl From<String> for TokenType {
//     fn from(other: String) -> TokenType {
//         let reserved = populate_hashset();
//
//         if reserved.contains(&other) {
//             TokenType::Reserved(other)
//         } else {
//             TokenType::Other
//         }
//     }
// }

fn populate_hashset<'a>() -> HashSet<String> {
    let mut hashset = HashSet::new();
    hashset.insert("AES128".to_string());
    hashset.insert("AES256".to_string());
    hashset.insert("ALL".to_string());
    hashset.insert("ALLOWOVERWRITE".to_string());
    hashset.insert("ANALYSE".to_string());
    hashset.insert("ANALYZE".to_string());
    hashset.insert("AND".to_string());
    hashset.insert("ANY".to_string());
    hashset.insert("ARRAY".to_string());
    hashset.insert("AS".to_string());
    hashset.insert("ASC".to_string());
    hashset.insert("AUTHORIZATION".to_string());
    hashset.insert("AZ64".to_string());
    hashset.insert("BACKUP".to_string());
    hashset.insert("BETWEEN".to_string());
    hashset.insert("BINARY".to_string());
    hashset.insert("BLANKSASNULL".to_string());
    hashset.insert("BOTH".to_string());
    hashset.insert("BYTEDICT".to_string());
    hashset.insert("BZIP2".to_string());
    hashset.insert("CASE".to_string());
    hashset.insert("CAST".to_string());
    hashset.insert("CHECK".to_string());
    hashset.insert("COLLATE".to_string());
    hashset.insert("COLUMN".to_string());
    hashset.insert("CONSTRAINT".to_string());
    hashset.insert("CREATE".to_string());
    hashset.insert("CREDENTIALS".to_string());
    hashset.insert("CROSS".to_string());
    hashset.insert("CURRENT_DATE".to_string());
    hashset.insert("CURRENT_TIME".to_string());
    hashset.insert("CURRENT_TIMESTAMP".to_string());
    hashset.insert("CURRENT_USER".to_string());
    hashset.insert("CURRENT_USER_ID".to_string());
    hashset.insert("DEFAULT".to_string());
    hashset.insert("DEFERRABLE".to_string());
    hashset.insert("DEFLATE".to_string());
    hashset.insert("DEFRAG".to_string());
    hashset.insert("DELTA".to_string());
    hashset.insert("DELTA32K".to_string());
    hashset.insert("DESC".to_string());
    hashset.insert("DISABLE".to_string());
    hashset.insert("DISTINCT".to_string());
    hashset.insert("DO".to_string());
    hashset.insert("ELSE".to_string());
    hashset.insert("EMPTYASNULL".to_string());
    hashset.insert("ENABLE".to_string());
    hashset.insert("ENCODE".to_string());
    hashset.insert("ENCRYPT".to_string());
    hashset.insert("ENCRYPTION".to_string());
    hashset.insert("END".to_string());
    hashset.insert("EXCEPT".to_string());
    hashset.insert("EXPLICIT".to_string());
    hashset.insert("FALSE".to_string());
    hashset.insert("FOR".to_string());
    hashset.insert("FOREIGN".to_string());
    hashset.insert("FREEZE".to_string());
    hashset.insert("FROM".to_string());
    hashset.insert("FULL".to_string());
    hashset.insert("GLOBALDICT256".to_string());
    hashset.insert("GLOBALDICT64K".to_string());
    hashset.insert("GRANT".to_string());
    hashset.insert("GROUP".to_string());
    hashset.insert("GZIP".to_string());
    hashset.insert("HAVING".to_string());
    hashset.insert("IDENTITY".to_string());
    hashset.insert("IGNORE".to_string());
    hashset.insert("ILIKE".to_string());
    hashset.insert("IN".to_string());
    hashset.insert("INITIALLY".to_string());
    hashset.insert("INNER".to_string());
    hashset.insert("INTERSECT".to_string());
    hashset.insert("INTERVAL".to_string());
    hashset.insert("INTO".to_string());
    hashset.insert("IS".to_string());
    hashset.insert("ISNULL".to_string());
    hashset.insert("JOIN".to_string());
    hashset.insert("LANGUAGE".to_string());
    hashset.insert("LEADING".to_string());
    hashset.insert("LEFT".to_string());
    hashset.insert("LIKE".to_string());
    hashset.insert("LIMIT".to_string());
    hashset.insert("LOCALTIME".to_string());
    hashset.insert("LOCALTIMESTAMP".to_string());
    hashset.insert("LUN".to_string());
    hashset.insert("LUNS".to_string());
    hashset.insert("LZO".to_string());
    hashset.insert("LZOP".to_string());
    hashset.insert("MINUS".to_string());
    hashset.insert("MOSTLY16".to_string());
    hashset.insert("MOSTLY32".to_string());
    hashset.insert("MOSTLY8".to_string());
    hashset.insert("NATURAL".to_string());
    hashset.insert("NEW".to_string());
    hashset.insert("NOT".to_string());
    hashset.insert("NOTNULL".to_string());
    hashset.insert("NULL".to_string());
    hashset.insert("NULLS".to_string());
    hashset.insert("OFF".to_string());
    hashset.insert("OFFLINE".to_string());
    hashset.insert("OFFSET".to_string());
    hashset.insert("OID".to_string());
    hashset.insert("OLD".to_string());
    hashset.insert("ON".to_string());
    hashset.insert("ONLY".to_string());
    hashset.insert("OPEN".to_string());
    hashset.insert("OR".to_string());
    hashset.insert("ORDER".to_string());
    hashset.insert("OUTER".to_string());
    hashset.insert("OVERLAPS".to_string());
    hashset.insert("PARALLEL".to_string());
    hashset.insert("PARTITION".to_string());
    hashset.insert("PERCENT".to_string());
    hashset.insert("PERMISSIONS".to_string());
    hashset.insert("PIVOT".to_string());
    hashset.insert("PLACING".to_string());
    hashset.insert("PRIMARY".to_string());
    hashset.insert("RAW".to_string());
    hashset.insert("READRATIO".to_string());
    hashset.insert("RECOVER".to_string());
    hashset.insert("REFERENCES".to_string());
    hashset.insert("REJECTLOG".to_string());
    hashset.insert("RESORT".to_string());
    hashset.insert("RESPECT".to_string());
    hashset.insert("RESTORE".to_string());
    hashset.insert("RIGHT".to_string());
    hashset.insert("SELECT".to_string());
    hashset.insert("SESSION_USER".to_string());
    hashset.insert("SIMILAR".to_string());
    hashset.insert("SNAPSHOT".to_string());
    hashset.insert("SOME".to_string());
    hashset.insert("SYSDATE".to_string());
    hashset.insert("SYSTEM".to_string());
    hashset.insert("TABLE".to_string());
    hashset.insert("TAG".to_string());
    hashset.insert("TDES".to_string());
    hashset.insert("TEXT255".to_string());
    hashset.insert("TEXT32K".to_string());
    hashset.insert("THEN".to_string());
    hashset.insert("TIMESTAMP".to_string());
    hashset.insert("TO".to_string());
    hashset.insert("TOP".to_string());
    hashset.insert("TRAILING".to_string());
    hashset.insert("TRUE".to_string());
    hashset.insert("TRUNCATECOLUMNS".to_string());
    hashset.insert("UNION".to_string());
    hashset.insert("UNIQUE".to_string());
    hashset.insert("UNNEST".to_string());
    hashset.insert("UNPIVOT".to_string());
    hashset.insert("USER".to_string());
    hashset.insert("USING".to_string());
    hashset.insert("VERBOSE".to_string());
    hashset.insert("WALLET".to_string());
    hashset.insert("WHEN".to_string());
    hashset.insert("WHERE".to_string());
    hashset.insert("WITH".to_string());
    hashset.insert("WITHOUT".to_string());
    hashset
}
