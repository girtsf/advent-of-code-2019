use std::collections::HashMap;
use std::env::args;
use std::fs;

#[derive(Debug, Clone)]
struct Chemical {
    name: String,
    qty: u64,
}

#[derive(Debug)]
struct Reaction {
    produced: Chemical,
    requires: Vec<Chemical>,
}

#[derive(Debug)]
struct Reactions {
    reactions: HashMap<String, Reaction>,
    to_make: HashMap<String, Chemical>,
    usage: HashMap<String, u64>,
}

impl Chemical {
    // input: "2 LGNW"
    fn parse(text: &str) -> Chemical {
        let parts: Vec<&str> = text.split(" ").collect();
        assert_eq!(parts.len(), 2);
        let name = parts[1].to_string();
        let qty = parts[0].parse::<u64>().unwrap();
        Chemical { name, qty }
    }
}

impl Reaction {
    fn parse(line: &str) -> Reaction {
        let parts: Vec<&str> = line.split(" => ").collect();
        assert_eq!(parts.len(), 2);
        let produced = Chemical::parse(parts[1]);
        let requires = parts[0]
            .split(", ")
            .map(|ch_txt| Chemical::parse(ch_txt))
            .collect();
        Reaction { produced, requires }
    }
}

impl Reactions {
    fn parse(input: &str) -> Reactions {
        let react_items = input.lines().map(|line| Reaction::parse(line));
        let mut reactions: HashMap<String, Reaction> = HashMap::new();
        let mut usage: HashMap<String, u64> = HashMap::new();
        for react in react_items {
            for r in react.requires.iter() {
                usage
                    .entry(r.name.clone())
                    .and_modify(|x| *x = *x + 1)
                    .or_insert(1);
            }

            reactions.insert(react.produced.name.clone(), react);
        }
        usage.insert("FUEL".to_string(), 0);
        Reactions {
            reactions,
            to_make: HashMap::new(),
            usage,
        }
    }

    fn find_zero_count_thing_to_make(&mut self) -> Chemical {
        let name = self
            .to_make
            .values()
            .find_map(|v| {
                if *self.usage.get(&v.name).unwrap() == 0 {
                    Some(v.name.clone())
                } else {
                    None
                }
            })
            .unwrap();
        self.to_make.remove(&name).unwrap()
    }

    fn produce(&mut self, chem: &Chemical) {
        println!("producing {:?}", chem);
        self.to_make.insert(chem.name.to_string(), chem.clone());

        while !self.to_make.is_empty() {
            dbg!(&self.reactions);
            dbg!(&self.to_make);
            // go through the "to_make" list and find a thing with count=0
            let next_thing = self.find_zero_count_thing_to_make();
            dbg!(&next_thing);

            if next_thing.name == "ORE" {
                dbg!(&next_thing);
                assert!(self.to_make.is_empty());
                return;
            }
            // dbg!(&self.to_make);

            let reaction = self.reactions.remove(&next_thing.name).unwrap();

            let need = next_thing.qty;
            let produced = reaction.produced.qty;
            let times = (need + produced - 1) / produced;

            for req in reaction.requires.iter() {
                *self.usage.get_mut(&req.name).unwrap() -= 1;
                let added_qty = req.qty * times;
                self.to_make
                    .entry(req.name.clone())
                    .and_modify(|chem| chem.qty += added_qty)
                    .or_insert(Chemical {
                        name: req.name.clone(),
                        qty: added_qty,
                    });
            }
        }
        panic!("nope");
    }
}

fn main() {
    let file = args().nth(1).expect("no filename given");
    let input = fs::read_to_string(file).unwrap();
    let mut reactions = Reactions::parse(&input);
    // dbg!(&reactions);
    reactions.produce(&Chemical {
        name: "FUEL".to_string(),
        qty: 1,
    });
}
