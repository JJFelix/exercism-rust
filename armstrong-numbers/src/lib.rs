pub fn is_armstrong_number(num: u32) -> bool {
	let num_str = num.to_string();
	let  num_digits = num_str.len() as u32;
    
    let sum: u64 = num_str
        .chars()
        .map(|c| {
            let digit = c.to_digit(10).unwrap() as u64;
            digit.pow(num_digits)
        })
        .sum();        
    sum == num as u64     
}
