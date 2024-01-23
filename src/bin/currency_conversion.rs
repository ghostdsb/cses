// Paramenters:

// array of currency conversion rates. E.g. ['USD', 'GBP', 0.77] which means 1 USD is equal to 0.77 GBP
// an array containing a 'from' currency and a 'to' currency
// Given the above parameters, find the conversion rate that maps to the 'from' currency to the 'to' currency.
// Your return value should be a number.

// Example:
// You are given the following parameters:

// Rates: ['USD', 'JPY', 110] ['US', 'AUD', 1.45] ['JPY', 'GBP', 0.0070]
// To/From currency ['GBP', 'AUD']
// Find the rate for the 'To/From' curency. In this case, the correct result is 1.89.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
// #[derive(Debug)]
struct Node<'a>{
    a: &'a str,
    b: &'a str,
    conv: f64,
}

impl<'a> Node<'a>{
    fn new(a: &'a str, b: &'a str, conv: f64) -> Self{
        Self { a, b, conv }
    }
}

// #[derive(Debug)]
struct Graph<'a>{
    conversions: BTreeMap<(&'a str, &'a str), f64>
}

impl<'a> Graph<'a>{
    fn new() -> Self{
        Self { conversions: BTreeMap::new() }
    }

    fn insert(&mut self, node: Node<'a>){
        self.conversions.insert((node.a, node.b), node.conv);
        self.conversions.insert((node.b, node.a), 1./node.conv);
    }

    fn get_connections(&self, currency: &'a str) -> Vec<(&str, f64)>{
        let mut neighbours:Vec<(&str, f64)> = vec![];
        for ((a, b), conv) in self.conversions.clone().into_iter(){
            if a==currency{
                neighbours.push((b, conv));
            }
        }
        neighbours
    }

    fn present(&self, currency: &'a str) -> bool{
        let mut found = false;
        for ((a, _b), _conv) in self.conversions.clone().into_iter(){
            if a==currency{
                found = true;
            }
        }
        found
    }
}


fn main() {
    let equations: Vec<Vec<String>> = vec![
        vec!["a".to_string(), "b".to_string()], 
        vec!["c".to_string(), "d".to_string()]
    ];
    let values: Vec<f64> = vec![
        1.0,
        1.0
    ];
    let queries: Vec<Vec<String>> = vec![
        vec!["a".to_string(), "c".to_string()], 
        vec!["b".to_string(), "d".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["d".to_string(), "c".to_string()],
    ];

    let x = calc_equation(equations, values, queries);
    dbg!(x);
}

pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut graph = Graph::new();

    for i in 0..equations.len(){
        let n = Node::new(&equations[i][0], &equations[i][1], values[i]);
        graph.insert(n);
    }

    let mut ans: Vec<f64> = vec![];

    for q in queries{
        let (start, end) = (&q[0], &q[1]);
        if !graph.present(start) || !graph.present(end){
            ans.push(-1.0);
            continue;
        }
        let mut conv: Vec<(&str, f64)> = vec![(start, 1.)];
        let mut recorded: BTreeSet<&str> = BTreeSet::new();
        let mut found = false;
        recorded.insert(start);
        while !conv.is_empty(){
            let (currency, rate) = conv.pop().unwrap();
    
            if currency == end{
                println!("{start} -> {end}: {}", rate) ;
                ans.push(rate);
                found = true;
                continue;
            }
    
            let neighbours = graph.get_connections(currency);
    
            for (c, f) in neighbours.iter(){
                if !recorded.contains(c){
                    conv.push((c, f*rate));
                    recorded.insert(c);
                }
            }
        }
        if !found{
            ans.push(-1.0);
        }
    }
    ans
}