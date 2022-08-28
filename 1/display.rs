use std::fmt;

struct StructureInterger(i32);

impl fmt::Display for StructureInterger
{
    //Assinatura a função fmt
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        //Parecido com o wrapper da C
        write!(f, "{}", self.0)
    }
}

//implementação de plano cartesiano

#[derive(Debug)]
struct CartesianPlane
{
    x: f64,
    y: f64
}

impl fmt::Display for CartesianPlane
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{0}, {1}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex
{
    real: f64,
    imag: f64
}

impl fmt::Display for Complex
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{0}, {1}i", self.real, self.imag)
    }
}
fn main()
{
   let valor = StructureInterger(32);
   let point = CartesianPlane {x:14.3, y:17.9};
   let complex = Complex {real: 14.5, imag: 16.4};
   println!("valor {}", valor);
   println!("Coordenadas: {}", point);
   println!("Formatado em debug {:?}", point);
   println!("Coordenadas: {}", complex);
   println!("Formatado em debug {:?}", complex);
}
