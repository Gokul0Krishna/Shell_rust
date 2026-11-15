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


fn shell_loop(){
    loop {
        let input = rprompt::prompt_reply(">>").unwrap();
        if (input == "bang")||(input == "BANG"){
            println!("{}",BANG);
            break;
        }
        else {
            println!("{}",input);
        }
    }
}

fn main() {
    shell_loop();
}
