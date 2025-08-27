use regex::Regex;

fn mask_email(email: &str) -> String {
    let re = Regex::new(r"(?P<first>[^@]{1})[^@]*(@.*)").unwrap();
    let masked = re.replace(email, "${first}***${2}");
    masked.to_string()
}

fn main() {
    let email = "john.doe@example.com";
    println!("{}", mask_email(email)); 
}
