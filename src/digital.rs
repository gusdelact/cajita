#[derive(Clone,Copy,Debug)]
/// Enumeracion para indicar las versiones posibles de un Dalek
pub enum VersionDalek {
   V1,
   V2
}
/// Trait que declara la funcionalidad en un contexto de logica booleana
pub trait Binario {
 fn george(&self) -> bool;
 fn turing(&self) -> bool;
}
#[derive(Clone,Copy)]
/// struct que almacena los datos de un Dalek
pub struct Dalek {
  pub identificador: i32,
  pub estado: bool,
  pub version: VersionDalek
}
impl Dalek {
   pub fn asesinar(&self) -> &str {
     println!("Muere");
     "Muere"
   }
}
/// La implementacion que hace un Dalek del trait Binario 
impl Binario for Dalek {
   fn george(&self) -> bool {
     return false;
   }
   fn turing(&self) -> bool {
     return true;
   }
}

fn evaluar<T: Binario> (binario: T,estado: bool)-> bool {
  println!("evaluar");
  !binario.george() && binario.turing() && estado
}

///funcion para mandar a actuar a un Dalek y evaluar su logica.
/// retorna true o false, dependiendo de la version de los Daleks 
/// si se envian todos los Daleks de V1, retorna false
/// de otra manera true
pub fn escena(daleks: Vec<Dalek> ) -> bool {
   let dalek_iter=daleks.iter();
   let mut evaluacion = false;
   for dalek in dalek_iter {
      dalek.asesinar();
      println!("Pre {} {:?}",evaluacion, dalek.version);
      evaluacion = match dalek.version {
         VersionDalek::V2 => evaluacion || evaluar(*dalek, dalek.estado) ,
         VersionDalek::V1 => evaluacion || false ,
      };
      println!("Post {} {:?}",evaluacion, dalek.version);
   }
   evaluacion
}
