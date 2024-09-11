use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::Not;
use std::sync::Mutex;

lazy_static! {
    static ref EXPLORED1: Mutex<HashMap<&'static str,i32>> = Mutex::new(HashMap::new());
}
lazy_static! {
    static ref EXPLORED2: Mutex<HashMap<&'static str,i32>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref FRONTIER1: Mutex<Vec<Node>> = Mutex::new(vec![]);
}

lazy_static! {
    static ref FRONTIER2: Mutex<Vec<Node>> = Mutex::new(vec![]);
}

lazy_static! {
    static ref TRAVELS: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut map = HashMap::new();
        map.insert(
            "argentina",
            HashMap::from([("west", "chile"), ("east", "uruguay"), ("north", "brazil") ]),
        );
        map.insert(
            "chile",
            HashMap::from([("east", "argentina"), ("north", "peru")]),
        );
        map.insert(
            "paraguay",
            HashMap::from([
                ("south", "argentina"),
                ("north", "bolivia"),
                ("east", "brazil"),
            ]),
        );
        map.insert(
            "uruguay",
            HashMap::from([("west", "argentina"), ("north", "paraguay")]),
        );
        map.insert(
            "bolivia",
            HashMap::from([
                ("south", "argentina"),
                ("north", "peru"),
                ("east", "brazil"),
                ("west", "paraguay"),
            ]),
        );
        map.insert(
            "peru",
            HashMap::from([
                ("east", "bolivia"),
                ("south", "chile"),
                ("north", "ecuador"),
                ("west", "colombia"),
            ]),
        );
        map.insert(
            "brazil",
            HashMap::from([
                ("north", "colombia"),
                ("northeast", "guyana"),
                ("east", "suriname"),
                ("south", "uruguay"),
                ("west", "peru"),
            ]),
        );
        map.insert(
            "colombia",
            HashMap::from([
                ("north", "venezuela"),
                ("east", "brazil"),
                ("south", "peru"),
                ("west", "panama"),
            ]),
        );
        
        map.insert(
            "venezuela",
            HashMap::from([
                ("west", "colombia"),
                ("east", "guyana"),
                ("south", "brazil"),
            ]),
        );

        map.insert(
            "guyana",
            HashMap::from([("south", "brazil"), ("west", "suriname")]),
        );
        
        map.insert(
            "suriname",
            HashMap::from([("east", "guyana"), ("south", "brazil")]),
        );
        
        map.insert(
            "ecuador",
            HashMap::from([("south", "peru"), ("east", "colombia")]),
        );
        
        map.insert("french_guiana", HashMap::from([("south", "brazil")]));
        
        map.insert(
            "mexico",
            HashMap::from([
                ("north", "united_states"),
                ("south", "guatemala"),
                ("east", "belize"),
            ]),
        );
        
        map.insert(
            "guatemala",
            HashMap::from([
                ("north", "mexico"),
                ("east", "belize"),
                ("south", "honduras"),
            ]),
        );

        map.insert("belize", HashMap::from([("west", "guatemala")]));
        
        map.insert(
            "honduras",
            HashMap::from([
                ("north", "guatemala"),
                ("east", "nicaragua"),
                ("south", "el_salvador"),
            ]),
        );
        
        map.insert(
            "nicaragua",
            HashMap::from([("west", "honduras"), ("south", "costa_rica")]),
        );

        map.insert("el_salvador", HashMap::from([("north", "honduras")]));
        
        map.insert("costa_rica", HashMap::from([("north", "nicaragua")]));
        
        map.insert(
            "panama",
            HashMap::from([("west", "colombia"), ("north", "costa_rica")]),
        );
        
        map.insert("united_states", HashMap::from([("south", "mexico")]));
        map
    };
}

lazy_static! {
    static ref SCORES: HashMap<&'static str, HashMap<&'static str, i32>> = {
        let mut map = HashMap::new();
        map.insert(
            "argentina",
            HashMap::from([("west", 1500), ("east", 350), ("north", 2500)]),
        );

        map.insert("chile", HashMap::from([("east", 1500), ("north", 3200)]));

        map.insert(
            "paraguay",
            HashMap::from([("south", 1100), ("north", 1300), ("east", 1200),]),
        );

        map.insert("uruguay", HashMap::from([("west", 350), ("north", 1100),]));

        map.insert(
            "bolivia",
            HashMap::from([
                ("south", 2500),
                ("north", 1800),
                ("east", 2900),
                ("west", 1300),
            ]),
        );

        map.insert(
            "peru",
            HashMap::from([
                ("east", 1800),
                ("south", 3200),
                ("north", 1700),
                ("west", 3200),
            ]),
        );

        map.insert(
            "brazil",
            HashMap::from([
                ("north", 4800),
                ("east", 6200),
                ("south", 2000),
                ("west", 3400),
                ("northeast", 4900),
            ]),
        );

        map.insert(
            "venezuela",
            HashMap::from([("east", 1400), ("south", 5000), ("west", 1300),]),
        );
 
// map.insert(
//             "venezuela",
//             HashMap::from([
//                 ("west", "colombia"),
//                 ("east", "guyana"),
//                 ("south", "brazil"),
//             ]),
//         );

        map.insert(
            "colombia",
            HashMap::from([("west", 1230), ("north", 1300), ("east", 4800), ("south", 3200),]),
        );

 // map.insert(
 //            "colombia",
 //            HashMap::from([
 //                ("north", "venezuela"),
 //                ("east", "brazil"),
 //                ("south", "peru"),
 //                ("west", "panama"),
 //            ]),
 //        );



        map.insert("ecuador", HashMap::from([("south", 1700), ("east", 800),]));

 // map.insert(
 //            "ecuador",
 //            HashMap::from([("south", "peru"), ("east", "colombia")]),
 //        );

        map.insert(
            "mexico",
            HashMap::from([("south", 1100), ("east", 2400), ("north", 2800),]),
        );

// map.insert(
//             "mexico",
//             HashMap::from([
//                 ("north", "united_states"),
//                 ("south", "guatemala"),
//                 ("east", "belize"),
//             ]),
//         );

        map.insert(
            "guatemala",
            HashMap::from([("north", 1100), ("east", 300), ("south", 630),]),
        );
// map.insert(
//             "guatemala",
//             HashMap::from([
//                 ("north", "mexico"),
//                 ("east", "belize"),
//                 ("south", "honduras"),
//             ]),
//         );
//
        map.insert(
            "honduras",
            HashMap::from([("east", 750), ("south", 320), ("north", 630),]),
        );

 // map.insert(
 //            "honduras",
 //            HashMap::from([
 //                ("north", "guatemala"),
 //                ("east", "nicaragua"),
 //                ("south", "el_salvador"),
 //            ]),
 //        );

        map.insert("costa_rica", HashMap::from([("north", 320)]));


        // map.insert("costa_rica", HashMap::from([("north", "nicaragua")]));

        map.insert(
            "nicaragua",
            HashMap::from([("west", 750), ("south", 320),]),
        );

 // map.insert(
 //            "nicaragua",
 //            HashMap::from([("west", "honduras"), ("south", "costa_rica")]),
 //        );

        map.insert("panama", HashMap::from([("west", 1230), ("north", 1370),]));

        // map.insert(
    //             "panama",
    //             HashMap::from([("west", "colombia"), ("north", "costa_rica")]),
    //         );
    
        map.insert("united_states", HashMap::from([("south", 3100)]));

        //mexico

        map.insert("el_salvador", HashMap::from([("north", 320)]));

        // honduras

        map.insert(
            "nicaragua",
            HashMap::from([("west", 750), ("south", 400),]),
        );

//      map.insert(
//             "nicaragua",
//             HashMap::from([("west", "honduras"), ("south", "costa_rica")]),
//         );

        map.insert("belize", HashMap::from([("west", 300)]));


        // map.insert("belize", HashMap::from([("west", "guatemala")]));

        map.insert(
            "guyana",
            HashMap::from([("south", 4800), ("west", 470),]),
        );

// map.insert(
//             "guyana",
//             HashMap::from([("south", "brazil"), ("west", "suriname")]),
//         );

        map.insert(
            "suriname",
            HashMap::from([("east", 470), ("south", 5700),]),
        );

// map.insert(
//             "suriname",
//             HashMap::from([("east", "guyana"), ("south", "brazil")]),
//         );
//

        map.insert("french_guiana", HashMap::from([("south", 330)]));

        map
    };
}


lazy_static! {
    static ref HEURISTIC: HashMap<&'static str, HashMap<&'static str, i32>> = {
        let mut map = HashMap::new();
        map.insert(
            "argentina",
            HashMap::from([("west", 1400), ("east", 200), ("north", 1700)]),
        );

        map.insert("chile", HashMap::from([("east", 1400), ("north", 2400)]));

        map.insert(
            "paraguay",
            HashMap::from([("south", 940), ("north", 1100), ("east", 1000),]),
        );

        map.insert("uruguay", HashMap::from([("west", 200), ("north", 940),]));

        map.insert(
            "bolivia",
            HashMap::from([
                ("south", 2600),
                ("north", 1550),
                ("east", 2400),
                ("west", 1100),
            ]),
        );

        map.insert(
            "peru",
            HashMap::from([
                ("east", 1550),
                ("south", 2400),
                ("north", 1180),
                ("west", 2800),
            ]),
        );

        map.insert(
            "brazil",
            HashMap::from([
                ("north", 3300),
                ("east", 3000),
                ("south", 940),
                ("west", 1550),
                ("northeast", 2500),
            ]),
        );

        map.insert(
            "venezuela",
            HashMap::from([("east", 1100), ("south", 3100), ("west", 1145),]),
        );

        map.insert(
            "colombia",
            HashMap::from([("west", 800), ("north", 1145), ("east", 750), ("south", 1180),]),
        );

        map.insert("ecuador", HashMap::from([("south", 1180), ("east", 800),]));

        map.insert(
            "mexico",
            HashMap::from([("south", 900), ("east", 1500), ("north", 700),]),
        );

        map.insert(
            "guatemala",
            HashMap::from([("north", 1100), ("east", 240), ("south", 420),]),
        );

        map.insert(
            "honduras",
            HashMap::from([("east", 350), ("south", 230), ("north", 420),]),
        );

        map.insert("costa_rica", HashMap::from([("north", 280)]));

        map.insert(
            "nicaragua",
            HashMap::from([("west", 350), ("south", 280),]),
        );

        map.insert("panama", HashMap::from([("west", 750), ("north", 700),]));

        map.insert("united_states", HashMap::from([("south", 3100)]));

        map.insert("el_salvador", HashMap::from([("north", 230)]));

        map.insert(
            "nicaragua",
            HashMap::from([("west", 350), ("south", 280),]),
        );

        map.insert("belize", HashMap::from([("west", 240)]));

        map.insert(
            "guyana",
            HashMap::from([("south", 3300), ("west", 370),]),
        );

        map.insert(
            "suriname",
            HashMap::from([("east", 370), ("south", 3300),]),
        );


        map.insert("french_guiana", HashMap::from([("south", 300)]));

        map
    };
}

fn resolve<'a>(state: &'a str, action: &'a str) -> &'static str {
    if !TRAVELS.contains_key(state) {
        panic!("State not found");
    }
    let state_map = TRAVELS.get(state).unwrap();
    match state_map.get(action) {
        Some(res) => *res,
        None => panic!("Action not found"),
    }
}

fn resolve_score<'a>(state: &'a str, action: &'a str) -> i32 {
    if !SCORES.contains_key(state) {
        panic!("State not found");
    }
    let state_map = SCORES.get(state).unwrap();
    match state_map.get(action) {
        Some(res) => *res,
        None => panic!("Action not found"),
    }
}

fn resolve_heuristic(state: &str, action: &str) -> i32{
    if !SCORES.contains_key(state) {
        panic!("State not found");
    }
    let state_map = SCORES.get(state).unwrap();
    match state_map.get(action) {
        Some(res) => *res,
        None => panic!("Action not found"),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    state: &'static str,
    heuristic: i32,
    score: i32,
    cost: i32,
    route: Vec<&'static str>,
}

impl Node {
    fn expand(&self, action: &str, state: &'static str) -> Node {
        Node {
            state,
            heuristic: resolve_heuristic(self.state, action),
            cost: resolve_score(self.state, action),
            score: resolve_score(self.state, action) + resolve_heuristic(self.state, action) + self.score,
            route: [self.route.clone(), vec![resolve(self.state, action)]].concat(),
        }
    }
}

lazy_static!{
    static ref ROUTES: Mutex<Vec<Vec<&'static str>>> = Mutex::new(vec![]);
}

fn search_route(node: Node, obj: &str, node2: Node, initial_final: bool, limit: &mut i32) {
    if initial_final {
        let actions = TRAVELS
                .get(&node.state)
                .unwrap()
                .keys()
                .cloned()
                .collect::<Vec<_>>();
        if node.clone().state == node2.clone().state {
            let route = {
            let mut route = vec![];
            
            for state in node.route.iter() {
                route.push(*state);
            }
            for state in node2.route.iter().rev(){
                route.push(*state);
            }
            route
        };

        ROUTES.lock().unwrap().push(route.clone());
        dbg!(ROUTES.lock().unwrap());
        std::process::exit(0);
        }            
        
                
        //if node.state == obj{
        //    ROUTES.lock().unwrap().push(node.route.clone());
        //    dbg!(ROUTES.lock().unwrap());
        //    dbg!(node.score);
        //
        //    if FRONTIER1.lock().unwrap().is_empty() {
        //    println!("empty");
        //    std::process::exit(0);
        //    }
        //
        //    let next = FRONTIER1.lock().unwrap().remove(0);
        //    search_route(next, obj, node2, initial_final)
        //}


        //else {
            for act in actions {
                let node_ = node.expand(act, resolve(node.state, act));
                    if EXPLORED1.lock().unwrap().contains_key(node_.state) {
                        if node_.score < *EXPLORED1.lock().unwrap().get(node_.state).unwrap(){
                            EXPLORED1.lock().unwrap().remove(node.state);
                            EXPLORED1.lock().unwrap().insert(node_.state, node_.score);
                        }
                    }
                    else {
                        EXPLORED1.lock().unwrap().insert(node_.state, node_.score);
                        FRONTIER1.lock().unwrap().push(node_);
                    }
            }

            if FRONTIER1.lock().unwrap().is_empty() && FRONTIER2.lock().unwrap().is_empty() {
                println!("empty");
                std::process::exit(0);
            }
            else if FRONTIER1.lock().unwrap().is_empty() {
                *limit = *limit * 2;
                search_route(node.clone(), obj, node2.clone(), initial_final.not(), limit)
            }
            FRONTIER1.lock().unwrap().sort_by(|a, b| a.score.cmp(&b.score));


            let next = FRONTIER1.lock().unwrap().remove(0);
            
            if next.score > *limit {
                *limit = *limit * 2;
                search_route(node, obj, node2, initial_final.not(), limit)
            }
            else {
                search_route(next, obj, node2, initial_final, limit)
            }
        }
    //}
    //
    else{
        let actions = TRAVELS
            .get(&node2.state)
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
            if node.clone().state == node2.clone().state {
                let route = {
                let mut route = vec![];
                for state in node.clone().route.into_iter() {
                    route.push(state);
                }
                for state in node2.route.iter().rev(){
                    route.push(state);
                }
                route
                };
                ROUTES.lock().unwrap().push(route);
                dbg!(ROUTES.lock().unwrap());
                std::process::exit(0);
            }
            
            
        for act in actions.iter() {
            let node_ = node2.expand(act, resolve(node2.state, act));
                if EXPLORED2.lock().unwrap().contains_key(node_.state) {
                    if node_.score < *EXPLORED2.lock().unwrap().get(node_.state).unwrap(){
                        EXPLORED2.lock().unwrap().remove(node2.state);
                        EXPLORED2.lock().unwrap().insert(node_.state, node_.score);
                    }
                }
                else {
                    EXPLORED2.lock().unwrap().insert(node_.state, node_.score);
                    FRONTIER2.lock().unwrap().push(node_);
                }
            }

            if FRONTIER2.lock().unwrap().is_empty() && FRONTIER1.lock().unwrap().is_empty() {
            println!("empty");
            std::process::exit(0);
            }
            else if FRONTIER2.lock().unwrap().is_empty(){
                *limit = *limit * 2;
                search_route(node.clone(), obj, node2.clone(), initial_final.not(), limit)
            }

            FRONTIER2.lock().unwrap().sort_by(|a, b| a.score.cmp(&b.score));
            
            let next = FRONTIER2.lock().unwrap().remove(0);
                   
            if next.score > *limit {
                *limit = *limit * 2;
                search_route(node, obj, node2, initial_final.not(), limit)
            }
            else {
                search_route(node, obj, next, initial_final, limit)
            }
        }


    }


fn main(){
    let init = Node {
        state: "argentina",
        heuristic: 0,
        score: 0,
        cost: 0,
        route: vec![],
    };

    let final_node = Node {
        state: "united_states",
        heuristic: 0,
        score: 0,
        cost: 0,
        route: vec![],
    };
    
    let mut limit = 10000;
    search_route(init, "united_states", final_node, true, &mut limit);
}
