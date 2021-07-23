use std::collections::HashMap;

use gccjit::LValue;

#[derive(Debug)]
pub struct GlobVals<'ctx> {
    lvals: Vec<HashMap<String, LValue<'ctx>>>,
}

impl<'ctx> GlobVals<'ctx> {
    pub fn new<'a>() -> Self {
        let mut v: Vec<HashMap<String, LValue>> = Vec::new();
        Self { lvals: v.clone() }
    }
    pub fn add_val<'a>(
        &mut self,
        val: HashMap<std::string::String, LValue<'ctx>>,
    ) -> Vec<HashMap<std::string::String, LValue<'ctx>>> {
        let i = self.lvals.push(val);
        self.lvals.clone()
    }
    pub fn get_val<'a>(&self, name: String) -> Option<LValue<'ctx>> {
        let mut lval = None;
        for i in self.lvals.iter() {
            match i.get_key_value(&name) {
                Some(a) => lval = Some(*a.1),
                None => todo!(),
            };
        }
        lval
    }
}
