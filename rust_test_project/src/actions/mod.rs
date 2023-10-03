
pub mod create {

    use postgres::{ Client, Error, NoTls };
    use postgres::types::Type;

    #[derive(Debug)]
    struct User {
        username: String,
        age: i32
    }
    

    pub fn Select(client: &mut Client) -> Result<(), Error> {


        for row in client.query("SELECT name,age FROM next_gen_app.json_data", &[])? {
            let user = User {
                username: row.get(0),
                age: row.get(1) 
            };
            println!("Record: {:?}", &user);
        }
        Ok(())

    }

    pub fn Insert(client: &mut Client, username: String, age: i32) -> Result<(), Error> {

        let statement = client.prepare_typed(
            "INSERT INTO next_gen_app.json_data (name, age) VALUES ($1, $2)",
            &[Type::VARCHAR, Type::INT4],
        )?;
    
        let res = client.execute(
            &statement,
            &[&username, &age]
        )?;
    
        print!("Result while INSERT -> {}", &res);
        Ok(())
    }



pub fn InsertData(client: &mut Result<Client, Error>,user: String, location: String) -> Result<(), Error> {
      
    let client1 = client.as_mut().unwrap();
                let statement = client1.prepare_typed(
                    "INSERT INTO next_gen_app.json_data (name, location) VALUES ($1, $2)",
                    &[Type::VARCHAR, Type::VARCHAR],
                )?;
            
                let res = client1.execute(
                    &statement,
                    &[&user, &location]
                )?;
            
                print!("Result while INSERT -> {}", &res);
                Ok(())
          
           
        }
    
    
    pub fn Drop(client: &mut Client, username: String, age: i32) -> Result<(), Error> {
    
        let statement = client.prepare_typed(
            "DELETE FROM next_gen_app.json_data WHERE age = $1",
            &[Type::INT4],
        )?;
    
        let res = client.execute(
            &statement,
            &[&age]
        )?;
    
        print!("Result while INSERT -> {}", &res);
    
        Ok(())
    }

    pub fn Update(client: &mut Client, username: String, age: i32) -> Result<(), Error> {
    
        let statement = client.prepare_typed(
            "UPDATE next_gen_app.json_data SET name = 'Bearz' WHERE age = $1",
            &[Type::INT4],
        )?;
    
        let res = client.execute(
            &statement,
            &[&age]
        )?;
    
        print!("Result while INSERT -> {}", &res);
    
        Ok(())
    }


    pub fn CreateTable(client: &mut Client) -> Result<(), Error> {
    
        client.batch_execute(
            "
            CREATE TABLE IF NOT EXISTS next_gen_app.app_user (
                id              SERIAL PRIMARY KEY,
                username        VARCHAR UNIQUE NOT NULL,
                password        VARCHAR NOT NULL,
                email           VARCHAR UNIQUE NOT NULL
                )
        ",
        )?;

        client.execute(
            "INSERT INTO next_gen_app.app_user (username, password, email) VALUES ($1, $2, $3)",
            &[&"user7", &"mypass", &"user@test1.com"],
        )?;

     
    
    client.execute(
        "INSERT INTO next_gen_app.app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user8", &"mypass2", &"use2@gmail3.com"],
    )?;

    client.execute(
        "INSERT INTO next_gen_app.app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user9", &"anotherpass", &"mister3@test34.com"],
    )?;

    for row in client.query("SELECT id, username, password, email FROM next_gen_app.app_user", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);
    
        println!(
            "found app user: {}) {} | {} | {}",
            id, username, password, email
        );

    
     
    }

    client.execute(
        "UPDATE next_gen_app.app_user SET username=$1 WHERE id=$2",
        &[&"jack1", &11],
    )?;

    client.execute("DELETE FROM next_gen_app.app_user WHERE id=$1", &[&11])?;

    Ok(())
    }


  



   
}