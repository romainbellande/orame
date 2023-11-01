use std::collections::HashMap;

use leptos::{ReadSignal, SignalGet};

pub enum Classname {
    String(String),
    HahsMap(HashMap<String, ReadSignal<bool>>),
}

pub fn classnames(classes_raw: Vec<Classname>) -> String {
    let mut classes = vec![];

    for class_raw in classes_raw {
        match class_raw {
            Classname::String(class) => {
                classes.push(class);
            }
            Classname::HahsMap(class_map) => {
                for (class, condition) in class_map {
                    if condition.get() {
                        classes.push(class);
                    }
                }
            }
        }
    }

    classes.join(" ")
}
