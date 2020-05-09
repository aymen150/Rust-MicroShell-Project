

 
//###################### EX 2 ####################################


/*
use std::io::{self, Write};
fn main() -> io::Result<()> {

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let stdin = io::stdin();
    let mut user_input = String::with_capacity(256);
     // On prend une référence mutable 

// c'est mieux que de crash avec un unwrap ou expect ;) 
    handle.write_all(b">")?;
    handle.flush()?;
    stdin.read_line(&mut user_input)?; // ? sert à «propager l'erreur» 
    
    Ok(())
}
*/


//###################### EX 3 ####################################
/*
use std::process::Command ;

fn main() {
    let liste_fichier = Command::new("ls")
                         .status()
                         .unwrap();

    println!("process exited with: {}", liste_fichier);
    
    let details_fichier = Command::new("ls")
             .args(&["-l", "-a"])
                     .status()
                     .unwrap();

    println!("process exited with: {}", details_fichier);

    }

*/


//###################### EX 4 ####################################


/*
use std::process::{Command, Stdio};

fn main() {
// stdout must be configured with `Stdio::piped` in order to use
// `echo_child.stdout`
let echo_child = Command::new("ls")
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to start echo process");

// Note that `echo_child` is moved here, but we won't be needing
// `echo_child` anymore
let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

let sed_child = Command::new("grep")
    .args("pro")
    .stdin(Stdio::from(echo_out))
    .spawn()
    .expect("Failed to start sed process");
}
*/


// #######################################
// MISE EN COMMUNs
//####################################
 
// use std::io::{self, Write};

// use std::process::{Command, Stdio};

// Commande Tester ==>
            //let user_input = "ls  -l -a | grep c";
            //let user_input = "ls "; ** faut un espace apres le ls**
            //let user_input = "ls -l";
use regex::Regex;
use std::iter::Iterator;
use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() -> io::Result<()> {

    loop {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let stdin = io::stdin();
        let mut user_input = String::with_capacity(256);
        // On prend une référence mutable 

        // c'est mieux que de crash avec un unwrap ou expect ;) 
        handle.write_all(b">")?;
        handle.flush()?;
        stdin.read_line(&mut user_input)?; // ? sert à «propager l'erreur» 
        


        //let user_input = "ls  -l -a | grep c";

        // on slip les différent commande
        let  div: Vec<&str> = user_input.split("|").collect();

        // on recupére chaque mots
        let  mut cap_1: Vec<&str>=  div[0].split(" ").collect();
        let mut function_1 = cap_1[0];
        cap_1.remove(0);
        
        let mut param_1 = vec![];
        // supprime les espace de la liste
        for i in cap_1{
            if i != ""{
                param_1.push(i);
            }
        }

        // si l'entre ne comprend qu'une seule commande
        if div.len() == 1 {
            let echo_child = Command::new(function_1)
                .args(param_1)
                .status()
                .unwrap();
        }

        // si elle en comprend 2
        else{
            let mut  x: Vec<&str>=  div[1].split(" ").collect();
            let mut cap_2 = vec![];
            for i in x{
                if i != ""{
                    cap_2.push(i);
                }
            }
            let mut function_2 = cap_2[0];
            cap_2.remove(0);
            
            let mut param_2 = vec![];
            for i in cap_2{
                if i != ""{
                    param_2.push(i);
                }
            }
            let echo_child = Command::new(function_1)
                .args(&param_1)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");

            // Note that `echo_child` is moved here, but we won't be needing
            // `echo_child` anymore
            let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

            let sed_child = Command::new(function_2)
                .args(&param_2)
                .stdin(Stdio::from(echo_out))
                .spawn()
                .expect("Failed to start sed process");
        }

    }
Ok(())
}   
