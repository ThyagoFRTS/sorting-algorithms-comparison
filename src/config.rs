//pub let vector_sizes = [100, 1000, 10000, 100000, 1000000, 10000000];
//pub let algorithms = [100, 1000, 10000, 100000, 1000000, 10000000];

pub struct Config<'a>{
    sizes: Vec<usize>,
    methods: Vec<&'a str>,
    cases: Vec<&'a str>,
}

impl Config<'static> {
    pub fn new() -> Self {
        Self {
            //sizes: vec![100, 1000, 10000, 100000, 1000000, 10000000],
            //sizes: vec![10000000, 1000000, 100000, 10000, 1000, 100],
            sizes: vec![ 10000, 1000, 100],
            methods: vec!["bubblesort", "quicksort", "selectionsort"],
            //cases: vec!["pior".to_string(), "medio".to_string(), "melhor".to_string()],
            cases: vec![ "medio", "melhor"],
        }
    }



    pub fn sizes(&self) -> Vec<usize> {
        self.sizes.clone()
    }

    pub fn methods(&self) -> Vec<&str> {
        self.methods.clone()
    }

    pub fn cases(&self) -> Vec<&str>{
        self.cases.clone()
    }
}