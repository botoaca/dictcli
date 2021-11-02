use curl::easy::Easy;

fn draw_box(s: String) {
    let mut len: usize = 0;

    for line in s.lines() {
        len = len.max(line.len());
    }

    for _i in 0..=len + 3 { print!("-"); }

    for line in s.lines() {
        print!("\n| ");
        print!("{}", line);
        if line.len() < len {
            for _i in 0..len - line.len() { print!(" "); }
        }
        print!(" |");
    }
    print!("\n");

    for _i in 0..=len + 3 { print!("-"); }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // forcefully exit if there aren't enough arguments
    if args.len() < 3 {
        draw_box(String::from("Usage: rs_dictcli <language_code> <word>"));
        std::process::exit(1);
    }
    
    let mut easy = Easy::new();
    let mut dst = Vec::new();

    // get request to the url and store the response in dst
    let url = String::from("https://api.dictionaryapi.dev/api/v2/entries/".to_string() + &args[1] + "/" + &args[2]);
    easy.url(&url).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    
    // convert the response to serde_json::Value
    let j: serde_json::Value = serde_json::from_slice(&dst).unwrap();

    /*
        the title field only exists if the word is not found, so we can use
        this to exit if the word is not found and forcefully exit
    */
    if j["title"] != serde_json::Value::Null {
        draw_box(String::from("Word not found!"));
        std::process::exit(1);
    }

    // building the string from the json
    let mut str = String::new();

    str.push_str(&format!("Word: {}\n", j[0]["word"]));
    if j[0]["origin"] != serde_json::Value::Null {
        str.push_str(&format!("\nOrigin: {}\n", j[0]["origin"]));
    }
    str.push_str("\nMeanings:");
    for i in 0..j[0]["meanings"].as_array().unwrap().len() {
        str.push_str(
            &format!(
                "\n    as {}:\n        definition: {}\n        example: {}",
                j[0]["meanings"][i]["partOfSpeech"],
                j[0]["meanings"][i]["definitions"][0]["definition"],
                {
                    if j[0]["meanings"][i]["definitions"][0]["example"] != serde_json::Value::Null {
                        j[0]["meanings"][i]["definitions"][0]["example"].to_string()
                    } else {
                        "-".to_string()
                    }
                }
            )
        );
    }

    /*
        replacing special characters used for quotation marks to normal ones
        to avoid unexpected behaviour
    */
    str = str.replace(&['’', '‘'][..], "\'");

    draw_box(str);
}
