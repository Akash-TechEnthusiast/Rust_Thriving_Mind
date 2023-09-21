use postgres::{ Client, Error, NoTls };
// use std::num::ParseIntError;
// mod actions; // this will be used in the next steps

fn main() -> Result<(), Error> {
    let mut _client = Client::connect(
        "host     = localhost
        port     = 5432
        user     = postgres
        password = KGM@123$
        
        dbname = postgres",

        NoTls
    )?;

  
    println!("Helo world " );
        for row in _client.query("SELECT id,name FROM next_gen_app.json_data", &[])? {
            let id: int4 = row.get(0);
            let username: &str = row.get(1);
      
        
            println!("found app user: {} ,{}", id,username );
        }
    

    // Using the "?" operator that works like the unwrap() method
    // This will panic an error of type postgres and will make the function return, thats because we have to use that kind of Error here
    // If we were parsing something, the kind of error could use is ParseIntError


    Ok(())
}
