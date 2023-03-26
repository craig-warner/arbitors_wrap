extern crate colored; // not needed in Rust 2018

use clap::{crate_version,App,Arg};
use std::process;
use colored::*;
use arbitors::rrarb::RRArb;

// Round Robin Arbitor Wrapper
// * Tests Arbitor
// * Tests Display 
fn main() {
  let arr = [ "  A program to implement a Round Robin Arbitor\n",
    "\n"
  ];
  let about_str: String = arr.join("");
  let mut max_requestors:u64 = 256;
  let mut is_verbose:bool = false;
  
  let matches = App::new("Round Robin Arb")

    .version(crate_version!())
    .author("Written by: Craig Warner")
    .about( about_str.as_str())
    .arg(Arg::with_name("REQUESTORS")
      .long("requestors")
      .short("r")
      .multiple(true)
      .help("Number of Requestors")
      .takes_value(true)
      .default_value("256")
    )
    .get_matches();

  if matches.is_present("verbose") {
    is_verbose = true;
  }
  // Argument Parsing: requestors 
  if let Some(input) = matches.value_of("REQUESTORS") {
    match input.parse::<u64>() {
      Ok(n) => {
        if is_verbose {
          println!("Number of Requestors = {}", n);
        }
        max_requestors= n;
      },
      Err(n) => {
        eprintln!("{}Requestors is not supported {}","Error:".red(),n);
        process::exit(1) 
      }
    }
  }
  // Body of Main
  let mut rrarb = RRArb::new(max_requestors,false);
  rrarb.set_request(5);
  rrarb.text_display();
  rrarb.step();
  rrarb.text_display();
  let selected = rrarb.arb();
  rrarb.text_display();
  match selected {
    Some(n) => println!("Selected {}",n),
    None => println!("Empty Requestor Set")
  }

  let mut rrarb2 = RRArb::new(10,false);
  rrarb2.set_request(0);
  rrarb2.set_request(3);
  rrarb2.set_request(5);
  rrarb2.text_display();
  let rrarb2_iter = rrarb2.clone().iter();
  for rq in rrarb2_iter {
    println!("Requestor {}",rq);
    if rq == 0 {
      rrarb2.grant(rq);
    }
  } 
  rrarb2.text_display();
  let rrarb2_iter = rrarb2.clone().iter();
  for rq in rrarb2_iter {
    println!("Requestor {}",rq);
  }
  rrarb2.text_display();

}
