#[derive(Clone,Copy)]
pub enum VersionDalek {
   V1,
   V2
}
pub trait Binario {
 fn george(&self) -> bool;
 fn turing(&self) -> bool;
}
#[derive(Clone,Copy)]
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
impl Binario for Dalek {
   fn george(&self) -> bool {
     return false;
   }
   fn turing(&self) -> bool {
     return true;
   }
}

fn evaluar<T: Binario> (binario: T)-> bool {
  !binario.george() && binario.turing()
}
pub fn escena(daleks: Vec<Dalek> ) -> bool {
   let dalek_iter=daleks.iter();
   let mut evaluacion = false;
   for dalek in dalek_iter {
      dalek.asesinar();
      evaluacion= evaluacion || evaluar(*dalek);
   }
   evaluacion
}
