# cajita
Un ejemplo sencillo de como estructurar el código de un Crate en Rust.  

Para compilar, probar y ver la documentacion, ejecutar las siguientes instrucciones:  

cargo clean  
cargo build  
cargo test  
cargo doc --open  

Para usar como un crate, editar en el archivo Cargo.toml del proyecto
[dependencies]  
cajita = {git = "https://github.com/gusdelact/cajita.git" }


