use std::env;
use std::process::Command;

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




fn parse_input(fninput:String)->String{
    let temp:Vec<&str> = fninput.split_whitespace().collect();
    println!("{:?}",temp);
    println!("{:?}",temp[0]);
    let mut command_opt = Command::new(temp[0]).output().expect("input was wrong");
    let outp = String::from_utf8_lossy(&command_opt.stdout).to_string();
    return outp;

}




fn shell_loop(mode:bool){
    loop {
        let path = env::current_dir().unwrap();
        let prompt = "".to_string() + &path.display().to_string() + ">>";
        let input = if mode == true {rprompt::prompt_reply(prompt).unwrap()
                            }else{
                                "ls".to_string()
                            };
        if input == "BANG"|| input == "bang"{
            println!("{}",BANG);
            break;
        }
        let opt = parse_input(input);
        println!("{:?}",opt.to_string().replace("\n","/"));
        break;
    }   
}

fn main() {
    shell_loop(false);
}
