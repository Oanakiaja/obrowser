// TODO:
// 1.cascade ， user agent sheets
// 2. display node except
// 3. initial and/or computed value
// 4. Inheritance
// 5. The dom ‘style’ Attributes

use crate::css::{Rule, Selector, SimpleSelector, Specificity, StyleSheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

pub type PropertyMap = HashMap<String, Value>;

pub type MatchedRule<'a> = (Specificity, &'a Rule);

pub enum Display {
    Inline,
    Block,
    None,
}

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

fn matches(element: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(element, simple_selector),
    }
}

fn matches_simple_selector(element: &ElementData, selector: &SimpleSelector) -> bool {
    if selector
        .tag_name
        .iter()
        .any(|name| element.tag_name != *name)
    {
        return false;
    }

    if selector.id.iter().any(|id| element.id() != Some(id)) {
        return false;
    }

    if selector
        .class
        .iter()
        .any(|class| !element.classes().contains(&**class))
    {
        return false;
    }

    return true;
}

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, style_sheet: &'a StyleSheet) -> Vec<MatchedRule<'a>> {
    style_sheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn specified_values<'a>(elem: &ElementData, style_sheet: &'a StyleSheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, style_sheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

pub fn style_tree<'a>(root: &'a Node, style_sheet: &'a StyleSheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, style_sheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, style_sheet))
            .collect(),
    }
}

impl StyledNode<'_> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }
}
