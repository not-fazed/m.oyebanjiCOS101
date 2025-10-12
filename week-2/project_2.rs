fn main () {
	let t:  f64 = 450000.0 * 2.0;
	let m:  f64 = 1500000.0;
	let h:  f64 = 750000.0 * 3.0;
	let d:  f64 = 2850000.0 * 3.0;
	let a:  f64 = 250000.0;

	let s = t + m + h + d + a; // Sum
	println! ("Sum is {}" , s);
	let avg = s / 10.0; //Average
	println! ("Average is {}" , avg);


}