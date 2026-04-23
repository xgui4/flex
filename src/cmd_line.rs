pub enum CmdArgs {
    CmdVersion,
    CmdVersionLong,
    CmdLicense,
    CmdLicenseLong,
    CmdAbout,
    CmdAboutLong,
    CmdHelp,
    CmdHelpLong,
    CmdColorCode,
    CmdColorCodeLong,
}

impl CmdArgs {
    pub fn get_string(&self) -> String {
        match self {
            CmdArgs::CmdVersion => "--v".to_owned(),
            CmdArgs::CmdVersionLong => "--version".to_owned(),
            CmdArgs::CmdAbout => "-a".to_owned(),
            CmdArgs::CmdAboutLong => "--about".to_owned(),
            CmdArgs::CmdLicense => "-l".to_owned(),
            CmdArgs::CmdLicenseLong => "--license".to_owned(),
            CmdArgs::CmdHelp => "-h".to_owned(),
            CmdArgs::CmdHelpLong => "--help".to_owned(),
            CmdArgs::CmdColorCode => "-c".to_owned(),
            CmdArgs::CmdColorCodeLong => "--color-code".to_owned()
        }
    }
}