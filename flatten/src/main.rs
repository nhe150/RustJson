extern crate flatten_json;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use serde_json::Value;

use std::io::{self, Write};
use flatten_json::flatten;

error_chain! {
foreign_links {
        Json(::serde_json::Error);
        Io(::std::io::Error);
        Flatten(::flatten_json::Error);
    }
}

static INPUT: &str = r#"{
	"data" : {
		"name" : {
			"first_name" : "",
			"last_name" : ""
		},
		"info" : {
			"gender" : "Male | Female"
		}
	},
	"user" : {
		"email" : "",
		"password" : ""
	}
}"#;

#[derive(Serialize, Deserialize, Debug)]
struct InputObject {
    data: InputData,
    user: InputUser,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputData {
    name: InputName,
    info: InputInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputName {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputInfo {
    gender: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputUser {
    email: String,
    password: String
}

#[derive(Debug)]
struct User {
      first: String,
      last_name: String
}

#[derive(Debug)]
struct CreateUser { 
    email: String,
    pd: String,
    User: User,
    gender: String
}

impl From<InputObject> for CreateUser {
    fn from(i: InputObject) -> CreateUser {
        let user = User { first : i.data.name.first_name, last_name: i.data.name.last_name};
                    
        CreateUser {
            email: i.user.email,
            pd: i.user.password,
            User: user,
            gender: i.data.info.gender
        }
    }
}

fn main() {
    let input: InputObject = serde_json::from_str(INPUT).unwrap();
    let create_user = CreateUser::from(input);
    println!("{:?}", create_user);

    let mut input = String::new();

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n > 0 {
            let v: Value = match serde_json::from_str(&input) {
                Ok(value) => value,
                Err(e) => {
                    error!("{}", &input);
                    panic!("{}", e);
                }
            };
            process_line(&v).unwrap();
            String::clear(&mut input);
        } else {
            info!("Reached end of stdin...");
            break;
        }
    }
}

fn process_line(value: &Value) -> Result<()> {
    let mut flat_value: Value = json!({});
    flatten(value, &mut flat_value, None, true, None)?;
    io::stdout().write_all(serde_json::to_string(&flat_value)?.as_bytes())?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}
