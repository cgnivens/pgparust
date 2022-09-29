use std::collections::HashSet;
// TODO: For the love of all that is holy
// please refactor this
pub fn is_reserved(token: String) -> bool {
    let reserved = populate_hashset();
    reserved.contains(&token.to_uppercase())
}

pub fn is_function(token: String) -> bool {
    let reserved = functions();
    reserved.contains(&token.to_uppercase())
}

fn populate_hashset() -> HashSet<String> {
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


fn functions() -> HashSet<String> {
    let mut hashset = HashSet::new();
    hashset.insert("ANY_VALUE".to_string());
    hashset.insert("APPROXIMATE PERCENTILE_DISC".to_string());
    hashset.insert("AVG".to_string());
    hashset.insert("COUNT".to_string());
    hashset.insert("LISTAGG".to_string());
    hashset.insert("MAX".to_string());
    hashset.insert("MEDIAN".to_string());
    hashset.insert("MIN".to_string());
    hashset.insert("PERCENTILE_CONT".to_string());
    hashset.insert("STDDEV_SAMP".to_string());
    hashset.insert("STDDEV_POP".to_string());
    hashset.insert("SUM".to_string());
    hashset.insert("VAR_SAMP".to_string());
    hashset.insert("VAR_POP".to_string());
    hashset.insert("array".to_string());
    hashset.insert("array_concat".to_string());
    hashset.insert("array_flatten".to_string());
    hashset.insert("get_array_length".to_string());
    hashset.insert("split_to_array".to_string());
    hashset.insert("subarray".to_string());
    hashset.insert("ADD_MONTHS".to_string());
    hashset.insert("AT TIME ZONE".to_string());
    hashset.insert("CONVERT_TIMEZONE".to_string());
    hashset.insert("CURRENT_DATE".to_string());
    hashset.insert("DATE_CMP".to_string());
    hashset.insert("DATE_CMP_TIMESTAMP".to_string());
    hashset.insert("DATE_CMP_TIMESTAMPTZ".to_string());
    hashset.insert("DATEADD".to_string());
    hashset.insert("DATEDIFF".to_string());
    hashset.insert("DATE_PART".to_string());
    hashset.insert("DATE_PART_YEAR".to_string());
    hashset.insert("DATE_TRUNC".to_string());
    hashset.insert("EXTRACT".to_string());
    hashset.insert("GETDATE".to_string());
    hashset.insert("INTERVAL_CMP".to_string());
    hashset.insert("LAST_DAY".to_string());
    hashset.insert("MONTHS_BETWEEN".to_string());
    hashset.insert("NEXT_DAY".to_string());
    hashset.insert("SYSDATE".to_string());
    hashset.insert("TIMEOFDAY".to_string());
    hashset.insert("TIMESTAMP_CMP".to_string());
    hashset.insert("TIMESTAMP_CMP_DATE".to_string());
    hashset.insert("TIMESTAMP_CMP_TIMESTAMPTZ".to_string());
    hashset.insert("TIMESTAMPTZ_CMP".to_string());
    hashset.insert("TIMESTAMPTZ_CMP_DATE".to_string());
    hashset.insert("TIMESTAMPTZ_CMP_TIMESTAMP".to_string());
    hashset.insert("TIMEZONE".to_string());
    hashset.insert("TO_TIMESTAMP".to_string());
    hashset.insert("TRUNC".to_string());
    hashset.insert("CHECKSUM".to_string());
    hashset.insert("farmFingerprint64".to_string());
    hashset.insert("FUNC_SHA1".to_string());
    hashset.insert("FNV_HASH".to_string());
    hashset.insert("MD5".to_string());
    hashset.insert("SHA".to_string());
    hashset.insert("SHA1".to_string());
    hashset.insert("SHA2".to_string());
    hashset.insert("IS_VALID_JSON".to_string());
    hashset.insert("IS_VALID_JSON_ARRAY".to_string());
    hashset.insert("JSON_ARRAY_LENGTH".to_string());
    hashset.insert("JSON_EXTRACT_ARRAY_ELEMENT_TEXT".to_string());
    hashset.insert("JSON_EXTRACT_PATH_TEXT".to_string());
    hashset.insert("JSON_PARSE".to_string());
    hashset.insert("CAN_JSON_PARSE".to_string());
    hashset.insert("JSON_SERIALIZE".to_string());
    hashset.insert("JSON_SERIALIZE_TO_VARBYTE".to_string());
    hashset.insert("ABS".to_string());
    hashset.insert("ACOS".to_string());
    hashset.insert("ASIN".to_string());
    hashset.insert("ATAN".to_string());
    hashset.insert("ATAN2".to_string());
    hashset.insert("CBRT".to_string());
    hashset.insert("CEILING (or CEIL)".to_string());
    hashset.insert("COS".to_string());
    hashset.insert("COT".to_string());
    hashset.insert("DEGREES".to_string());
    hashset.insert("DEXP".to_string());
    hashset.insert("DLOG1".to_string());
    hashset.insert("DLOG10".to_string());
    hashset.insert("EXP".to_string());
    hashset.insert("FLOOR".to_string());
    hashset.insert("LN".to_string());
    hashset.insert("LOG".to_string());
    hashset.insert("MOD".to_string());
    hashset.insert("PI".to_string());
    hashset.insert("POWER".to_string());
    hashset.insert("RADIANS".to_string());
    hashset.insert("RANDOM".to_string());
    hashset.insert("ROUND".to_string());
    hashset.insert("SIN".to_string());
    hashset.insert("SIGN".to_string());
    hashset.insert("SQRT".to_string());
    hashset.insert("TAN".to_string());
    hashset.insert("TRUNC".to_string());
    hashset.insert("ASCII".to_string());
    hashset.insert("BPCHARCMP".to_string());
    hashset.insert("BTRIM".to_string());
    hashset.insert("BTTEXT_PATTERN_CMP".to_string());
    hashset.insert("CHAR_LENGTH".to_string());
    hashset.insert("CHARACTER_LENGTH".to_string());
    hashset.insert("CHARINDEX".to_string());
    hashset.insert("CHR".to_string());
    hashset.insert("COLLATE".to_string());
    hashset.insert("CONCAT".to_string());
    hashset.insert("CRC32".to_string());
    hashset.insert("DIFFERENCE".to_string());
    hashset.insert("INITCAP".to_string());
    hashset.insert("LEFT".to_string());
    hashset.insert("RIGHT".to_string());
    hashset.insert("LEN".to_string());
    hashset.insert("LENGTH".to_string());
    hashset.insert("LOWER".to_string());
    hashset.insert("LPAD".to_string());
    hashset.insert("RPAD".to_string());
    hashset.insert("LTRIM".to_string());
    hashset.insert("OCTETINDEX".to_string());
    hashset.insert("OCTET_LENGTH".to_string());
    hashset.insert("POSITION".to_string());
    hashset.insert("QUOTE_IDENT".to_string());
    hashset.insert("QUOTE_LITERAL".to_string());
    hashset.insert("REGEXP_COUNT".to_string());
    hashset.insert("REGEXP_INSTR".to_string());
    hashset.insert("REGEXP_REPLACE".to_string());
    hashset.insert("REGEXP_SUBSTR".to_string());
    hashset.insert("REPEAT".to_string());
    hashset.insert("REPLACE".to_string());
    hashset.insert("REPLICATE".to_string());
    hashset.insert("REVERSE".to_string());
    hashset.insert("RTRIM".to_string());
    hashset.insert("SOUNDEX".to_string());
    hashset.insert("SPLIT_PART".to_string());
    hashset.insert("STRPOS".to_string());
    hashset.insert("STRTOL".to_string());
    hashset.insert("SUBSTR".to_string());
    hashset.insert("SUBSTRING".to_string());
    hashset.insert("TEXTLEN".to_string());
    hashset.insert("TRANSLATE".to_string());
    hashset.insert("TRIM".to_string());
    hashset.insert("UPPER".to_string());
    hashset.insert("CAST".to_string());
    hashset.insert("CONVERT".to_string());
    hashset.insert("TO_CHAR".to_string());
    hashset.insert("TO_DATE".to_string());
    hashset.insert("TO_NUMBER".to_string());
    hashset.insert("TEXT_TO_INT_ALT".to_string());
    hashset.insert("TEXT_TO_NUMERIC_ALT".to_string());
    hashset
}
