

pub mod utilities {
  use serde_json::Value;
  /*
     Should this be a macro?
   */
  pub fn bld_base_url(base_url: &String, app_path: &String ) -> String {
    let srv_app_path = base_url.to_owned() + app_path;
    srv_app_path
  }
 
 pub fn del_url_bldr(mut url: &String, id: &String, onboard_url: &String) -> String {
   let query_url = "?onboardApp=".to_owned() + onboard_url;
   let  full_url = url.to_owned() + &id + &query_url.to_owned();
   //println!("URL = {}", full_url);
   full_url
 }

 pub fn get_request_id(v:&Value )-> String {
   let id = &v["id"];
   id.to_string()
 }

}