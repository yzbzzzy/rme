use std::collections::HashMap;
use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use serde_derive::{Deserialize, Serialize};
use std::str;

#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
pub struct dev_type{
    device_info:Vec<dev_info>
}
#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
pub struct dev_info{
    dev_name:String,
    data:HashMap<String,String>
}
pub fn get_network_info() ->HashMap<String,dev_type>{
    let mut data = HashMap::new();
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut network:Vec<dev_info> = Vec::new();
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
        let mut map = HashMap::new();
        map.insert(String::from("received"),data.received().to_string());
        map.insert(String::from("transmitted"),data.transmitted().to_string());
        let info = dev_info { dev_name: String::from(interface_name), data: map };
        network.push(info);
    }
    data.insert(String::from("network"),dev_type {device_info: network });
    return data;
}

pub  fn get_mem_info()->HashMap<String,dev_type>{
    let mut data = HashMap::new();
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut mem:Vec<dev_info> = Vec::new();

    let mut memory=HashMap::new();
    memory.insert("total_memory".to_string(),(sys.total_memory()>>10>>10).to_string());
    memory.insert("used_memory".to_string(),(sys.used_memory()>>10>>10).to_string());
    memory.insert("available_memory".to_string(),(sys.available_memory()/1024/1024).to_string());
    mem.push(dev_info{dev_name:"memory".to_string(),data:memory});

    let mut swap=HashMap::new();
    swap.insert("total_swap".to_string(),(sys.total_swap()>>10>>10).to_string());
    swap.insert("used_swap".to_string(),(sys.used_swap()>>10>>10).to_string());
    mem.push(dev_info{dev_name:"swap".to_string(),data:swap});

    data.insert("memory".to_string(),dev_type{device_info:mem});
    return  data;
}

pub fn get_disk_info()->HashMap<String,dev_type>{
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut data = HashMap::new();
    let mut _disk:Vec<dev_info> = Vec::new();
    for disk in sys.disks(){
        let mut map = HashMap::new();
        map.insert("file_system".to_string(),str::from_utf8(disk.file_system()).unwrap().to_string());
        map.insert("total_space".to_string(),(disk.total_space()>>10>>10>>10).to_string());
        map.insert("available_space".to_string(),(disk.available_space()>>10>>10>>10).to_string());
        map.insert("mount_point".to_string(),disk.mount_point().display().to_string());
        _disk.push( dev_info{dev_name:disk.name().to_str().unwrap().to_string(),data:map})
    }
    data.insert("disk".to_string(),dev_type{device_info:_disk});
    return data;
}