use std::collections::HashMap;

pub struct SymbolTable<'a> {
    pub symbol: HashMap<&'a str, usize>,
    memory: usize,
}

impl<'a> SymbolTable<'a> {

    pub fn new() -> SymbolTable<'a> {

        let mut symb = HashMap::new();
        //add predefined symbols
        symb.insert("SP", 0);
        symb.insert("LCL", 1);
        symb.insert("ARG", 2);
        symb.insert("THIS", 3);
        symb.insert("THAT", 4);
        symb.insert("R0", 0);
        symb.insert("R1", 1);
        symb.insert("R2", 2);
        symb.insert("R3", 3);
        symb.insert("R4", 4);
        symb.insert("R5", 5);
        symb.insert("R6", 6);
        symb.insert("R7", 7);
        symb.insert("R8", 8);
        symb.insert("R9", 9);
        symb.insert("R10", 10);
        symb.insert("R11", 11);
        symb.insert("R12", 12);
        symb.insert("R13", 13);
        symb.insert("R14", 14);
        symb.insert("R15", 15);
        symb.insert("SCREEN", 16384);
        symb.insert("KBD", 24576);

        SymbolTable{symbol: symb, memory: 15}
    }

    pub fn add_symbol(&mut self, sym: &'a str, val: usize) {
        self.symbol.insert(sym, val);
    }

    pub fn get_symbol(&mut self, sym: &'a str) -> usize {
        if self.symbol.contains_key(sym) {
            self.symbol.get(sym).unwrap().clone()
        } else {
            self.add_and_inc(sym)
        }
    }

    fn add_and_inc(&mut self, sym: &'a str) -> usize {
        self.memory = self.memory + 1;
        let mem = self.memory.clone();
        self.add_symbol(sym, mem);
        self.memory.clone()
    }
}
