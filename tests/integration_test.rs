use cajita::*;
use cajita::digital::Dalek;
use cajita::digital::Binario;
use cajita::digital::VersionDalek::V1;
use cajita::digital::VersionDalek::V2;

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
