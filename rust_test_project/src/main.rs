use mysql::prelude::*;
use mysql::{self, Opts};

fn main() -> Result<(), mysql::Error> {
    // Define database connection options
    let opts = Opts::from_url("mysql://root:KGM@123$@localhost:3306/biometric_data")?;

    // Establish a MySQL connection
    let pool = mysql::Pool::new(opts)?;

    // Execute a simple query
    let query = "SELECT serverip, port FROM server_details";
    
    // Get a mutable connection from the pool
    let mut conn = pool.get_conn()?;
    
    // Call prep_exec on the mutable connection
    let query_results = conn.prep_exec(query, ())?;

    // Process and print the query results
    for row_result in query_results {
        let row = row_result?;
        let person_id: String = row.get("serverip").unwrap();
        let person_name: String = row.get("port").unwrap();
        println!("Person ID: {}, Person Name: {}", person_id, person_name);
    }

     Ok(())
}
