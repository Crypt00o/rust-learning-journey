#![allow(warnings)]

enum Animal {
    Cat,
    Dog,
    Snake,
    Ferris,
}

impl Animal {
    fn is_equal_to(&self, other: &Self) -> bool {
        match self {
            &Self::Cat => match other {
                &Self::Cat => return true,
                _ => return false,
            },
            &Self::Dog => match other {
                &Self::Dog => return true,
                _ => return false,
            },
            &Self::Snake => match other {
                &Self::Snake => return true,
                _ => return false,
            },
            &Self::Ferris => match other {
                &Self::Ferris => return true,
                _ => return false,
            },
            _ => return false,
        }
    }
}

enum Person {
    NormalPerson {
        name: String,
        age: u8,
    },
    Developer {
        name: String,
        age: u8,
        language: String,
    },
    Hacker {
        name: String,
        age: u8,
        language: String,
        skills: String,
    },
}

impl Person {
    fn info(&self) {
        match self {
            Self::Hacker {
                name,
                language,
                skills,
                age,
            } => {
                println!("\n Type Hacker : \nname : {name} , age : {age}, fav languages : {language} ,skills : {skills} ")
            }
            Self::NormalPerson { name, age } => {
                println!("\n Type NormalPerson : \nname : {name},age : {age}")
            }
            Self::Developer {
                name,
                age,
                language,
            } => {
                println!(
                    "\n Type Developer : \nname : {name}, age : {age} , languages  : {language} "
                )
            }
            _ => {
                println!("default")
            }
        }
    }

    fn init_hacker(name: &str, age: u8, language: &str, skills: &str) -> Self {
        return Self::Hacker {
            name: String::from(name),
            age,
            language: String::from(language),
            skills: String::from(skills),
        };
    }

    fn init_normal_person(name: &str, age: u8) -> Self {
        return Self::NormalPerson {
            name: String::from(name),
            age,
        };
    }

    fn init_developer(name: &str, age: u8, language: &str) -> Self {
        return Self::Developer {
            name: String::from(name),
            age,
            language: String::from(language),
        };
    }

    fn clone(&self) -> Self {
        match self {
            Self::Hacker {
                name,
                age,
                language,
                skills,
            } => {
                return Self::Hacker {
                    name: name.clone(),
                    age: *age,
                    language: language.clone(),
                    skills: skills.clone(),
                }
            }
            Self::Developer {
                name,
                age,
                language,
            } => {
                return Self::Developer {
                    name: name.clone(),
                    age: *age,
                    language: language.clone(),
                };
            }
            Self::NormalPerson { name, age } => {
                return Self::NormalPerson {
                    name: name.clone(),
                    age: *age,
                };
            }
        }
    }

    fn clear(&mut self) {
        match self {
            Self::Hacker {
                name,
                age,
                language,
                skills,
            } => {
                name.clear();
                *age = 0;
                language.clear();
                skills.clear()
            }
            Self::NormalPerson { name, age } => {
                name.clear();
                *age = 0
            }
            Self::Developer {
                name,
                age,
                language,
            } => {
                name.clear();
                language.clear();
                *age = 0
            }
        }
    }
}

fn main() {
    let mut me: Person = Person::init_hacker(
        "0xCrypt00o",
        21,
        "assembly 0x86 , assembly 0x86_64 , C , Python3 , Js  ",
        "Network , Os Internals(Unix,NT) Exploitation with manual written assembly shellcodes ",
    );
    me.info();
    me.clear();
    me = Person::init_developer(
        "eslam mohamed",
        21,
        "Js , Skills(NodeJs , ExpressJs , MySQl , Postgresql , MongoDB) ",
    );
    me.info();
    me.clear();
    me = Person::init_normal_person("eslam", 21);
    me.info();
    me.clear();

    let cat: Animal = Animal::Cat;
    let dog: Animal = Animal::Dog;
    let cat2: Animal = Animal::Cat;
    println!("\nis cat2 equal to cat ?  {} ", cat2.is_equal_to(&cat));
    println!("\nis cat  equal to dog ?  {} ", cat.is_equal_to(&dog));


    //if let equal
        if let Animal::Cat=cat{
           println!("equal to cat useing if let equal"); 

        }


}
