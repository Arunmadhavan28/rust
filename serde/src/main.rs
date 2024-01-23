use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize, Debug)]
struct Mystruct {
    name:String,
    age:u32,
}


fn main(){
let mystruct = Mystruct { name:"Arun".to_string() , age:23 };
let serialized = serde_json::to_string(&mystruct).unwrap();

//for serialisation
println!("the serialize output is : {}" , serialized );


//for deserialization
let deserialize:Mystruct = serde_json::from_str(&serialized).unwrap();
println!("the deserialized output is : {:?} " , deserialize);

}