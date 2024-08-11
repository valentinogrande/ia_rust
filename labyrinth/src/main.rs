use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashSet;
use std::usize;
use lazy_static::lazy_static; 
use rand::Rng;


lazy_static!{
    static ref EXPLORED: Mutex<Vec<(i32,i32)>> = Mutex::new(vec![]);
    static ref FRONT: Mutex<Vec<Node>> = Mutex::new(vec![]);
}

#[derive(Debug,Clone, Eq, Hash, PartialEq, PartialOrd)]
struct Node {
    state: (i32, i32), // x | y
    heuristic: i32,
    route: Vec<(i32,i32)>
}

impl Node {
    fn new(state: (i32, i32)) -> Node {
        Node {
            state,
            heuristic: 1000,
            route: vec![(0,0)],
        }
    }

    fn get_possible_nodes_to_expand(&self, lab: &Vec<Vec<i32>>) -> Vec<Node>{
        let mut nodes_to_expand: Vec<Node>= vec![];
        
        let (x,y) = self.state;
        
        let x = x as usize;
        let y = y as usize;

        
        if x < 27 && lab[x+1][y] == 0 {
            nodes_to_expand.push(self.expand((x as i32+2,y as i32)));
        }
        if x > 2 &&lab[x-1][y] == 0 {
            nodes_to_expand.push(self.expand((x as i32 - 2,y as i32)));
        }
        if y < 27 && lab[x][y+1] == 0 {
            nodes_to_expand.push(self.expand((x as i32,y as i32 + 2)));
        }
        if y > 2 && lab[x][y-1] == 0 {
            nodes_to_expand.push(self.expand((x as i32,y as i32 - 2)));
        }
        nodes_to_expand
    }

    fn heuristic(&self) -> i32 {
        let (x,y) = self.state;
        (x - 29) + (y - 29)
    }

    fn expand(&self, state: (i32, i32)) -> Node {
        Node {
            state,
            heuristic: self.heuristic(),
            route: {
                let mut rou = vec![];
                for str in self.route.clone(){
                    rou.push(str);
                }
                rou.push(state);
                rou
            }
        }
    }


}

fn get_lab() -> Vec<Vec<i32>>{
        
    let mut lab: Vec<Vec<i32>> = (0..30)
       .map(|_| (0..30).collect())
        .collect();

    for i in 0..30 {
        for j in 0..30 {

            if i % 2 == 0 {
                let mut rang = rand::thread_rng();
                let rand = rang.gen_range(0..10);
                
                if rand >= 6 {
                    lab[i][j] = 1;
                }
                else {
                    lab[i][j] = 0;
                }
            }

            else if j % 2 == 0 {
                let mut rang = rand::thread_rng();
                let rand = rang.gen_range(0..10);
                
                if rand >= 6 {
                    lab[i][j] = 2;
                }
                else {
                    lab[i][j] = 0;
                }
            }

            else {
                lab[i][j] = 0;
            }
        }
    }
    
    lab[0][0] = 0;
    lab[1][0] = 0;

    lab[29][29] = 0;
    lab[28][29] = 0;
    lab

}

fn print_lab(lab: &Vec<Vec<i32>>,route : Vec<(i32, i32)>) {
    for i in 0..30 {
        for j in 0..30 {

            let flag: bool ={
                let mut flag = false;
                for state in route.iter() {
                    if (i,j) == *state{
                        flag = true;
                    }
                }    
                flag
            };
            let i = i as usize;
            let j = j as usize;

            
            if flag {
                print!("☐");
            }
            else if lab[i][j] == 1 {
                print!("|");
            }
            else if lab[i][j] == 2 {
                print!("-");
            }
            else {
                print!(" ");
            }
            print!(" ");

        }
        println!();
    }
}

fn search_rooute(node: &Node, lab: &Vec<Vec<i32>>,i : i32) -> Vec<(i32, i32)> {
    
    if node.state == (28,28) {
        //objective reached

        println!("reached obj, route: ");
        print_lab(lab, node.clone().route);
        node.clone().route
    }
    else{
        let nodes_to_expand = node.get_possible_nodes_to_expand(lab);

        for _node in nodes_to_expand.iter(){
            if !EXPLORED.lock().unwrap().contains(&_node.state){
                FRONT.lock().unwrap().push(_node.clone());
                EXPLORED.lock().unwrap().push(_node.clone().state);
            }
        }

        FRONT.lock().unwrap().sort_by(|a,b| {a.heuristic.cmp(&b.heuristic)});

        let flag = FRONT.lock().unwrap().is_empty();
        if !flag{
            let next = FRONT.lock().unwrap().remove(0);

            search_rooute(&next, lab,i+1)
        }
        else {
            EXPLORED.lock().unwrap().clear();

            let init: Node = Node::new((0,0));

            EXPLORED.lock().unwrap().push(init.clone().state);
            let lab = get_lab();

            search_rooute(&init, &lab,0)

        }


       


    }   

}

fn main() {
    
    // define a initial node whose position is (0, 0)
    let init: Node = Node::new((0,0));

    let lab = get_lab();    

    let route = search_rooute(&init, &lab,0);

}
