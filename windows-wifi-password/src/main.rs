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
            WLAN_INTERFACE_INFO_LIST,
            WLAN_INTERFACE_INFO,
            WLAN_INTERFACE_STATE,
        
    },
},
};
fn main() {
    println!("Hello, world!");
}
