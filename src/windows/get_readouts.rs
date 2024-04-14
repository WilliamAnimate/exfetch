// TODO: implement
use winreg::enums::*;
use winreg::RegKey;

use exfetch::get_env_var;

pub async fn obtain() -> crate::data::Data<'static> {
    let header_thread = spawn(async {
        let usr = get_env_var!("USERNAME");

        let mut result = String::from("\x1B[0;31m\x1B[1mex\x1B[0;36mFetch\x1B[0m - ");

        result.push_str(&usr);
        result.push('\n');

        result
    });

    let distro_thread = spawn(async {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ).unwrap();
        let mut version: String = subkey.get_value("ProductName").unwrap();
        let current_build: String = subkey.get_value("CurrentBuild").unwrap();
        let display_version: String = subkey.get_value("DisplayVersion").unwrap();

        // remove pro/enterprise/home/etc from the version
        version = version.replace(" Pro", "").replace(" Home", "").replace(" Enterprise", "");

        format!("{}, {} (build {})", version, display_version, current_build)
    });

    todo!("windows support is currently **unimplemented!**");
}
