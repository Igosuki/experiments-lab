use lbug::{Connection, Database, Error, SystemConfig};

fn main() -> Result<(), Error> {
    let db = Database::new("social_network.lbdb", SystemConfig::default())?;
    let conn = Connection::new(&db)?;

    // Max folloers
    let result2 = conn.query(
        "MATCH (u1:User)-[f:FOLLOWS]->(u2:User)
        RETURN u2.username, COUNT(u2) AS follower_count
        ORDER BY follower_count DESC
        LIMIT 1",
    )?;
    println!("{}", result2);

    // Shortest path
    let result4 = conn.query(
        "MATCH p = (u1:User)-[f:FOLLOWS* SHORTEST]->(u2:User)
        WHERE u1.username = 'silentguy245'
        AND u2.username = 'epicwolf202'
        RETURN p",
    )?;
    println!("{}", result4);

    // How many paths between two nodes
    let result6 = conn.query(
        "MATCH (u1:User)-[f1:FOLLOWS]->(u2:User)-[f2:Follows]->(u3:User)-[f3:FOLLOWS]->(u4:User)
        RETURN COUNT(u4)",
    )?;
    println!("{}", result6);

    Ok(())
}
