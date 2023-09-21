
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

}