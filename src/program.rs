use crate::cmd_line;

use crate::features;

use cmd_line::CmdArgs;

pub fn start_program(args: Vec<String>) {
    if args.len() > 1 {
        let arg: String = args[1].to_owned(); 
        if arg == CmdArgs::CmdVersionLong.get_string() || arg == CmdArgs::CmdVersion.get_string()  {
            features::display_version_str();
        }
        else if arg == CmdArgs::CmdLicense.get_string() || arg == CmdArgs::CmdLicenseLong.get_string() {
            features::display_license_info();
        }
        else if arg == CmdArgs::CmdAbout.get_string() || arg == CmdArgs::CmdAboutLong.get_string() {
            features::display_about_info();
        }
        else if arg == CmdArgs::CmdHelp.get_string() || arg == CmdArgs::CmdHelpLong.get_string() {
            features::print_help();
        }
        else if arg == CmdArgs::CmdColorCode.get_string() || arg == CmdArgs::CmdColorCodeLong.get_string() {
            features::print_colors_code_refs();
        }
        else {
            features::output_image(&args[1]);
            features::pause_app();
        }
    }
    else if args.len() == 1{
        features::choice_user();
        features::pause_app();
    }
}