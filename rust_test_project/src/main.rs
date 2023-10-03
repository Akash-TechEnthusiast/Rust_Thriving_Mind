use postgres::{ Client, Error, NoTls };
// use std::num::ParseIntError;
// mod actions; // this will be used in the next steps
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;
use postgres::types::Type;



#[macro_use]
extern crate serde_derive;


#[derive(Debug)]  #[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}



const DB_URL: &str = "postgres://postgres:KGM@123$:5432/postgres";

//cosntants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";


mod actions;

fn main() {



   /* if let Err(_) = set_database() {
        println!("Error setting database");
        return;
    } */


// below code workig fine but just commented now
  /*  match postgresConnection() {
        Ok(()) => {
            println!("Function executed successfully.");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            // Handle the error as needed
        }
    }
*/



     //start server and print port
     let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
     println!("Server listening on port 8080");
 
     for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                 handle_client(stream);
             }
             Err(e) => {
                 println!("Unable to connect: {}", e);
             }
         }
     }
    // Using the "?" operator that works like the unwrap() method
    // This will panic an error of type postgres and will make the function return, thats because we have to use that kind of Error here
    // If we were parsing something, the kind of error could use is ParseIntError


    //Ok(());
}

fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect("postgres://postgres:KGM@123$:5432/postgres", NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS next_gen_app.users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    "
    )?;
    Ok(())
}




fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(r),
                r if r.starts_with("GET /users/") => handle_get_request(r),
                r if r.starts_with("GET /users") => handle_get_all_request(r),
                r if r.starts_with("PUT /users/") => handle_put_request(r),
                r if r.starts_with("DELETE /users/") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}





fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body without id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}


/*fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO next_gen_app.json_data (name, location) VALUES ($1, $2)",
                    &[&user.name, &user.email]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
} */



 fn handle_post_request(request: &str) -> (String, String) {

    
// postgres://postgres:KGM@123$:5432/postgres
    
    match get_user_request_body(&request) {
        Ok(user) => {

            println!("{:?}",user);

            println!("field1: {}", user.name);
            println!("field2: {}", user.email);

          
            match insertdata(user) {
                Ok(()) => {
                    println!("Function executed successfully.");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    // Handle the error as needed
                }
            }

           
            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
} 

fn insertdata(user : User) -> Result<(), Error> {

 
    let mut _client = Client::connect(
        "host     = localhost
        port     = 5432
        user     = postgres
        password = KGM@123$
        dbname = postgres",
        NoTls
    );    

    println!("new field1: {}", user.name);
    println!("new field2: {}", user.email);

    println!("Helo world " );

     actions::create::InsertData(&mut _client,user.name,user.email);

   /* let statement = client12?.prepare_typed(
        "INSERT INTO next_gen_app.json_data (name, age) VALUES ($1, $2)",
        &[Type::VARCHAR, Type::INT4],
    )?;*/


   
    Ok(())

}


//handle get request
fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle get all request
fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle put request
fn handle_put_request(request: &str) -> (String, String) {
    match
        (
            get_id(&request).parse::<i32>(),
            get_user_request_body(&request),
            Client::connect(DB_URL, NoTls),
        )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle delete request
fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            //if rows affected is 0, user not found
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}


fn postgresConnection() -> Result<(), Error> {

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
    actions::create::Select(&mut _client);                                     
  //  actions::create::Insert(&mut _client,user,user_id);
  //  actions::create::Drop(&mut _client, dropuser,user_id);
  //  actions::create::Update(&mut _client, updateuser,user_id);
    
  // actions::create::CreateTable(&mut _client);
    

    println!("Helo world " );
    Ok(())

}

