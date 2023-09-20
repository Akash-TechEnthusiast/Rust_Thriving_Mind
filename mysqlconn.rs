use mysql::{Opts, Conn};

fn main() {
    let opts = Opts::new()
        .user("root")
        .pass("KGM@123$")
        .db_name("biometric_data")
        .host("localhost")
        .port(3306);

    let conn = Conn::connect(opts).unwrap();

    // Do something with the database connection
    let results = conn.query_iter("SELECT * FROM biometric_attendancedata").unwrap();

for row in results {
    println!("User ID: {}", row[0].as_str().unwrap());
}

}


