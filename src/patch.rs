//use std::process::Command;
use anyhow::{Result, anyhow};
use sysinfo::{ProcessExt, System, SystemExt, ProcessRefreshKind, PidExt};
use lazy_static::lazy_static;
use bytes::{Bytes, BytesMut, BufMut};
use process_memory::{Memory, Pid as PidHandle, TryIntoProcessHandle, DataMember};

use crate::encrypt::tea32_encrypt;


lazy_static! {
    #[allow(non_upper_case_globals)]
    pub static ref SERVER_INFO_TEAENCRYPTION_KEY: [u32;4] = [
        0x4B694CD6,
        0x96ADA235,
        0xEC91D9D4,
        0x23F562E5
    ];
    pub static ref SERVER_INFO_PATCH_SIZE: usize = 520;

    // Maximum length of the UTF8 encoded public key. 
    pub static ref SERVER_INFO_MAX_KEY_SIZE: usize= 430;

    // Maximum length of the UTF16 encoded hostname.
    pub static ref SERVER_INFO_MAX_HOST_SIZE: usize = 85; // Leave at least 2 bytes from the end of ServerInfoPatchSize for nullptr.

    // Offset into the data block that the hostname is placed.
    pub static ref SERVER_INFO_HOST_OFFSET: usize = 432;

    pub static ref SERVER_INFO_ADDRESS: usize = 0x144F4A5B1;
}

pub struct Patches {
    sys: System,
}

impl Patches {
    pub fn new() -> Self {
        Patches {
            sys: System::new(),
        }
    }
    // pub fn run_game() -> Result<()> {
    //     Command::new("steam")
    //         .arg("steam://run/374320")
    //         .spawn()?;
    //     Ok(())
    // }

    pub fn find_process(&mut self) -> Result<u32> {
        self.sys.refresh_processes_specifics(ProcessRefreshKind::new());
        let mut res: u32 = 0;

        // It seems that process name in linux is "DarkSoulsIII.ex", so keep the last "e" out
        for process in self.sys.processes_by_name("DarkSoulsIII.ex") {
            let pid: u32 = process.pid().as_u32();
            if pid > res {
                res = pid;
            }
        }
        println!("Game's pid: {}", res);
        if res == 0 { Err(anyhow!("Can't find process")) } else { Ok(res) }
    }

    pub fn patch(pid: u32, hostname: &str, pubkey: &str) -> Result<usize> {
        let handle = (pid as i32 as PidHandle).try_into_process_handle()?;
        let mut member = DataMember::new_offset(handle, vec![*SERVER_INFO_ADDRESS]);

        let data_block = Self::encrypt(hostname, pubkey)?;
        let data_len = data_block.len();
        let mut writed_len = 0;

        data_block.into_iter().for_each(|byte| {
            match member.write(&byte) {
                Ok(_) => {
                    writed_len += 1;
                    member.set_offset(vec![*SERVER_INFO_ADDRESS + writed_len]);
                },
                Err(_) => ()
            }
        });
        if data_len != writed_len {
            Err(anyhow!("Exception happened during the patch!"))
        }
        else {
            Ok(writed_len)
        }
    }
    fn encrypt(hostname: &str, pubkey: &str) -> Result<Bytes> {
        let host_data: &[u8] = &hostname.encode_utf16().flat_map(|twin| {twin.to_le_bytes()} ).collect::<Vec<u8>>();
        let key_data = pubkey.as_bytes();

        if key_data.len() > *SERVER_INFO_MAX_KEY_SIZE {
            return Err(anyhow!("Key's size is too big!"))
        }  

        if host_data.len() > *SERVER_INFO_MAX_HOST_SIZE {
            return Err(anyhow!("Host's size is too big!"))
        }

        let mut data_block: BytesMut = BytesMut::with_capacity(*SERVER_INFO_PATCH_SIZE);
        data_block.put_slice(key_data);
        data_block.put_bytes(0, *SERVER_INFO_HOST_OFFSET - key_data.len());
        data_block.put_slice(host_data);

        data_block.resize(*SERVER_INFO_PATCH_SIZE, 0);
        
        Ok(tea32_encrypt(&data_block, &*SERVER_INFO_TEAENCRYPTION_KEY))
    }
}