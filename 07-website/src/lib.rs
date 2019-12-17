#![recursion_limit = "128"]
#![allow(dead_code)]

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::model::EvalResult;
use location::UrlLocation;
use model::Expression;
use parser::ParseResult;
use stack::{Operation, Program};

mod location;
mod model;
mod parser;
mod stack;

pub struct Model {
    console: ConsoleService,
    text: String,
    ast: ParseResult<Expression>,
    location: UrlLocation,
    program: Program,
}

pub enum Msg {
    TextChanged(String),
    Compile,
    Step,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let location = UrlLocation::new();
        let text = location.query_param.clone();
        let ast = parser::parse(&text);
        let program = stack::compile(ast.as_ref().unwrap().clone());
        Model {
            console: ConsoleService::new(),
            text,
            ast,
            program,
            location,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TextChanged(text) => {
                self.text = text;
            }
            Msg::Compile => {
                self.ast = parser::parse(&self.text);
                self.location.update_route(self.text.clone());
                match self.ast.as_ref() {
                    Ok(expr) => {
                        self.program = stack::compile(expr.clone());
                    }
                    Err(e) => {
                        self.console.error(&format!("{:?}", e));
                    }
                }
            }
            Msg::Step => {
                self.program.step();
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div id="main">
                <textarea id="input"
                          rows=5
                          value=&self.text
                          oninput=|e| Msg::TextChanged(e.value)
                          placeholder="1 + 1">
                </textarea>

                <nav id="menu">
                  <button onclick=|_| Msg::Compile>{ "Compile" }</button>
                  <button onclick=|_| Msg::Step>{ "Step" }</button>
                </nav>

                {view_program(&self.program)}
            </div>
        }
    }
}

fn view_program(program: &Program) -> Html<Model> {
    html! {
      <div id="program">
        <div id="operations">
            <div id="pointer"></div>
            { for program.operations.iter().enumerate().map(|(idx, op)| view_op(op, idx as i32 - program.pointer as i32)) }
        </div>
        <ol id="stack">
            { for program.stack.iter().rev().map(view_stack_value) }
        </ol>
      </div>
    }
}

fn view_op(op: &Operation, idx: i32) -> Html<Model> {
    html! {
        <div class=("stack-operation", "tooltip")
             style=format!("transform: translate({}px)", 64*idx)>
           <div class="tooltip">
            { op.short() }
            <div class="tooltiptext">{ op.tooltip() }</div>
           </div>
        </div>
    }
}

fn view_stack_value(v: &EvalResult) -> Html<Model> {
    html! {
        <li class=("stack-value",
                   if v.is_ok() { "stack-value-ok" } else { "stack-value-err"})>
           { match v {
               Ok(v) => format!("{}", v),
               Err(e) => format!("{:?}", e),
           } }
        </li>
    }
}
