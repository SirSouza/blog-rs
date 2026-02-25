use crate::models::{NewPost, Post};
use rusqlite::{Connection, Result};

pub fn get_connection() -> Result<Connection> {
    Connection::open("blog.db")
}

pub fn initilize_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
            CREATE TABLE IF NOT EXISTS posts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            content     TEXT NOT NULL,
            created_at  TEXT NOT NULL DEFAULT (datetime('now')) 
            );
        ",
    )?;
    Ok(())
}

pub fn get_all_posts(conn: &Connection) -> Result<Vec<Post>> {
    let mut stmt =
        conn.prepare("SELECT id, title, content, created_at FROM posts ORDER BY id DESC")?;

    let posts = stmt
        .query_map([], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?
        .filter_map(|p| p.ok())
        .collect();

    Ok(posts)
}

pub fn get_post_by_id(conn: &Connection, id: i64) -> Result<Post> {
    conn.query_row(
        "SELECT id, title, content, created_at FROM posts WHERE id = ?1",
        [id],
        |row: &rusqlite::Row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        },
    )
}

pub fn create_post(conn: &Connection, new_post: &NewPost) -> Result<()> {
    conn.execute(
        "INSERT INTO posts (title, content) VALUES (?1, ?2)",
        (&new_post.title, &new_post.content),
    )?;
    Ok(())
}
