use std::io::Read;

#[tokio::main]
async fn main() {
    /*
    let response = reqwest::get("https://q-bit.dots.org.ua").await;
    if response.is_err() {
        panic!("Response is not OK");
    }
    let ok_response = response.unwrap();
    println!("RESPONSE: {:?}", ok_response);

    let text: Result<String, _> = ok_response.text().await;
    if text.is_err() {
        panic!("Ошибка получения текстового ответа от сервера");
    }
    let ok_text: String = text.unwrap();
    println!("TEXT: {}", ok_text);
    //let text = response.text().await.unwrap();
    //println!("HTML: {}", &resp[100..200]);*/

    /*let ok_text = reqwest::get("https://q-bit.dots.org.ua")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("TEXT: {}", ok_text);*/

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    /*
    let ok_text = client
        .get("https://q-bit.dots.org.ua")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("TEXT: {}", ok_text);
    */

    // from=
    // username=test
    // password=test
    // token=bNqUGu0z4hT0PlCD
    let login_params = [
        ("from", ""),
        ("username", "test"),
        ("password", "test"),
        ("token", "bNqUGu0z4hT0PlCD"),
    ];

    let login_response = client
        .post("http://q-bit.dots.org.ua/login")
        .form(&login_params)
        .send()
        .await
        .unwrap();
    println!("Login response is {}", login_response.status());

    /*
    let ok_text = client
        .get("https://q-bit.dots.org.ua/contests")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("TEXT: {}", ok_text);
    */

    let contest_id = "603";

    let contest_login_response = client
        .get(&format!(
            "https://q-bit.dots.org.ua/contests?login={}",
            contest_id
        ))
        .send()
        .await
        .unwrap();
    println!(
        "Contest #{} Login response is {}",
        contest_id,
        contest_login_response.status()
    );

    let problem_id = "1001";
    let lang_id = "12";

    //let source_code = "print((0.3 * 5**2 - 15) / (3.5 + 2*2))";
    let mut source_code = String::new();
    std::io::stdin().read_to_string(&mut source_code).unwrap();

    let solution_params = [
        ("new", "2".to_string()),
        ("MAX_FILE_SIZE", "2097152".to_string()),
        ("pid", problem_id.to_string()),
        ("lang", lang_id.to_string()),
        ("ctype", "F".to_string()),
        ("source", source_code),
    ];

    let solution_response = client
        .post("http://q-bit.dots.org.ua/solutions")
        .form(&solution_params)
        .send()
        .await
        .unwrap();
    println!("Solution response is {}", solution_response.status());
}
