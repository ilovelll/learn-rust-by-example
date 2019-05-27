use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1221" => {
            "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again."
        }
        "241-5962" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
  username: &'a str,
  password: &'a str,
}

struct AccountInfo<'a> {
  name: &'a str,
  email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
  println!("Username: {}", username);
  println!("Password: {}", password);
  println!("Attempting logon...");

  let logon = Account {
    username, password
  };

  match accounts.get(&logon) {
    Some(account_info) => {
      println!("Successful logon!");
      println!("Name: {}", account_info.name);
      println!("Email: {}", account_info.email);
    },
    _ => println!("Login failed"),
  }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1221");
    contacts.insert("Ashley", "241-5962");
    contacts.insert("Katie", "031-9502");
    contacts.insert("Robert", "512-7721");

    match contacts.get("Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Dainel's number"),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get("Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove("Ashley");

    for (contact, number) in contacts.iter() {
      println!("Calling {}: {}", contact, call(number));
    }

    let mut accounts: Accounts = HashMap::new();

    let account = Account {
      username: "j.everyman",
      password: "password123",
    };

    let account_info = AccountInfo {
      name: "John Everyman",
      email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);
    try_logon(&accounts, "j.everyman", "passff");
    try_logon(&accounts, "j.everyman", "password123");
}
