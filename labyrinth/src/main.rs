use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use lazy_static::lazy_static; 
use rand::random;
use rand::thread_rng;
use rand::Rng;


lazy_static!{
    static ref FRONT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

struct Node {
    state: String,
    pos: (i32,i32), // x | y
    father: Option<Box<Node>>,
}

impl Node {
    fn new(state: String, pos: (i32,i32)) -> Node {
        Node {
            state,
            pos,
            father: None,
        }
    }

    fn expand(self, state: String, pos: (i32,i32)) -> Node {
        Node {
            state,
            pos,
            father: Some(Box::new(self)),
        }
    }


}

fn get_lab() -> Vec<Vec<i32>>{
        
    let mut lab: Vec<Vec<i32>> = (0..20)
       .map(|_| (0..20).collect())
        .collect();

    for i in 0..20 {
        for j in 0..20 {
            if i % 2 == 0 {
                let mut rang = rand::thread_rng();
                let rand = rang.gen_range(0..10);
                
                if rand >= 7 {
                    lab[i][j] = 1;
                }
                else {
                    lab[i][j] = 0;
                }
            }

            else if j % 2 == 0 {
                let mut rang = rand::thread_rng();
                let rand = rang.gen_range(0..10);
                
                if rand >= 7 {
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


    lab

}

fn print_lab(lab: Vec<Vec<i32>>) {
    for i in 0..20 {
        for j in 0..20 {
            
            if lab[i][j] == 0 {
                print!("☐");
            }
            if lab[i][j] == 1 {
                print!("|");
            }
            if lab[i][j] == 2 {
                print!("-");
            }
            print!(" ");

        }
        println!();
    }
}

fn main() {

    let lab = get_lab();    
    print_lab(lab);

}
