#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        return Store{products: products}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>
    // expected public fields
}
impl Cart {
    pub fn new() -> Cart {
        return Cart{items: Vec::new(), receipt: Vec::new()}
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for ss in &s.products {
            if ss.0 == ele {
                //println!("{:?}", ss);
                self.items.push(ss.clone());
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut vi32: Vec<f32> = Vec::new();
        for ss in &self.items {
            vi32.push(ss.1);
        }
        let number_gratis: u32 = ((vi32.len() - (vi32.len()%3)) / 3) as u32;
        //let mut count_gratis: u32 = 0;
        //println!("{}", number_gratis);
        let mut all_lower: Vec<f32> = Vec::new();
        while all_lower.len() < (number_gratis as usize){
            if all_lower.len() < 1 {
                all_lower.push(lower(&vi32, &0.0));
            } else {
                //println!("{:?}", all_lower[all_lower.len()-1]);
                all_lower.push(lower(&vi32, &all_lower[all_lower.len()-1]));
            }
            //count_gratis += 1;
        }
        //println!("all_lower: {:?}", all_lower);
        let mut deduce: f32 = 0.0;
        for i in &all_lower {
            deduce += *i;
        }
        let mut val_tot: f32 = 0.0;
        for i in &vi32 {
            let mut is_not_lower: bool = true;
            for lower in &all_lower {
                if *i == *lower {
                    is_not_lower = false;
                }
            }
            if is_not_lower {
                val_tot += i;
            }
        }
 
 
        let pourcent_reduce: f32 = val_tot / (val_tot + deduce);
        let mut not_sorted: Vec<f32> = Vec::new();
        for val in &vi32 {
            not_sorted.push(((*val*pourcent_reduce)*100.0).round()/100.0);
        }
        //println!("not_sorted: {:?}", not_sorted);
        while self.receipt.len() < not_sorted.len() {
            if self.receipt.len() < 1 {
                self.receipt.push(lower(&not_sorted, &0.0));
            } else {
                //println!("min receipt: {:?}", &self.receipt[self.receipt.len()-1]);
                self.receipt.push(lower(&not_sorted, &self.receipt[self.receipt.len()-1]));
            }
        }
        //println!("self: {:?}", self.receipt);
        return self.receipt.clone()
    }
}

pub fn lower(arr: &Vec<f32>, min_sup: &f32) -> f32 {
    //println!("min_sup: {}", min_sup);
    let mut lower: f32 = 0.0;
    for val in arr {
        if *val > *min_sup {
            let mut is_lower: bool = true;
            for val2 in arr {
                if val2 > min_sup {
                    if val > val2 {
                        is_lower = false;
                    }
                }
            }
            if is_lower {
                lower = *val;
                //println!("to lower: {}", lower);
            }
        }
    }
    //println!("  ");
    return lower
}