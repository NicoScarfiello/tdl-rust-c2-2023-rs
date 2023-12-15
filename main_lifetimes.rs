// struct FileHolder<'a> {
//     file_name: &'a str,
// }

// impl<'a> FileHolder<'a> {
//     fn new(file_name: &'a str) -> Self {
//         FileHolder { file_name }
//     }

//     fn get_a_file_name<'b>(&'b self) -> &'b str {
//         self.file_name
//     }
// }


struct FileHolderStatic {
    file_name: &'static str,
}

impl FileHolderStatic {
    fn new(file_name: &'static str) -> Self {
        FileHolderStatic { file_name }
    }

    fn get_a_file_name_static(&self) -> &'static str {
        self.file_name
    }
}

fn main() {
    //let a_file_name;
    let static_file_name;

    {
        let file_name_a: &str = "archivo_a.txt";
        //let file_holder_a = FileHolder::new(file_name_a);
        let file_holder_static = FileHolderStatic::new(file_name_a);

        //a_file_name = file_holder_a.get_a_file_name();
        static_file_name = file_holder_static.get_a_file_name_static();
    }
    
    //println!("Lifetime A externo fuera del bloque: {}", a_file_name);

    println!("Lifetime static externo fuera del bloque: {}", static_file_name);
}