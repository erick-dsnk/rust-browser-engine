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
            let styles = self.parse_styles();
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
                    }
                }
            }
        };
    }
}