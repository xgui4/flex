pub enum Args {
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

pub fn get_string(arg : Args) -> String {
    return match arg {
        Args::CmdVersion => "--v".to_owned(),
        Args::CmdVersionLong => "--version".to_owned(),
        Args::CmdAbout => "-a".to_owned(),
        Args::CmdAboutLong => "--about".to_owned(),
        Args::CmdLicense => "-l".to_owned(),
        Args::CmdLicenseLong => "--license".to_owned(),
        Args::CmdHelp => "-h".to_owned(),
        Args::CmdHelpLong => "--help".to_owned(),
        Args::CmdColorCode => "-c".to_owned(),
        Args::CmdColorCodeLong => "--color-code".to_owned()
    }
}