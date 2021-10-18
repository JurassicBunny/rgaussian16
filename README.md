Interface for the Gaussian16 quantum chemical package.
This structure provides functions for auto generating input, and
running the external program `g16`.

As configuration is tied to Gaussian, multiple Gaussian objects may
be use to extract different result form Gaussian16. This greatly 
simplifies the processes of interfacing with the quantum chemical
package.

# Example

 Generate Gaussian object write input and run `g16`:
 
```rust

   use rgaussian16::Gaussion;

   fn main() {
     let input_file = std::fs::File::create("input.com").unwrap();
     let output_file = std::fs::File::create("output.out").unwrap();
  
     let job1_config = std::fs::File::open("config.yaml").unwrap();
     let job1_interface = Gaussion::new(config).unwrap();
 
     job1_interface.gen_input(input_file).unwrap();
     job1_interface.run(input_file, output_file).unwrap();
   }
```
