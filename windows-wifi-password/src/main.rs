//Simple project to extract wifi password from windows
use windows ::{
    core:: {GUID, HSTRING,PCWSTR, PCWSTR},
    Win32::{
        Foundation::{ERROR_SUCCESS, HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR},
        NetworkManagement::WiFi{
            WlanOpenHandle,
            WlanCloseHandle,
            WlanEnumInterfaces,
            WlanFreeMemory,
            WlanGetProfile,
            WlanGetProfileList,
            WLAN_API_VERSION_2_0,
            WLAN_INTERFACE_INFO,
            WLAN_PROFILE_GET_PLAINTEXT_KEY,
            WLAN_PROFILE_INFO_LIST,       
    },
},
};

//Functions
fn open_wlan_handle(api version :u32) -> Result<HANDLE, WIN32_ERROR>{
    let mut handle :HANDLE = INVALID_HANDLE_VALUE;
    let mut result = unsafe{
        WlanOpenHandle(api, None, &mut handle)
    }
    WIN32_ERROR(result).ok_or(result)?;
    Ok(handle)
}
fn main() {
    println!("Hello, world!");
}
