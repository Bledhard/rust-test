use super::ExitCode;
use collections::directory::dir_struct::Directory;

pub fn add_user(user: String, department: String, directory: &mut Directory) -> (ExitCode, String) {
    let dept_record = directory.departments.entry(department).or_insert(Vec::new());

    (*dept_record).push(user);

    (ExitCode::Continue, String::from(""))
}