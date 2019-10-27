#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn filter_in_shoes(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|eachShoe| eachShoe.size == size).collect()
}

#[test]
fn test_filter_in_shoes() {
    let shoes = vec![
        Shoe { size: 12, style: String::from("sneaker") },
        Shoe { size: 10, style: String::from("sport shoes") },
        Shoe { size: 12, style: String::from("climber") }
    ];
    let my_size = filter_in_shoes(shoes, 12);

    assert_eq!(my_size, vec![Shoe { size: 12, style: String::from("sneaker") },
                             Shoe { size: 12, style: String::from("climber") }]
    )
}


struct Counter {
    count:u32,
}

impl Counter{
    fn new () -> Counter{
        Counter { count:0}
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        self.count +=1;
        if self.count < 6{
            Some(self.count)
        }else{
            None
        }
    }
}

#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();
    assert_eq!(counter.next() , Some(1));
}

#[test]
fn using_other_iterator_trait_methods(){
    let sum:u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a,b)| a*b)
        .filter(|x| x%3 == 0)
        .sum();
    assert_eq!(18,sum);
}