use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufRead},
    rc::{Rc, Weak},
};

// Some comments for future me. Using a Weak pointer to the parent prevents a
// cyclical reference between a parent pointing at a child and the child
// pointing at the parent. Because we allocate the memory for pointed at objects
// on the heap with reference counted borrows, this causes us to allocate
// infinite memory.
//
// This is resolved by making the parent pointer a weak reference,
// see https://doc.rust-lang.org/book/ch15-06-reference-cycles.html?highlight=Weak.
//
// We make the parent a weak reference because if a parent has no children, we
// do not want to drop it. However, if a child has no parent, we want to drop
// it, because we should never visit it (unless our current pointed is in the
// orphaned branch of the tree, in which case it will not drop until that
// pointer moves out).
#[derive(Debug)]
struct Node<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
    name: String,
    files: Vec<(usize, String)>,
}

impl<'a> Node<'a> {
    /// Return accumulated size of all files stored directly in this directory
    fn get_direct_filesize(&self) -> usize {
        let mut accumulator = 0;
        for file in self.files.iter() {
            accumulator += file.0;
        }
        return accumulator;
    }

    /// Return file size of all files and all subdirectories of this directory
    fn get_total_filesize(&self) -> usize {
        if self.children.len() == 0 {
            return self.get_direct_filesize();
        } else {
            let mut accumulator = self.get_direct_filesize();
            for child in self.children.iter() {
                accumulator += child.borrow().get_total_filesize();
            }
            return accumulator;
        }
    }
}

#[derive(Debug)]
struct FileTree<'a> {
    root: Rc<RefCell<Node<'a>>>,
    curr_ptr: Option<Rc<RefCell<Node<'a>>>>,
}

impl<'a> FileTree<'a> {
    fn empty() -> FileTree<'a> {
        let root = Node {
            parent: None,
            children: vec![],
            name: String::from("/"),
            files: vec![],
        };
        FileTree {
            root: Rc::new(RefCell::new(root)),
            curr_ptr: None,
        }
    }

    /// Point at root directory.
    fn go_home(&mut self) {
        self.curr_ptr = Some(Rc::clone(&self.root));
    }

    /// Change directory pointed at to parent of currently pointed at directory.
    fn move_out(&mut self) {
        let current = Rc::clone(self.curr_ptr.as_ref().unwrap());
        if current.borrow().name == "/" {
            return ();
        } else {
            let parent = Weak::clone(current.borrow().parent.as_ref().unwrap());
            self.curr_ptr = Some(parent.upgrade().unwrap());
        }
    }

    /// Change directory being pointed at to subdirectory of currently pointed
    /// at directory `dir`.
    fn change_directory(&mut self, dir: String) {
        let curr_node = Rc::clone(self.curr_ptr.as_ref().unwrap());
        let mut assigned = false;
        for child in curr_node.borrow_mut().children.iter() {
            if child.borrow().name == dir {
                self.curr_ptr = Some(Rc::clone(child));
                assigned = true;
                break;
            }
        }
        // path not in children of this node, create new node
        if !assigned {
            let new_node = Rc::new(RefCell::new(Node {
                parent: Some(Rc::downgrade(self.curr_ptr.as_ref().unwrap())),
                children: vec![],
                name: dir,
                files: vec![],
            }));
            curr_node.borrow_mut().children.push(Rc::clone(&new_node));
            self.curr_ptr = Some(Rc::clone(&new_node));
        }
    }

    /// Push a file into the file buffer of the currenty pointed at directory
    fn add_file(&mut self, file: (usize, String)) {
        self.curr_ptr
            .as_mut()
            .unwrap()
            .borrow_mut()
            .files
            .push(file);
    }

    /// Traverse the filetree, visiting each directory and marking down its
    /// name and size. This is mad inefficient because I redundantly calculate
    /// the size of all subdirectories over and over as I walk down the tree,
    /// but I'm so far in I can't be bothered to make this efficient right now.
    fn traverse_and_store_dirsize(&self) -> Vec<(String, usize)> {
        let mut out: Vec<(String, usize)> = vec![];
        let mut queue: Vec<Rc<RefCell<Node>>> = vec![];
        queue.push(Rc::clone(&self.root));
        while queue.len() > 0 {
            let curr_node = Rc::clone(&queue.pop().as_ref().unwrap());
            out.push((
                curr_node.borrow().name.clone(),
                curr_node.borrow().get_total_filesize(),
            ));
            for child in curr_node.borrow().children.iter() {
                queue.push(Rc::clone(&child));
            }
        }
        out
    }
}

struct Command {
    cmd: String,
    arg: String,
    output: Vec<String>,
}

/// Today is pretty interesting! Most of the challenge is again in parsing
/// the input into an appropriate data structure. I think my strategy is going
/// to be as follows:
///
///   - Initialize an empty tree structure
///   - Parse commands line by line
///   - On each `cd`, move through the tree, adding a new node if necessary
///   - On each `ls`, read the files and associate them with the current node
///
/// Then I just have to traverse the tree and calculate what I need to.
///
/// My chipper attitude when writing the above comment was dashed on the rocky
/// shores of the rust borrow checker. I am grown.
fn main() {
    // Init input reader
    let file = File::open("day7_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let lineiter = filebuf.lines();
    // Init tree structure
    let mut filetree = FileTree::empty();
    filetree.curr_ptr = Some(Rc::clone(&filetree.root));

    // Parse all the input into a command stack
    let mut commands: Vec<Command> = vec![];
    for line in lineiter {
        let line = line.unwrap();
        let temp = line.split(' ').map(str::to_owned).collect::<Vec<String>>();
        if temp[0].contains('$') {
            // parse command
            if temp[1].contains("cd") {
                commands.push(Command {
                    cmd: temp[1].clone(),
                    arg: temp[2].clone(),
                    output: vec![],
                })
            } else if temp[1].contains("ls") {
                commands.push(Command {
                    cmd: temp[1].clone(),
                    arg: String::from(""),
                    output: vec![],
                });
            }
        } else {
            // dump output of ls into current directory
            let mut last_command = commands.pop().unwrap();
            last_command.output.push(temp.join(" "));
            commands.push(last_command);
        }
    }

    // Use command stack to construct filetree
    for command in commands {
        match command.cmd.as_str() {
            "cd" => {
                if command.arg.as_str() == "/" {
                    filetree.go_home();
                } else if command.arg.as_str() == ".." {
                    filetree.move_out();
                } else {
                    filetree.change_directory(command.arg.to_string());
                }
            }
            "ls" => {
                for line in command.output {
                    if &line[0..3] == "dir" {
                        continue;
                    } else {
                        // parse ls and populate the files of this directory
                        let ls_out = line.split(' ').map(str::to_owned).collect::<Vec<String>>();
                        filetree.add_file((ls_out[0].parse().unwrap(), ls_out[1].clone()));
                    }
                }
            }
            _ => continue,
        }
    }

    // Recurse through file tree and store the size of each directory
    let mut dir_and_sizes = filetree.traverse_and_store_dirsize();
    // Sort the directory sizes, print the accumulated size of all small
    // directories
    dir_and_sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut accumulator = 0;
    for (_, size) in dir_and_sizes {
        if size > 100000 {
            println!("{}", accumulator);
            return ();
        }
        accumulator += size;
    }
}
