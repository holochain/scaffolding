use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Crud {
    // We don't include create and read because they must always exist
    pub update: bool,
    pub delete: bool,
}

pub fn parse_crud(crud_str: &str) -> Result<Crud, String> {
    if !crud_str.contains('c') {
        return Err(String::from("create ('c') must be present"));
    }
    if !crud_str.contains('r') {
        return Err(String::from("read ('r') must be present"));
    }

    let mut crud = Crud {
        update: false,
        delete: false,
    };

    for c in crud_str.chars() {
        match c {
            'c' | 'r' => {}
            'u' => {
                crud.update = true;
            }
            'd' => {
                crud.delete = true;
            }
            _ => {
                return Err(String::from(
                    "Only 'c', 'r', 'u' and 'd' are allowed in the crud argument",
                ));
            }
        }
    }

    Ok(crud)
}
