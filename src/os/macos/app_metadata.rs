use std::path::PathBuf;

use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSString;

use super::app::App;
use crate::util::capitalize;

pub fn resolve(bundle_id: &str) -> App {
    let app_path = resolve_app_path(bundle_id);
    let (name, icon_path) = app_path
        .as_deref()
        .and_then(read_plist)
        .unwrap_or((heuristic_name(bundle_id), None));

    App::new(bundle_id.to_string(), name, icon_path, app_path)
}

fn resolve_app_path(bundle_id: &str) -> Option<String> {
    let ns_id = NSString::from_str(bundle_id);
    let url = NSWorkspace::sharedWorkspace().URLForApplicationWithBundleIdentifier(&ns_id)?;
    Some(url.path()?.to_string())
}

fn heuristic_name(bundle_id: &str) -> String {
    let name = bundle_id.split('.').next_back().unwrap_or(bundle_id);
    capitalize(name)
}

fn read_plist(app_path: &str) -> Option<(String, Option<PathBuf>)> {
    let plist_path = format!("{app_path}/Contents/Info.plist");
    let file = std::fs::File::open(&plist_path).ok()?;
    let plist: plist::Value = plist::from_reader(file).ok()?;
    let dict = plist.as_dictionary()?;
    let name = extract_name(dict)?;
    let icon_path = extract_icon_path(dict, app_path);
    Some((name, icon_path))
}

fn extract_name(dict: &plist::Dictionary) -> Option<String> {
    let value = dict
        .get("CFBundleDisplayName")
        .or_else(|| dict.get("CFBundleName"))?;
    Some(value.as_string()?.to_string())
}

fn extract_icon_path(dict: &plist::Dictionary, app_path: &str) -> Option<PathBuf> {
    let icon_file = dict.get("CFBundleIconFile")?.as_string()?;
    let icon_file = if icon_file.ends_with(".icns") {
        icon_file.to_string()
    } else {
        format!("{icon_file}.icns")
    };
    let path = PathBuf::from(format!("{app_path}/Contents/Resources/{icon_file}"));
    path.exists().then_some(path)
}
