use std::fmt;
use std::default::Default;

#[derive(PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(PartialEq, Eq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

#[derive(PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

#[derive(PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: Value
}

#[derive(PartialEq)]
pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String)
}

#[derive(PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

#[derive(PartialEq, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}



impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        return Stylesheet { rules }
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        return Stylesheet { rules: Vec::new() }
    }
}

impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();

        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\n\n");
            };

            rule_result.push_str(&format!("{:?}", rule));
        };

        write!(f, "{}", rule_result)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
        return Rule { selectors, declarations }
    }
}

impl Default for Rule {
    fn default() -> Self {
        return Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "    ";

        for sel in &self.selectors {
            if sel_result.len() > 0 {
                sel_result.push_str(", ");
            };

            sel_result.push_str(&format!("{:?}", sel));
        };

        for decl in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{:?}", decl));
            decl_result.push_str("\n");            
        };

        write!(f, "{} {{\n{}}}", sel_result, decl_result)
    }
}


impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Self {
        return Selector {
            simple,
            combinators,
        }
    }
}


impl Default for Selector {
    fn default() -> Self {
        return Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}


impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }

            result.push_str(&format!("{:?}", sel));
        };

        write!(f, "{}", result)
    }
}


impl SimpleSelector {
    pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> Self {
        return SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        return SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}


impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            Some(ref t) => result.push_str(t),
            None => {},
        };
        
        match self.id {
            Some(ref s) => {
                result.push('#');
                result.push_str(s);
            },
            None => {},
        };

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        };

        write!(f, "{}", result)
    }
}


impl Declaration {
    pub fn new(property: String, value: Value) -> Self {
        return Declaration {
            property,
            value
        }
    }
}


impl Default for Declaration {
    fn default() -> Self {
        return Declaration {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}


impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(ref o) => write!(f, "{:?}", o),
        }
    }
}


impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Color{ r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        return Color::new(1.0, 1.0, 1.0, 1.0)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}