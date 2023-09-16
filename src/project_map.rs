use crate::error::Error;

#[derive(Debug)]
pub struct Class {
    pub full_qualifier: String,
    pub name: String,
    pub namespace: String,
}

impl Class {
    pub fn from_full_qualifier(full_qualifier: String) -> Self {
        Class {
            full_qualifier: full_qualifier.clone(),
            name: full_qualifier.clone(),
            namespace: full_qualifier,
        }
    }
}

#[derive(Debug)]
pub enum Usage {
    Inherits,
    Uses,
}

#[derive(Debug)]
pub struct Relation {
    pub class_a: usize,
    pub class_b: usize,
    pub usage: Usage,
}

#[derive(Debug)]
pub struct ProjectMap {
    classes: Vec<Class>,
    relations: Vec<Relation>,
}

impl ProjectMap {
    pub fn new() -> Self {
        Self {
            classes: vec![],
            relations: vec![],
        }
    }

    pub fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }

    pub fn get_relations(&self) -> &Vec<Relation> {
        &self.relations
    }

    pub fn add_class(&mut self, class: Class) -> usize {
        self.classes.push(class);
        self.classes.iter().count() - 1
    }

    pub fn add_relation(&mut self, relation: Relation) -> Result<usize, Error> {
        let nb_of_classes = self.classes.iter().count();
        if relation.class_a >= nb_of_classes {
            return Err(Error::Message(
                "class_a is larger than the number of classes".to_string(),
            ));
        }
        if relation.class_b >= nb_of_classes {
            return Err(Error::Message(
                "class_b is larger than the number of classes".to_string(),
            ));
        }
        self.relations.push(relation);
        Ok(self.relations.iter().count() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let res = ProjectMap::new();
        assert!(res.get_classes().is_empty());
        assert!(res.get_relations().is_empty());
    }

    #[test]
    fn test_add_class() {
        let mut res = ProjectMap::new();
        assert_eq!(
            res.add_class(Class::from_full_qualifier("A::B".to_string())),
            0
        );
        assert_eq!(
            res.add_class(Class::from_full_qualifier("A::C".to_string())),
            1
        );
        assert_eq!(res.get_classes().iter().count(), 2);
    }

    #[test]
    fn test_add_relation() {
        let mut res = ProjectMap::new();
        res.add_class(Class::from_full_qualifier("A::B".to_string()));
        res.add_class(Class::from_full_qualifier("A::C".to_string()));
        assert!(res
            .add_relation(Relation {
                class_a: 0,
                class_b: 1,
                usage: Usage::Uses
            })
            .is_ok());
    }

    #[test]
    fn test_add_relation_invalid_class_a() {
        let mut res = ProjectMap::new();
        res.add_class(Class::from_full_qualifier("A::B".to_string()));
        assert!(res
            .add_relation(Relation {
                class_a: 1,
                class_b: 0,
                usage: Usage::Inherits
            })
            .is_err());
    }

    #[test]
    fn test_add_relation_invalid_class_b() {
        let mut res = ProjectMap::new();
        res.add_class(Class::from_full_qualifier("A::B".to_string()));
        assert!(res
            .add_relation(Relation {
                class_a: 0,
                class_b: 1,
                usage: Usage::Inherits
            })
            .is_err());
    }
}
