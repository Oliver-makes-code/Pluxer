use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD_INDIFFERENT};

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandRoot, builder::CommandBuilder, get_argument_single,
            node::unix::UnixParameter,
        },
        commands::{member::MemberCommand, system::SystemCommand},
    },
    database::{DatabaseExtension, DatabaseUpdate},
};

pub mod member;
pub mod system;

pub const NAME: &str = "name";
pub const NAME_VARIANTS: &[&str] = &["name", "n"];
pub const DISPLAY_NAME: &str = "display_name";
pub const DISPLAY_NAME_VARIANTS: &[&str] = &["display_name", "displayname", "dn"];
pub const PRONOUNS: &str = "pronouns";
pub const PRONOUNS_VARIANTS: &[&str] = &["pronouns", "pn"];
pub const TAG: &str = "tag";
pub const TAG_VARIANTS: &[&str] = &["tag", "t"];
pub const AVATAR_URL: &str = "avatar_url";
pub const AVATAR_URL_VARIANTS: &[&str] = &["avatar_url", "avatar", "a"];
pub const DESCRIPTION: &str = "description";
pub const DESCRIPTION_VARIANTS: &[&str] = &["description", "desc"];
pub const COLOR: &str = "color";
pub const PROXY: &str = "proxy";
pub const COLOR_VARIANTS: &[&str] = &["color", "col", "c"];
pub const CREATE_VARIANTS: &[&str] = &["new", "n", "create", "make", "add"];
pub const CLEAR_VARIANTS: &[&str] = &["clear", "remove", "unset"];
pub const UPDATE_VARIANTS: &[&str] = &["update", "set", "u"];
pub const DELETE_VARIANTS: &[&str] = &["delete"];
const YES: &str = "yes";
const YES_UNIX: &[UnixParameter] = &[UnixParameter::flag(YES, &["yes", "y"])];

pub fn parse_color_rgb(s: &str) -> Option<u32> {
    let color = csscolorparser::parse(s).ok()?;

    let [r, g, b, _] = color.to_rgba8();
    return Some((u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b));
}

fn u32_to_base64(n: u32) -> String {
    let bytes = n.to_be_bytes();
    return URL_SAFE_NO_PAD_INDIFFERENT.encode(bytes);
}

fn base64_to_u32(s: &str) -> Option<u32> {
    let bytes = URL_SAFE_NO_PAD_INDIFFERENT.decode(s).ok()?;

    if bytes.len() > 4 || bytes.is_empty() {
        return None;
    }

    let mut buf = [0u8; 4];

    let start_idx = 4 - bytes.len();
    buf[start_idx..].copy_from_slice(&bytes);

    let arr: [u8; 4] = bytes.try_into().ok()?;
    return Some(u32::from_be_bytes(arr));
}

fn extract_arg<'a>(
    args: &'a CommandArguments<'a>,
    arg_name: &'static str,
    current_arg: Option<&str>,
    clear: bool,
) -> DatabaseUpdate<Option<&'a str>> {
    if clear && current_arg.is_some_and(|it| it == arg_name) {
        return DatabaseUpdate::Set(None);
    }

    let Some(value) = get_argument_single(args, arg_name) else {
        return DatabaseUpdate::Keep;
    };

    if clear {
        return DatabaseUpdate::Set(None);
    }

    return DatabaseUpdate::Set(Some(value));
}

pub fn create_command_tree<'a, A: DatabaseExtension>() -> CommandRoot<PluxerContext<A>> {
    return CommandBuilder::<PluxerContext<A>>::build(|command| {
        SystemCommand::append(command);
        MemberCommand::append(command);
    });
}
