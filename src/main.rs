use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct Data{
    pub name: String,
    pub quiz1: u32,
    pub quiz2: u32,
    pub quiz3: u32,
}

pub struct Node{
    pub data: Data,
    pub next: Option<Box<Node>>,
}

pub struct LinkedList{
    pub head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn add_rec(&mut self, rec: Data) {

        let new_node = Node {
            data: rec,
            next: None,
        };

        if self.head.is_none() {
            self.head = Some(Box::new(new_node))
        }
        else {
            let mut p = &mut self.head;
            while p.as_ref().unwrap().next.is_some() {
                p = &mut p.as_mut().unwrap().next;
            }
            p.as_mut().unwrap().next = Some(Box::new(new_node));
        }
    }

    fn save_file(&self) -> io::Result<()> {
        let mut fp = File::create("main_db.txt")?;
        let mut p = &self.head;

        while let Some(node) = p {
            let fp_str = format!("{} {} {} {:}\n", node.data.name, node.data.quiz1,
                                 node.data.quiz2, node.data.quiz3);
            fp.write_all(fp_str.as_bytes())?;
            p = &node.next;
        }
        Ok(())
    }

    fn read_file(&mut self) -> io::Result<()> {
        let fp = File::open("main_db.txt")?;
        let reader = BufReader::new(fp);
        
        for line in reader.lines()  {
            let line_content = line?;
            let parts: Vec<&str> = line_content.trim().split_whitespace().collect();

            if parts.len() >= 4 {
                let n = parts[0..parts.len() - 3].join(" ");
                let q1: u32 = parts[parts.len() - 3].parse().unwrap_or_default();
                let q2: u32 = parts[parts.len() - 2].parse().unwrap_or_default();
                let q3: u32 = parts[parts.len() - 1].parse().unwrap_or_default();

                let data = Data {
                    name: n,
                    quiz1: q1,
                    quiz2: q2,
                    quiz3: q3,
                };
                self.add_rec(data);
            }

        }
        Ok(())
    }


    fn display(&self)  {
        let mut p = &self.head;

        while let Some(node) = p.as_ref() {
            println!("{} {} {} {}", node.data.name, node.data.quiz1, node.data.quiz2, node.data.quiz3);
            io::stdout().flush().unwrap();
            p = &node.next;
        }
    }
}

fn new_rec() -> Data{
    let scan = io::stdin();
    let mut out = io::stdout();
    println!("Student Information");
    print!("Name: ");
    out.flush().unwrap();
    let mut n_name = String::new();
    scan.read_line(&mut n_name).expect("Invalid name");
    print!("Quiz 1: ");
    out.flush().unwrap();
    let mut n_quiz1 = String::new();
    scan.read_line(&mut n_quiz1).expect("Invalid input");
    let n_quiz1: u32 = n_quiz1.trim().parse().expect("Invalid score");
    print!("Quiz 2: ");
    out.flush().unwrap();
    let mut n_quiz2 = String::new();
    scan.read_line(&mut n_quiz2).expect("Invalid input");
    let n_quiz2: u32 = n_quiz2.trim().parse().expect("Invalid score");
    print!("Quiz 3: ");
    out.flush().unwrap();
    let mut n_quiz3 = String::new();
    scan.read_line(&mut n_quiz3).expect("Invalid input");
    let n_quiz3: u32 = n_quiz3.trim().parse().expect("Invalid score");

    let new_data = Data {
        name: n_name,
        quiz1: n_quiz1,
        quiz2: n_quiz2,
        quiz3: n_quiz3,
    };
    return new_data;
}

fn main() {
    let mut list = LinkedList::new();
    let scan = io::stdin();
    let mut out = io::stdout();
    list.read_file().expect("File does not exist");
    loop {
        println!("Student Record");
        print!("1. Add new record\n2. View record\n3. Exit\nChoice: ");
        out.flush().unwrap();
        let mut choice = String::new();
        scan.read_line(&mut choice).expect("Invalid input");
        let choice: u32 = choice.trim().parse().expect("Choose from 1-3 only!");
        match choice{
            1 => {
                let new_data: Data = new_rec();
                list.add_rec(new_data);
                list.save_file().expect("Failed to save file");
            }
            2 => {
                list.display();
            }
            3 => {
                std::process::exit(0);
            }
            _ => {}
        }
    }



}


