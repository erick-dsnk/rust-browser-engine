use crate::css::{
    Color,
    Declaration,
    Rule,
    Selector,
    SimpleSelector,
    Stylesheet,
    Unit,
    Value
};
use std::iter::Peekable;
use std::str::Chars;


pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>
}


impl<'a> CssParser<'a> {
    pub fn new(full_css: &'a str) -> Self {
        return CssParser{
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut sheet = Stylesheet::default();

        while self.chars.peek().is_some() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            sheet.rules.push(rule);
        };

        return sheet
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let sel = self.parse_selector();

            if sel != Selector::default() {
                selectors.push(sel);
            }

            self.consume_while(char::is_whitespace);

            if self.chars.peek().map_or(false, |c| *c == ',') {
                self.chars.next();
            }
        };

        self.chars.next();

        return selectors;
    }


    fn parse_selector(&mut self) -> Selector {
        let mut simple_sel = SimpleSelector::default();
        let mut sel = Selector::default();

        self.consume_while(char::is_whitespace);

        simple_sel.tag_name = match self.chars.peek() {
            Some(&c) if is_valid_start_indent(c) => Some(self.parse_identifier()),
            _ => None,
        };

        let multiple_ids = false;

        while self.chars
            .peek()
            .map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace())
        {
            match self.chars.peek() {
                Some(&c) if c == '#' => {
                    self.chars.next();

                    if simple_sel.id.is_some() || multiple_ids {
                        simple_sel.id = None;

                        multiple_ids = true;

                        self.parse_id();
                    } else {
                        simple_sel.id = self.parse_id();
                    }
                }

                Some(&c) if c == '.' => {
                    self.chars.next();

                    let class_name = self.parse_identifier();

                    if class_name != String::from("") {
                        simple_sel.classes.push(class_name);
                    }
                }

                _ => { self.consume_while(|c| c != ',' && c != '{'); }
            }
        };

        if simple_sel != SimpleSelector::default() {
            sel.simple.push(simple_sel)
        };

        return sel
    }

    fn parse_identifier(&mut self) -> String {
        let mut indent = String::new();

        match self.chars.peek() {
            Some(&c) => if is_valid_start_indent(c) {
                indent.push_str(&self.consume_while(is_valid_indent))
            },
            None => {}
        }

        return indent.to_lowercase()
    }

    fn parse_id(&mut self) -> Option<String> {
        return match &self.parse_identifier()[..] {
            "" => None,
            s @ _ => Some(s.to_string()),
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(char::is_whitespace);

            let property = self.consume_while(|x| x != ':').to_lowercase();

            self.chars.next();

            self.consume_while(char::is_whitespace);

            let value = self.consume_while(|x| x != ';' && x != '\n' && x != '}').to_lowercase();

            let value_enum = match property.as_ref() {
                "background-color" | "border-color" | "color" => {
                    Value::Color(translate_color(&value))
                },
                "margin-right" |
                "margin-bottom" |
                "margin-left" |
                "margin-top" |
                "padding-right" |
                "padding-bottom" |
                "padding-left" |
                "padding-top" |
                "border-right-width" |
                "border-bottom-width" |
                "border-left-width" |
                "border-top-width" |
                "height" |
                "width" => translate_length(&value),
                _ => Value::Other(value)
            };

            let declaration = Declaration::new(property, value_enum);

            if self.chars.peek().map_or(false, |c| *c == ';') {
                declarations.push(declaration);
                self.chars.next();
            } else {
                self.consume_while(char::is_whitespace);

                if self.chars.peek().map_or(false, |c| *c == '}') {
                    declarations.push(declaration);
                }
            };

            self.consume_while(char::is_whitespace);
        }

        self.chars.next();

        return declarations
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        };
        
        return result
    }
}

fn translate_length(value: &str) -> Value {
    let mut num_string = String::new();
    let mut unit = String::new();
    let mut parsing_num = true;

    for c in value.chars() {
        if c.is_numeric() && parsing_num {
            num_string.push(c);
        } else {
            unit.push(c);
            parsing_num = false;
        }
    };

    let number = num_string.parse().unwrap_or(0.0);

    return match unit.as_ref() {
        "em" => Value::Length(number, Unit::Em),
        "ex" => Value::Length(number, Unit::Ex),
        "ch" => Value::Length(number, Unit::Ch),
        "rem" => Value::Length(number, Unit::Rem),
        "vh" => Value::Length(number, Unit::Vh),
        "vw" => Value::Length(number, Unit::Vw),
        "vmin" => Value::Length(number, Unit::Vmin),
        "vmax" => Value::Length(number, Unit::Vmax),
        "px" => Value::Length(number, Unit::Px),
        "mm" => Value::Length(number, Unit::Mm),
        "q" => Value::Length(number, Unit::Q),
        "cm" => Value::Length(number, Unit::Cm),
        "in" => Value::Length(number, Unit::In),
        "pt" => Value::Length(number, Unit::Pt),
        "pc" => Value::Length(number, Unit::Pc),
        "%" => Value::Length(number, Unit::Pct),
        _ => Value::Length(number, Unit::Px),
    }
}