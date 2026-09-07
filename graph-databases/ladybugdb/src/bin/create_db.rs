use lbug::{Connection, Database, Error, SystemConfig};

fn main() -> Result<(), Error> {
    let db = Database::new("social_network.lbdb", SystemConfig::default())?;
    let conn = Connection::new(&db)?;

    conn.query(
        "CREATE NODE TABLE User(
            user_id INT64 PRIMARY KEY,
            username STRING,
            account_creation_date DATE
        )",
    )?;
    conn.query(
        "CREATE NODE TABLE Post(
            post_id INT64 PRIMARY KEY,
            post_date DATE,
            like_count INT64,
            retweet_count INT64
        )",
    )?;
    conn.query(
        "CREATE REL TABLE FOLLOWS(
            FROM User TO User
        )",
    )?;
    conn.query(
        "CREATE REL TABLE POSTS(
            FROM User TO Post
        )",
    )?;
    conn.query(
        "CREATE REL TABLE LIKES(
            FROM User TO Post
        )",
    )?;

    conn.query("COPY User FROM './tutorial_data/node/user.csv'")?;
    conn.query("COPY Post FROM './tutorial_data/node/post.csv'")?;
    conn.query("COPY FOLLOWS FROM './tutorial_data/relation/FOLLOWS.csv'")?;
    conn.query("COPY POSTS FROM './tutorial_data/relation/POSTS.csv'")?;
    conn.query("COPY LIKES FROM './tutorial_data/relation/LIKES.csv'")?;

    let result = conn.query("CALL SHOW_TABLES() RETURN *")?;
    println!("{}", result);

    Ok(())
}
