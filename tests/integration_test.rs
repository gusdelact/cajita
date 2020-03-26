use cajita::digital::Dalek;
use cajita::digital::Binario;
use cajita::digital::VersionDalek::V1;
use cajita::digital::VersionDalek::V2;

///! en esta prueba de integracion, se agregan nuevos elementos para probar la extension del Crate
enum TipoPoder {
  NORMAL,
  ESPECIAL,
  INVENCIBLE,
}
struct Cyborg<'a> {
   name : &'a str,
   nivel_poder : TipoPoder,
}

impl Binario for Cyborg<'_> {
  fn george(&self) -> bool {
     match self.nivel_poder {
       TipoPoder::NORMAL => false,
       TipoPoder::ESPECIAL => true,
       TipoPoder::INVENCIBLE => true,
     }
  }
  fn turing(&self) -> bool {
   true
  }
}
#[test]
fn un_dalek() {
  let d00 = Dalek { identificador: 42, estado: false , version: V2 };
  let mensaje=d00.asesinar();
  d00.george();
}
#[test] 
fn una_escena_daleks() {
         let daleks = [
         Dalek { identificador: 42, estado: false , version: V1 },
         Dalek { identificador: 53, estado: true  , version: V2},
         Dalek { identificador: 64, estado: true  , version: V1},
         Dalek { identificador: 75, estado: false , version: V2},
       ];
       let resultado=cajita::digital::escena(daleks.to_vec());
       assert!(resultado);
}

#[test]
fn un_cyborg() {
  let cyb0 = Cyborg {name : "1138", nivel_poder: TipoPoder::ESPECIAL };
  assert!(cyb0.george());
}
