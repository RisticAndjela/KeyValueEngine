use crate::put::put;
use crate::representation_of_program_as_structure::Program;

impl Program{
    pub fn script(self:&mut Program){
        for i in 1..216{
            let mut str = "key".to_string();
            str.push_str(&i.to_string());
            put(self, str.clone(), "some value".to_string());
            println!("{:?}",str);
        }
    }
}
