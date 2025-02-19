
//############PROVISION INTERFACE####################
pub mod provision {

use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

#[derive(Serialize, Deserialize)]
pub struct coreDevice {
     schemas: Vec<String>,
     deviceDisplayName: String,
     adminState: bool,
     #[serde(rename = "urn:ietf:params:scim:schemas:extension:ble:2.0:Device")]
     ble_device: bleDevice,
     #[serde(rename = "urn:ietf:params:scim:schemas:extension:endpointAppsExt:2.0:Device")] 
     endpointAppsExt: endpoint_Apps_Ext,
     //TODO at device UUID
}

#[derive(Serialize, Deserialize)]
pub struct bleDevice {
  versionSupport: Vec<String>,
  deviceMacAddress: String,
  isRandom: bool,
  pairingMethods: Vec<String>,
  //TODO: Need Enum Here for Pairing methods
  #[serde(rename = "urn:ietf:params:scim:schemas:extension:pairingNull:2.0:Device")]
  //pair_just_works: pairingJustWorks
  ble_sec: ble_pairing_index
}

//BLE-Security-Enum
#[derive(Serialize, Deserialize)]
enum ble_pairing_index {
  pairingOOB(pairingOOB),
  pairingJustWorks(pairingJustWorks), 
  pairingNull(pairingNull), 
} 

pub struct ciscoEndpoint {
  url: String, 
  apiKey: String
}


#[derive(Serialize, Deserialize)]
struct pairingPassKey {
     key: String
}


#[derive(Serialize, Deserialize)]
#[serde(rename = "urn:ietf:params:scim:schemas:extension:pairingOOB:2.0:Device")]
struct pairingOOB { //TODO implement this
    key: String, 
    randomNumber: u32,
    confirmationNumber: Option<u32>
}

/*
 * Level 1 security
 */
#[derive(Serialize, Deserialize)]
#[serde(rename = "urn:ietf:params:scim:schemas:extension:pairingJustWorks:2.0:Device")]
struct pairingJustWorks {
  key: String
}

/*
 * No security pairing
 */
#[derive(Serialize, Deserialize)]
#[serde(rename = "urn:ietf:params:scim:schemas:extension:pairingNull:2.0:Device")]
struct pairingNull {
}


#[derive(Serialize, Deserialize, Debug)]
struct endpoint_Apps_Ext {
 onboardingUrl: String,
 deviceControlUrl: Vec<String>,
 dataReceiverUrl:   Vec<String>
}

//TODO -- Use configuration struct or variables to pull in endpoints 
 pub fn prov_dev_wrapper(control_url : &String, data_url: &String, onboard_url: &String) -> coreDevice {

  let my_device =  coreDevice {
    schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Device".to_owned(), 
                  "urn:ietf:params:scim:schemas:extension:ble:2.0:Device".to_owned(),
                  "urn:ietf:params:scim:schemas:extension:endpointapps:2.0:Device".to_owned()
                    ],
    deviceDisplayName: "Braeden's Ble HR Sensor".to_string(),
    adminState: true,
    ble_device : bleDevice {
      versionSupport : vec!["5.2".to_string()],
      deviceMacAddress : "D6:BA:05:D7:AC:87".to_string(),
      isRandom: false,
      pairingMethods : vec![ "urn:ietf:params:scim:schemas:extension:pairingNull:2.0:Device".to_string()],
      ble_sec: ble_pairing_index::pairingNull(pairingNull{}),
    },
        endpointAppsExt: endpoint_Apps_Ext  { 
        onboardingUrl: onboard_url.to_string(),
        deviceControlUrl: vec![control_url.to_string()],
        dataReceiverUrl: vec![data_url.to_string()]              
      }
  };
  
  let devreq = serde_json::to_string(&my_device).unwrap();
  println!("{}", &devreq);
my_device
}

/** Webrequest framework to provision a device to 
 *  Cisco Infrastructure
 */
pub async fn prov_device(core_dev: &coreDevice, url: &String, api_key: &String) -> Value {
   let client = reqwest::Client::builder()
  .danger_accept_invalid_certs(true)//Super sketch
  .no_proxy()
  .build()
  .unwrap()
  .post(url)
  .header(CONTENT_TYPE, "application/json")
  .header("x-api-key",api_key)
  .json(core_dev)
  .send()
  .await.expect("No response")
  .text()
  .await;
  let jsn_str = client.unwrap();
  let rqst_body: Value = serde_json::from_str(&jsn_str).unwrap();

  rqst_body
}

/**
 * NIPC interface to serialize all connection
 * topics to IOT-Controller or other endpoints
 */
pub mod connect {

use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

/*
   Disconnect messaging
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct device_disco {
 pub technology: String,
 pub id: String,
 pub controlApp: String
}

//service id connect message
#[derive(Serialize, Deserialize, Debug)]
pub struct service_id {
  pub serviceID: String
}

//BLE service connect message
#[derive(Serialize, Deserialize, Debug)]
pub struct ble {
  pub services: Vec<service_id>
}

//Device connection message
#[derive(Serialize, Deserialize, Debug)]
pub struct device_con {
  pub technology: String,
  pub id: String,
  pub ble: ble,
  pub controlApp: String
}


pub async fn conDevice(connect_message: &device_con ,connect_url: &String, cntrl_api_key: &String) {
  println!("Connect URL = {} ", connect_url);
  let client = reqwest::Client::builder()
   .danger_accept_invalid_certs(true)//Super sketch
   .no_proxy()
   .build()
   .unwrap()
   .post(connect_url)
   .header(CONTENT_TYPE, "application/json")
   .header("x-api-key",cntrl_api_key)
   .json(connect_message)
   .send()
   .await.expect("No response")
   .text()
   .await;
   dbg!(client.unwrap());
}

//Check this I don't think it works as is
pub async fn delDevice(url : &String, id : &String, apiKey: &String)
 {
  let mut delURL = url.clone();
  //del_url_bldr(url, id);
  let client = reqwest::Client::builder()
  .danger_accept_invalid_certs(true)
  .no_proxy()
  .build()
  .unwrap()
  .delete(delURL)
  .header("x-api-key",apiKey)
  .send()
  .await;
  dbg!(client.unwrap());
}

pub async fn discodev(disco_msg: &device_disco, disco_url: &String, cntrl_api_key: &String) {
   //let url = BASE_URL.to_owned() + DISCO_URL;
   let client = reqwest::Client::builder()
   .danger_accept_invalid_certs(true)//Super sketch
   .no_proxy()
   .build()
   .unwrap()
   .post(disco_url)
   .header(CONTENT_TYPE, "application/json")
   .header("x-api-key",cntrl_api_key)
   .json(disco_msg)
   .send()
   .await.expect("No response")
   .text()
   .await;
   dbg!(client.unwrap());
 }

/**
 * NIPC post will take a nipc msg enum serializ it and 
 */
 pub async fn dscvr_srvs(connect_message: &device_con,  url: &String, cntrl_api_key: &String)
 {
   let client = reqwest::Client::builder()
   .danger_accept_invalid_certs(true)//Super sketch
   .no_proxy()
   .build()
   .unwrap()
   .post(url)
   .header(CONTENT_TYPE, "application/json")
   .header("x-api-key",cntrl_api_key)
   .json(connect_message)
   .send()
   .await.expect("No response")
   .text()
   .await;
   dbg!(client.unwrap());
 }
}
//##############################Subscription Interface########################################

/* topic registration method */

#[derive(Serialize, Deserialize, Debug)]
pub struct regDataApp_msg {
 pub controlApp: String, 
 pub topic: String, 
 pub dataApps: Vec<dataAppID>  // Probably should be enum 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct dataAppID {
  pub dataAppID: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct regTopic_msg {
  pub technology: String, 
  pub ids: Vec<String>,
  pub controlApp: String,
  pub topic: String,
  pub ble: ble_sub
}

pub async fn regTopic(reg_msg: &regTopic_msg, cntrl_url: &String, cntrl_api_key: &String)
{
    let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)//Super sketch
    .no_proxy()
    .build()
    .unwrap()
    .post(cntrl_url)
    .header(CONTENT_TYPE, "application/json")
    .header("x-api-key",cntrl_api_key)
    .json(reg_msg)
    .send()
    .await.expect("No response")
    .text()
    .await;
    dbg!(client.unwrap());
}

//Work on this, There are multiple ble subs
//Maybe use an enum of sorts.
#[derive(Serialize, Deserialize, Debug)]
pub struct ble_sub {
 #[serde(rename = "type")]
  pub typeSub: String,
  pub serviceID: String, 
  pub characteristicID: String,
}

pub async fn regDataApp(reg_msg: &regDataApp_msg, regdata_url: &String, cntrl_api_key: &String)
{
    //let cnct_url = BASE_URL.to_owned() + "control/registration/registerDataApp";
    let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)//Super sketch
    .no_proxy()
    .build()
    .unwrap()
    .post(regdata_url)
    .header(CONTENT_TYPE, "application/json")
    .header("x-api-key",cntrl_api_key)
    .json(reg_msg)
    .send()
    .await.expect("No response")
    .text()
    .await;
    dbg!(client.unwrap());
     //println!("{:?}", client.unwrap());
}

enum scim_message {

}


 async fn prov_dev_topic()
 {

 }

 async fn topic_subscrb()
 {

 }
}



pub mod telemetry 
{ 

  use prost::Message;
  use rumqttc::{MqttOptions,Event, Incoming, Client, QoS};
  use std::time::Duration;
  use nipc::nipc::DataSubscription;
  use crate::iot_nipc::PhilipsEnterprise::*;
  
pub fn data_client()
  {
    let mut mqttoptions = MqttOptions::new("test-data-app", "10.228.97.60", 41883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_credentials("data-app-1", "3b04e597e53583f03b8b503763775cf5a0280131fde674f38ba313ca3bbfee1f");
    let (client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("enterprise/hospital/zephyr_hr_A2", QoS::AtMostOnce).unwrap();
    let mut heart_rate;
    println!("Starting Notifications");
    // Iterate to poll the eventloop for connection progress
  for  notification in connection.iter().flatten() {
    //Eventually Copy this to a msg queue.   //Check if the event queue is really a notification?
    if let Event::Incoming(Incoming::Publish(packet)) = notification {
        heart_rate = packet.payload.clone();
        //let hrt_strng = String::from_utf8(heart_rate).expect("Couldn't convert to String");
       // println!("Payload = {:?}", heart_rate);
        let my_dat = DataSubscription::decode(heart_rate).unwrap();
        let ibevitals = bld_ibe_data(my_dat);
        snd_ibe_msg(ibevitals);
        
      }
      
  }
    
  }

}


  pub mod PhilipsEnterprise {

    use nipc::nipc::DataSubscription;
    use reqwest::*;
    use serde::{Deserialize, Serialize};
    use reqwest::header::CONTENT_TYPE;
    use chrono;
  //Using a json structure
  
   #[derive(Serialize, Deserialize, Debug)] 
  pub struct entries {
    patientId : String,
    hospitalName : String,
    clinicalUnitName : String,
    deviceId : String,
    timeStamp : String,
    receiveTime: String, 
    vitals : Vitals,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Vitals {
   pub activity : u16,
   pub pulserate : u8,
   pub respirationrate : u16,
   pub posture : u8,
   pub posturelength : u16,
   pub skinTemperature : u16,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct IbePayloadMsg {
    total : u16,
    entries : Vec<entries>,
  }
 
 //https://users.rust-lang.org/t/from-bytes-to-u32-in-c-vs-rust/62713
  fn slice_to_num(buff: &[u8]) -> u8 {
    u8::from_ne_bytes(
        buff.try_into().unwrap())
}
 

 pub fn bld_ibe_data(data_sub: DataSubscription) -> IbePayloadMsg
  {
    let mut entrs:Vec<entries> = Vec::new();
    println!("Data buffer size = {:?}", data_sub.data.len());
    let data_vec = &data_sub.data[3..4];
    let plsrate = slice_to_num(data_vec);
    let vtls = Vitals {
      activity : 5,
      pulserate : plsrate,
      respirationrate : 15,
      posture : 1,
      posturelength : 100,
      skinTemperature : 37,
    };

    let newEntry = entries {
      patientId : "123456".to_string(),
      hospitalName : "My Hospital".to_string(),
      clinicalUnitName : "My Unit".to_string(),
      deviceId : data_sub.device_id.unwrap(),
      timeStamp : chrono::offset::Utc::now().to_string(),
      receiveTime : data_sub.timestamp.unwrap().to_string(),
      vitals : vtls,
    };
       entrs.push(newEntry);

    let ibepayld = IbePayloadMsg { total: 1, entries : entrs};
     dbg!(&ibepayld);
     ibepayld
  }

//Configuraiton here:  IBE server configuration.
 pub fn snd_ibe_msg (ibepayld: IbePayloadMsg)
 {
    let ibesrvr = "http://10.228.96.6:8998/vitals";
    let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)//Super sketch
      .no_proxy()
      .build()
      .unwrap()
      .post(ibesrvr)
      .header(CONTENT_TYPE, "application/json")
      .json(&ibepayld)
      .send()
      .expect("No Response from IBE")
      .text();
    dbg!(client.unwrap());
    
   // println!("{:?}", respnse);
 }
    //let cnct_url = BASE_URL.to_owned() + "control/registration/registerDataApp";
    
 
}
 
