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


fn main() {
    println!("Hello, world!");
}
