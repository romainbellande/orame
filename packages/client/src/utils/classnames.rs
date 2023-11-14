pub enum Classname {
    String(String),
}

pub fn classnames(classes_raw: Vec<Classname>) -> String {
    let mut classes = vec![];

    for class_raw in classes_raw {
        match class_raw {
            Classname::String(class) => {
                classes.push(class);
            }
        }
    }

    classes.join(" ")
}
