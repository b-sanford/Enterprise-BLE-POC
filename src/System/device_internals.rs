mod device_internals {

/*
   Internals, Device mapping between Physical device and 
   ASDF, NIPC abstractions of the device
 */
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
}  


