extern crate windows_exe_info;
use windows_exe_info::versioninfo::*;

fn main(){
    windows_exe_info::icon::icon_ico("assets/avatar.ico");
    VersionInfo {
        file_version: Version(0, 0, 0, 2),
        product_version: Version(0, 0, 0, 2),
        file_flag_mask: FileFlagMask::Win16,
        file_flags: FileFlags {
            debug: false,
            patched: false,
            prerelease: false,
            privatebuild: false,
            infoinferred: false,
            specialbuild: false,
        },
        file_os: FileOS::Windows32,
        file_type: FileType::App,
        file_info: vec![FileInfo {
            lang: Language::USEnglish,
            charset: CharacterSet::Multilingual,
            comment: None,
            company_name: "Xgui4 Studio".into(),
            file_description: "Show an ASCII Art File in a terminal".into(),
            file_version: "0.0.0.2".into(),
            internal_name: "flex".into(),
            legal_copyright: None,
            legal_trademarks: None,
            original_filename: "flex.exe".into(),
            product_name: "Flex".into(),
            product_version: "0.0.0.2".into(),
            private_build: None,
            special_build: None,
        }],
    }
    .link().unwrap();
}