//! # Ejemplo
//!
//! ```rust
//! extern crate cajita;
//! use cajita::digital::*;
//! use cajita::digital::Dalek;
//! use cajita::digital::Binario;
//! use cajita::digital::VersionDalek;
//!   let d00 = Dalek { identificador: 42, estado: false , version: VersionDalek::V1};
//!   let mensaje=d00.asesinar();
//!   assert_eq!(mensaje , "Muere");
//!   d00.george();
//!       let daleks = [
//!         Dalek { identificador: 42, estado: false , version: VersionDalek::V1 },
//!         Dalek { identificador: 53, estado: true  , version: VersionDalek::V2},
//!         Dalek { identificador: 64, estado: true  , version: VersionDalek::V1},
//!         Dalek { identificador: 75, estado: false , version: VersionDalek::V2},
//!       ];
//!       let resultado=escena(daleks.to_vec());
//!       assert!(resultado);
//! ```
//!
pub mod digital;
#[cfg(test)]
mod tests { 
    use crate::digital::*;
    use crate::digital::Dalek;
    use crate::digital::Binario;
    use crate::digital::VersionDalek;
    #[test]
    fn dalek_life() {
        let d00 = Dalek { identificador: 42, estado: false, version: VersionDalek::V1  };
        let mensaje=d00.asesinar();
        assert_eq!(mensaje , "Muere");
    }
    #[test]
    fn dalek_logico() {
        let d00 = Dalek { identificador: 42, estado: false, version: VersionDalek::V2  };
        d00.george();
        d00.turing();
        assert!(d00.george() || d00.turing());
    }
    #[test]
    fn dalek_escena() {
       let daleks = [
         Dalek { identificador: 42, estado: false , version: VersionDalek::V1 },
         Dalek { identificador: 53, estado: true  , version: VersionDalek::V2},
         Dalek { identificador: 64, estado: true  , version: VersionDalek::V1},
         Dalek { identificador: 75, estado: false , version: VersionDalek::V2},
       ];
       let resultado=escena(daleks.to_vec());
       assert!(resultado);
    }
}
