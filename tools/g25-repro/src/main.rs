use scm_sdk::validation::{parse,Mode};
use serde_json::{Value,json};
fn main(){
 let cases:Value=serde_json::from_str(include_str!("../cases.json")).unwrap();
 for case in cases.as_array().unwrap(){
  let schema=format!("vendor:red-team-dp:0.93.0:{}",case["schema"].as_str().unwrap());
  let accepted=parse(&schema,case["value"].clone(),Mode::Response).is_ok();
  println!("{}",json!({"schema":case["schema"],"field":case["field"],"accepted":accepted}));
  assert_eq!(accepted,case["expectedSdk"].as_bool().unwrap(),"historical G25 reproduction changed");
 }
}
