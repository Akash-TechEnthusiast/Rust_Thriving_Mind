use postgres::{ Client, Error, NoTls };
// use std::num::ParseIntError;
// mod actions; // this will be used in the next steps

mod actions;

fn main() -> Result<(), Error> {
    let mut _client = Client::connect(
        "host     = localhost
        port     = 5432
        user     = postgres
        password = KGM@123$
        dbname = postgres",
        NoTls
    )?;

    let user = String::from("Jess");
    let user_id: i32 = 11; 
    let dropuser = user.clone(); // Clone the value
    let updateuser = user.clone(); // Clone the value
   // working just commeted below code 
  //  actions::create::Select(&mut _client);                                     
  //  actions::create::Insert(&mut _client,user,user_id);
  //  actions::create::Drop(&mut _client, dropuser,user_id);
  //  actions::create::Update(&mut _client, updateuser,user_id);
    
  actions::create::CreateTable(&mut _client);
    

    println!("Helo world " );
       

    // Using the "?" operator that works like the unwrap() method
    // This will panic an error of type postgres and will make the function return, thats because we have to use that kind of Error here
    // If we were parsing something, the kind of error could use is ParseIntError


    Ok(())
}
