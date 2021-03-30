pub struct RandomInfo { //has to be public to use
    pub some_int: i8, //rust will assume everything is private unless made public
    pub some_float: f32,
    pub call_count: i64,
}

impl RandomInfo {
    pub fn new(param_a: i8) -> Self { //capatial Self is the type of self
        Self {
            some_int: param_a,
            some_float: 9.0,
            call_count: 0,
        }
    }

    pub fn is_smaller(&mut self, compare_to: i8) -> bool { //lowercase self is the struct iteself, with the & borrowing memory from the heap
        self.call_count += 1;
        
        self.some_int < compare_to
    }
}