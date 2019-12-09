
fn main() {
	let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
		"eight", "ninth", "10th", "11th", "12th"];
		
	let verses = ["A partridge in a pear tree", "Two turtle doves", "Three French hens",
	"Four calling birds", "Five gold rings", "Six geese a laying", "Seven swans a swimming",
	"Eight maids a milking", "Nine ladies dancing", "10 lords a leaping", 
	"11 pipers piping", "12 drummers drumming"];

	let mut index = 1;

    for day in days.iter() {
    	println!("On the {} day of Christmas my true love sent to me", day);
    	for i in (0..index).rev() {
    		println!("{}", verses[i]);	
    	}
    	println!();
    	index = index + 1;
    }
}
