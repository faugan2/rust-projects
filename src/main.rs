extern crate serde_json;

use serde_json::Value as JsonValue;
fn main() {
    let json_str=r#"
        {
            "name":"faugan",
            "age":35,
            "is_male":true,
            "hobbies":[1,2,3]
        }
    "#;

    let res:JsonValue=serde_json::from_str(json_str).unwrap();
    let obj=res.as_object().unwrap();
    let hobbies=obj.get("hobbies").unwrap();
    for (key,value) in obj.iter(){
        println!("{}={}",key,value);
    }
   
}
