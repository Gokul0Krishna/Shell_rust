use std::{env, ops::ControlFlow::Break};

const BANG:&str = r#" 
 \
 /                                 />
 \__+_____________________/\/\___/ /|
 ()______________________      / /|/\
             /0 0  ---- |----    /---\
            |0 o 0 ----|| - \ --|      \
             \0_0/____/ |    |  |\      \
                         \__/__/  |      \
Bang! Bang!                       |       \
                                  |         \
                                  |__________|
"#;
const PRIMARY_COMMAND:[&str;2] = ["lol","olo"]; 


fn parse_input(fninput:String){
    let temp:Vec<&str> = fninput.split_whitespace().collect();
    if temp.len() == 0{
        return;
    }
    for i in temp{
        println!("{}",i);
    }
}




fn shell_loop(mode:bool){
    loop {
        let path = env::current_dir().unwrap();
        let prompt = "".to_string() + &path.display().to_string() + ">>";
        let input = if mode == true {rprompt::prompt_reply(prompt).unwrap()
                            }else{
                                "ls -l".to_string()
                            };
        if input == "BANG"|| input == "bang"{
            println!("{}",BANG);
            break;
        }
        parse_input(input);
        break;
    }   
}

fn main() {
    shell_loop(false);
}
