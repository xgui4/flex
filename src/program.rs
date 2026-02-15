use crate::cmd_line::{self, get_string};

use crate::features;

use cmd_line::Args;

pub fn start_program(args: Vec<String>) {
    if args.len() > 1 {
        let arg: String = args[1].to_owned(); 
        if arg == get_string(Args::CmdVersionLong) || arg == get_string(Args::CmdVersion)  {
            features::display_app_version_info();
        }
        else if arg == get_string(Args::CmdLicense) || arg == get_string(Args::CmdLicenseLong) {
            features::display_license_info();
        }
        else if arg == get_string(Args::CmdAbout) || arg == get_string(Args::CmdAboutLong) {
            features::display_about_info();
        }
        else if arg == get_string(Args::CmdHelp) || arg == get_string(Args::CmdHelpLong) {
            features::print_help();
        }
        else if arg == get_string(Args::CmdColorCode) || arg == get_string(Args::CmdColorCodeLong) {
            features::print_colors_code_references();
        }
        else {
            features::output_image(&args[1]);
            features::keep_app_open_unil_key_pressed();
        }
    }
    else if args.len() == 1{
        features::choice_user();
        features::keep_app_open_unil_key_pressed();
    }
}