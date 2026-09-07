use lbug::{Connection, Database, Error, SystemConfig, Value};

fn main() -> Result<(), Error> {
    let db = Database::new("social_network.lbdb", SystemConfig::default())?;
    let conn = Connection::new(&db)?;

    let query =
        "MATCH (u1:User)-[f1:FOLLOWS]->(u2:User)-[f2:Follows]->(u3:User)-[f3:FOLLOWS]->(u4:User)
        WHERE u1.username = $usersrc
        AND u4.username = $userdst
        AND (u2.username = $userint OR u3.username = $userint)
        RETURN COUNT(u4)";
    let mut prepared_query = conn.prepare(query)?;
    let usersrc = "epicwolf202";
    let userdst = "stormcat597";
    let userint = "stormfox762";
    let params = vec![
        ("usersrc", Value::String(usersrc.to_string())),
        ("userdst", Value::String(userdst.to_string())),
        ("userint", Value::String(userint.to_string())),
    ];
    let result7 = conn.execute(&mut prepared_query, params)?;
    println!("{}", result7);

    Ok(())
}
