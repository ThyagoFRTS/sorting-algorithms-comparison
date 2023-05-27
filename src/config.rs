//pub let vector_sizes = [100, 1000, 10000, 100000, 1000000, 10000000];
//pub let algorithms = [100, 1000, 10000, 100000, 1000000, 10000000];

pub struct Config {
    sizes: Vec<usize>,
    methods: Vec<String>,
    cases: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            //sizes: vec![100, 1000, 10000, 100000, 1000000, 10000000],
            sizes: vec![10000000, 1000000, 100000, 10000, 1000, 100],
            methods: vec!["bubblesort".to_string(), "quicksort".to_string(), "selectionsort".to_string()],
            cases: vec!["pior".to_string(), "medio".to_string(), "melhor".to_string()],
        }
    }



    pub fn sizes(&self) -> &[usize] {
        self.sizes.as_ref()
    }

    pub fn methods(&self) -> Vec<String> {
        self.methods.clone()
    }

    pub fn cases(&self) -> &[String] {
        self.cases.as_ref()
    }
}