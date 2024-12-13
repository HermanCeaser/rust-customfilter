fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let condition = FilterCondition { value: &5 };
    let filtered_numbers = custom_filter(numbers.iter(), &condition);
    println!("Filtered numbers: {:?}", filtered_numbers);

    
    let characters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let char_condition = FilterCondition { value: &'c' };

    let filtered_characters = custom_filter(&characters, &char_condition);

    // Print the filtered characters
    println!("Filtered characters: {:?}", filtered_characters);
}

struct FilterCondition<T> {
    value: T,
}

impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        *item > self.value
    }
}

fn custom_filter<T, C>(collection: C, condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialOrd,
    C: IntoIterator<Item = T>,
{
    let mut result = Vec::new();
    for item in collection {
        if condition.is_match(&item) {
            result.push(item)
        }
    }
    result
}
