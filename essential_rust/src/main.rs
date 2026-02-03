
fn main() {



    let mut main_count: i32 = 0;

    'main: loop {
        println!("Outer : {main_count}");
        let mut inner_count: i32 = 0;


        loop {
            println!("inner : {inner_count}");
            inner_count +=1;


            if inner_count == 3 {
                println!("---");
            }

            if main_count == 3 {
                println!("Existing out os all loops ");
                break 'main;
            }
            main_count += 1;
        }
    }
}




