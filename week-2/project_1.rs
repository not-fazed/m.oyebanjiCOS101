fn main () {
	let p: f64 = 520000000.0;
	let r: f64 = 10.0;
	let t: f64 = 5.0;

	let a = p * (1.0 + (r / 100.0)) * t; // Amount
	println! ("Amount is {}" , a);
	let ci = a - p; // Compound Interest
    println! ("Compound interest is {}" , ci);

}