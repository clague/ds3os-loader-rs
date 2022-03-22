use std::process::Command;
use anyhow::{Result, anyhow};
use sysinfo::{ProcessExt, System, SystemExt, ProcessRefreshKind, Pid, PidExt};
use lazy_static::lazy_static;
use bytes::{Bytes, BytesMut, BufMut};
use process_memory::{Memory, Pid as PidHandle, TryIntoProcessHandle, DataMember, CopyAddress};

use crate::encrypt::tea32_encrypt;
use crate::api::{Server, MasterServerApi};

lazy_static! {
    pub static ref ServerInfoTEAEncryptionKey: [u32;4] = [
        0x4B694CD6,
        0x96ADA235,
        0xEC91D9D4,
        0x23F562E5
    ];

    pub static ref ServerInfoPatchSize: usize = 520;

    // Maximum length of the UTF8 encoded public key. 
    pub static ref ServerInfoMaxKeySize: usize= 430;

    // Maximum length of the UTF16 encoded hostname.
    pub static ref ServerInfoMaxHostSize: usize = 85; // Leave at least 2 bytes from the end of ServerInfoPatchSize for nullptr.

    // Offset into the data block that the hostname is placed.
    pub static ref ServerInfoHostOffset: usize = 432;

    pub static ref ServerInfoAddress: usize = 0x144F4A5B1;
}

pub struct Launcher {
    sys: System,
}

impl Launcher {
    pub fn new() -> Self {
        Launcher {
            sys: System::new(),
        }
    }
    pub fn run_game() -> Result<()> {
        Command::new("steam")
            .arg("steam://run/374320")
            .spawn()?;
        Ok(())
    }
    pub async fn patch_game(pid: u32, api: MasterServerApi, mut server: Server, password: &str) -> Result<()> {
        if server.pubkey.is_empty() {
            server.pubkey = api.get_pubkey(&server.ip_addr, password).await?;
        }
        Self::patch(pid, &server.hostname, &server.pubkey).map(|_| ())
    }
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

    fn patch(pid: u32, hostname: &str, pubkey: &str) -> Result<usize> {
        let handle = (pid as i32 as PidHandle).try_into_process_handle()?;
        let mut member = DataMember::new_offset(handle, vec![*ServerInfoAddress]);

        let data_block = Self::encrypt(hostname, pubkey)?;
        let data_len = data_block.len();
        let mut writed_len = 0;

        data_block.into_iter().for_each(|byte| {
            match member.write(&byte) {
                Ok(_) => {
                    writed_len += 1;
                    member.set_offset(vec![*ServerInfoAddress + writed_len]);
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

        if key_data.len() > *ServerInfoMaxKeySize {
            return Err(anyhow!("Key's size is too big!"))
        }  

        if host_data.len() > *ServerInfoMaxHostSize {
            return Err(anyhow!("Host's size is too big!"))
        }

        let mut data_block: BytesMut = BytesMut::with_capacity(*ServerInfoPatchSize);
        data_block.put_slice(key_data);
        data_block.put_bytes(0, *ServerInfoHostOffset - key_data.len());
        data_block.put_slice(host_data);

        data_block.resize(*ServerInfoPatchSize, 0);
        
        Ok(tea32_encrypt(&data_block, &*ServerInfoTEAEncryptionKey))
    }
}