#[derive(Debug, Clone)]
pub struct Crud {
    // We don't include create because create must always exist
    pub read: bool,
    pub update: bool,
    pub delete: bool,
}

pub fn parse_crud(crud_str: &str) -> Result<Crud, String> {
    if !crud_str.contains('c') {
        return Err(String::from("create ('c') must be present"));
    }

    let mut crud = Crud {
        read: false,
        update: false,
        delete: false,
    };

    for c in crud_str.chars() {
        match c {
            'c' => {}
            'r' => {
                crud.read = true;
            }
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
