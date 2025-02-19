use std::io;
use std::thread;
use std::collections::HashMap;
use crate::nipc_utils::utilities::*;
use crate::iot_nipc::provision::*;
use crate::connect::*;
use crate::iot_nipc::telemetry::*;
pub mod iot_nipc;
pub mod nipc_utils;

//For now just set up constants of the API_KEYS until a better 
//implementation comes along
const CONTROL_API_KEY: &str = "b0f4d0e5e157a107a2bf0bb45cce3106576af6cd4506d7f3b608d41eacca2a56";
const ONBOARD_API_KEY: &str = "2920ae4475d893cdea35dcf2fa97db96831d4db019a0672a59a3e2fa04d40478";
const DATA_API_KEY: &str = "3b04e597e53583f03b8b503763775cf5a0280131fde674f38ba313ca3bbfee1f";

//APP URLS -- 
const CONTROL_URL: &str = "https://ciscotest.com/controlapp/1";
const ONBOARD_URL: &str = "https://ciscotest.com/onboardapp/1";
const DATA_URL: &str = "data-app-1";

//Constrol URLs 
const BASE_URL: &str = "https://10.228.97.60:8081/";
const SKIM_V2: &str =  "scim/v2/Devices/";
const DISCO_URL: &str = "control/connectivity/disconnect/";
const CNCT_URL: &str = "control/connectivity/connect/";
const DLT_URL: &str = "";
const DSCVR_URL: &str = "control/data/discover/";
const REG_TOPIC_URL: &str = "control/registration/registerTopic";

//-------------------------------Start Device Mapping --------------------------------------------

//need a provisioned device, ID, patient ID?
fn init_map() -> HashMap<String, coreDevice> {
  let device_hash: HashMap<String, coreDevice> = HashMap::new();  
   device_hash
}

fn add_device(){}

fn find_device(device_id: String, device_map: &HashMap<String, coreDevice>) -> Option<&coreDevice>
{
  let core_device = device_map.get(&device_id);
  core_device
}

fn dlt_device(deviceID: String){}

fn dlt_all_devices(device_map: &HashMap<String, coreDevice>){
}

//-------------------------------End Device Mapping ----------------------------------------------

//TODO: Move CLI Arg parser into module
#[tokio::main]
async fn main() -> Result<(),reqwest::Error> {
      let mut argBuffer = String::new();
      let mut ble_device_map = init_map();

      loop {

       io::stdin().read_line(&mut argBuffer).expect("Failed to recieve arguments");
       let mut args = argBuffer.split(" ");
       let cmd = args.nth(0).unwrap().trim();
     
       println!("Recieved {}", cmd);

      match cmd {
        "provision" => {
                         let mydev = prov_dev_wrapper(&CONTROL_URL.to_string(),
                          &DATA_URL.to_string(),&ONBOARD_URL.to_string());
                         let prv_url = bld_base_url(&BASE_URL.to_string(), &SKIM_V2.to_string());
                         let id= prov_device(&mydev, &prv_url, &ONBOARD_API_KEY.to_string()).await;
                         let idstr = get_request_id(&id);
                         ble_device_map.insert(idstr, mydev);
                         println!("We have {} devices in map", ble_device_map.len());
        }
         "connect" => { 
                    let idstr = args.nth(0).unwrap().to_string();  
                     let myserviceID = service_id {
                        serviceID: args.nth(0).unwrap().to_string(),
                     };
                     let myble = ble {
                           services: vec![myserviceID]
                     };
                     let mycon = device_con {
                      technology : "ble".to_owned(),
                      id : idstr.to_string(),
                      ble: myble,
                      controlApp:  CONTROL_URL.to_owned()
                     };
                     let con_url = bld_base_url(&BASE_URL.to_string(), &CNCT_URL.to_string());
                     println!("{}", serde_json::to_string(&mycon).unwrap());
                     conDevice(&mycon, &con_url, &CONTROL_API_KEY.to_string()).await;
       },
        "delete" => {
                      //let delurl =  
                      let idstr = args.nth(0).unwrap().trim().to_string();
                      let url = bld_base_url(&BASE_URL.to_string(), &SKIM_V2.to_string());
                      let delurl= del_url_bldr(&url.to_string(), &idstr, &ONBOARD_URL.to_string());
                      delDevice(&delurl.to_string(), &idstr.to_string(),&CONTROL_API_KEY.to_string()).await;
                      ble_device_map.remove(&idstr);
        },
       "disconnect" => {  
            let idstr = args.nth(0).unwrap().trim().to_string();
            let mydisco = device_disco {
              technology: "ble".to_owned(),
              id: idstr,
              controlApp: CONTROL_URL.to_owned()
            };
            let disco_url = bld_base_url(&BASE_URL.to_string(),&DISCO_URL.to_string());
            discodev(&mydisco, &disco_url ,&CONTROL_API_KEY.to_string()).await
        },

        "discover" => { 
          println!("setting up discover message");
          let idstr = args.nth(0).unwrap().trim();  
          let myserviceID = service_id {
             serviceID: args.nth(0).unwrap().trim().to_string(),
          };
          let myble = ble {
                services: vec![myserviceID]
          };
          let mycon = device_con {
           technology : "ble".to_owned(),
           id : idstr.to_string(),
           ble: myble,
           controlApp: CONTROL_URL.to_owned()
          };
          let dcvr_url = bld_base_url(&BASE_URL.to_string(), &DSCVR_URL.to_string());
         dscvr_srvs(&mycon, &dcvr_url,&CONTROL_API_KEY.to_string()).await;

        },

        "read" => {

        },

        "regTopic" => {
          let ids = args.nth(0).unwrap().trim().to_string();

          let bleMsg = ble_sub {
               typeSub: "gatt".to_string(),
               serviceID: "180d".to_string(),
               characteristicID: "2a37".to_string(),
          };

          let topicMsg =  regTopic_msg {
           technology: "ble".to_string(),
           ids: vec![ids],
           topic: "enterprise/hospital/zephyr_hr_A3".to_string(),
           controlApp: CONTROL_URL.to_string(),
           ble: bleMsg,
          };
          
         let topic_url = bld_base_url(&BASE_URL.to_string(), &REG_TOPIC_URL.to_string());
          regTopic(&topicMsg, &topic_url, &CONTROL_API_KEY.to_string()).await;
        
        },

        "regdatapp" => {
          let mut posts:Vec<dataAppID> = Vec::new();
          let dataAppID = dataAppID {
            dataAppID: "https://data-app-2".to_string(),
           
        };
          posts.push(dataAppID);
          let topic = args.nth(0).unwrap().trim().to_string();
          let reg_data = regDataApp_msg {
           controlApp : CONTROL_URL.to_string(),
           topic: topic,
           dataApps: posts,
          };
          let json = serde_json::to_string(&reg_data).unwrap();
          dbg!(json);
          //regDataApp(&reg_data).await;
        },

        "subscribe" => {
          let idstr = args.nth(0).unwrap().to_string();
          println!("Recieved Subscribe Topic");
        },

        "unsubscribe" => {
          let idstr = args.nth(0).unwrap().to_string();     
          println!("Recieved Unsubscribe");
        },

        "data-client" => { //automatically connects to subscrption interface
          println!("Spawning Thread");
          thread::spawn(|| {data_client();}).join().expect("Thread Failed");
        },
        
        "done" => break,
        _=> println!("Error not a good command "),
            
      }
      argBuffer.clear();
    }
    Ok(())
}