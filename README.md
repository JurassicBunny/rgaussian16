# rgaussian16

Interface for the Gaussian16 [Gaussian16](https://gaussian.com/gaussian16/) quantum chemical package.
This structure provides functions for auto generating input, and
running the external program `g16`.

As configuration is tied to Gaussian, multiple Gaussian objects may
be used to extract different results form Gaussian16. This greatly 
simplifies the processes of interfacing with the quantum chemical
package.

# Configuration 

An example configuration file.

config.yaml:
```yaml
---
mem: "136GB"
cpu: "0-47"
gpu: ~
checkpoint: "output.chk"
key_words: "#p WB97XD/Def2tzvpp SCF=XQC"
title: "single point"
charge: 0
multiplicity: 3
```

Gaussian16 may be run with graphical processing units (gpus). In-order to generate 
input for gpu targeted calculations, replace `~` with the appropriate string. For 
example, changing the above configurations gpu field to `"0=0"` will instruct Gaussian16 
to utilize one gpu controlled by cpu number 0.

# Example Usage

 Generate Gaussian object write input and run `g16`:
 
```rust

   use rgaussian16::Gaussion;

   fn main() -> Result<()> {

     let input_file = std::fs::File::create("input.com")?;
     let output_file = std::fs::File::create("output.out")?;
  
     let job1_config = std::fs::File::open("config.yaml")?;
     let job1_interface = Gaussion::new(config)?;
 
     job1_interface.gen_input(input_file)?;
     job1_interface.run(input_file, output_file)?;

	 Ok(())
   }
```
