use std::collections::HashMap;

pub fn parse(string: String) -> (String, String, String) {
    let v: Vec<_> = string.split('=').collect();
    let dest = match v.len() {
        2 => v[0].to_owned(),
        _ => "NULL".to_owned(),
    };
    let rem = match v.len() {
        1 => v[0].to_owned(),
        _ => v[1].to_owned(),
    };

    let v2: Vec<_> = rem.split(';').collect();

    let jump = match v2.len() {
        2 => v2[1].to_owned(),
        _ => "NULL".to_owned(),
    };

    let instr = v2[0].to_owned();

    (dest, instr, jump)
}

pub fn get_instr(string: String) -> String {
    let (d, i, j) = parse(string);
    let mut cp = CParse::new();

    format!("111{}{}{}", cp.get_inst(i), cp.get_dest(d), cp.get_jump(j))
}

pub struct CParse {
    dest: HashMap<String, String>,
    inst: HashMap<String, String>,
    jump: HashMap<String, String>,
}

impl CParse {

    pub fn get_dest(&mut self, d: String) -> String {
        self.dest.get(&d).unwrap().to_owned()
    }

    pub fn get_inst(&mut self, i: String) -> String {
        self.inst.get(&i).unwrap().to_owned()
    }

    pub fn get_jump(&mut self, j: String) -> String {
        self.jump.get(&j).unwrap().to_owned()
    }

    pub fn new() -> CParse {
        let mut dest = HashMap::new();
        let mut inst = HashMap::new();
        let mut jump = HashMap::new();

        dest.insert("NULL".to_owned(), "000".to_owned());
        dest.insert("M".to_owned(), "001".to_owned());
        dest.insert("D".to_owned(), "010".to_owned());
        dest.insert("MD".to_owned(), "011".to_owned());
        dest.insert("A".to_owned(), "100".to_owned());
        dest.insert("AM".to_owned(), "101".to_owned());
        dest.insert("AD".to_owned(), "110".to_owned());
        dest.insert("AMD".to_owned(), "111".to_owned());

        jump.insert("NULL".to_owned(), "000".to_owned());
        jump.insert("JGT".to_owned(), "001".to_owned());
        jump.insert("JEQ".to_owned(), "010".to_owned());
        jump.insert("JGE".to_owned(), "011".to_owned());
        jump.insert("JLT".to_owned(), "100".to_owned());
        jump.insert("JNE".to_owned(), "101".to_owned());
        jump.insert("JLE".to_owned(), "110".to_owned());
        jump.insert("JMP".to_owned(), "111".to_owned());

        inst.insert("0".to_owned(), "0101010".to_owned());
        inst.insert("1".to_owned(), "0111111".to_owned());
        inst.insert("-1".to_owned(), "0111010".to_owned());
        inst.insert("D".to_owned(), "0001100".to_owned());
        inst.insert("A".to_owned(), "0110000".to_owned());
        inst.insert("M".to_owned(), "1110000".to_owned());
        inst.insert("!D".to_owned(), "0001101".to_owned());
        inst.insert("!A".to_owned(), "0110001".to_owned());
        inst.insert("!M".to_owned(), "1110001".to_owned());
        inst.insert("-D".to_owned(), "0001111".to_owned());
        inst.insert("-A".to_owned(), "0110011".to_owned());
        inst.insert("-M".to_owned(), "1110011".to_owned());
        inst.insert("D+1".to_owned(), "0011111".to_owned());
        inst.insert("A+1".to_owned(), "0110111".to_owned());
        inst.insert("M+1".to_owned(), "1110111".to_owned());
        inst.insert("D-1".to_owned(), "0001110".to_owned());
        inst.insert("A-1".to_owned(), "0110010".to_owned());
        inst.insert("M-1".to_owned(), "1110010".to_owned());
        inst.insert("D+A".to_owned(), "0000010".to_owned());
        inst.insert("D+M".to_owned(), "1000010".to_owned());
        inst.insert("D-A".to_owned(), "0010011".to_owned());
        inst.insert("D-M".to_owned(), "1010011".to_owned());
        inst.insert("A-D".to_owned(), "0000111".to_owned());
        inst.insert("M-D".to_owned(), "1000111".to_owned());
        inst.insert("D&A".to_owned(), "0000000".to_owned());
        inst.insert("D&M".to_owned(), "1000000".to_owned());
        inst.insert("D|A".to_owned(), "0010101".to_owned());
        inst.insert("D|M".to_owned(), "1010101".to_owned());

        CParse{ dest:dest, inst:inst, jump:jump }
    }
}
