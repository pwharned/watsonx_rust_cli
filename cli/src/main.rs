use api_macro::GetAttributesMacro;
use std::env;
mod deployment;

fn main() {
    let d = deployment::Asset {
        id: String::from("test"),
    };

    let p = deployment::Parameters {
        serving_name: String::from("test"),
    };
    let o = deployment::Online { parameters: p };
    let v = deployment::Online::get_attributes(&o);
    let args: Vec<String> = env::args().collect();

    for i in v {
        let j: String = String::from(i);
        if !args.contains(&j) {
            println!("{}", j)
        }
    }
}
