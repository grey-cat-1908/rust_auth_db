use rusqlite::{Connection, Result};
use std::io;

#[derive(Debug)]
struct AuthInfo {
    login: String,
    password: String
}

fn main() -> Result<()> {
    let conn = Connection::open("auth.db")?;

    conn.execute(
        "create table if not exists users (
             login text primary key unique,
             password text not null
         )",
        [],
    )?;

    println!("[INFO] Здравствуйте! Если Вы хотите войти, то отправьте первой строкой свой логин, а второй свой пароль!");

    let mut login: String = "".to_string();
    let mut password: String = "".to_string();

    io::stdin().read_line(&mut login).unwrap().to_string();
    io::stdin().read_line(&mut password).unwrap().to_string();

    let mut stmt = conn.prepare("SELECT login, password FROM users WHERE login = :login AND password = :password")?;
    let checked_data = stmt.query_map(&[(":login", &login), (":password", &password)], |row| {
        Ok(AuthInfo {
            login: row.get(0)?,
            password: row.get(1)?,
        })
    })?;

    let mut check_result = Vec::new();
    for name_result in checked_data {
        check_result.push(name_result?);
    }

    if check_result.len() >= (1 as usize) {
        println!("Вы успешно авторизованы!");

        println!("Вы хотите зарегистрировать новую учетную запись / удалить текущую / изменить пароль?");

        let mut user_choice: String = "".to_string();

        io::stdin().read_line(&mut user_choice).unwrap().to_string();

        if user_choice.replace("\n", "").to_string().to_lowercase() == "зарегистрировать" {
            println!("Отправьте следующими двумя строки новый логин и пароль!");

            let mut new_user_login: String = "".to_string();
            let mut new_user_password: String = "".to_string();

            io::stdin().read_line(&mut new_user_login).unwrap().to_string();
            io::stdin().read_line(&mut new_user_password).unwrap().to_string();

            conn.execute(
                "INSERT INTO users (login, password) VALUES (:login, :password)",
                &[(":login", &new_user_login), (":password", &new_user_password)],
            )?;
        } else if user_choice.replace("\n", "").to_string().to_lowercase() == "изменить" {
            println!("Отправьте следующей строкой свой новый пароль!");

            let mut new_password: String = "".to_string();

            io::stdin().read_line(&mut new_password).unwrap().to_string().replace("\n", "").to_string();

            conn.execute(
                "UPDATE users SET password = :password WHERE login = :login",
                &[(":login", &login), (":password", &new_password)],
            )?;
        } else {
                conn.execute(
                    "DELETE FROM users WHERE login = :login AND password = :password",
                    &[(":login", &login), (":password", &password)],
                )?;
            }

    }
    else {
        println!("Неправильно введенные данные!")
    }

    Ok(())
}
