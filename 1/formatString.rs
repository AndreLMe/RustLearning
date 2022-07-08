use std::fmt;
fn main(){
	let pi = 3.141592;
	println!("Algo aconteceu {} vezes", 23);
	println!("{0} e {1} são idiotas", "João", "Alice");
	println!("Diferentes hexas: {:X} e {:x}", 123, 123);

	//É possível alinhar um texto a direita.
	println!("{:>15}", 12);
	//Padding de 0s.
	println!("{:0>15}", 12);
	//Podemos passar o width como uma var, para melhorar a view. Adicionar $
	println!("{number:>width$}", number=12, width=15);
	println!("Pi is roughly {:.3}", pi);
	#[allow(dead_code)]
	#[derive(Debug)]
    struct Structure(i32);
	println!("This struct `{:?}` won't print...", Structure(3));
}