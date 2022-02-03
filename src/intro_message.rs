use clap::{
  crate_name,
  crate_authors,
  crate_description,
  crate_version,
};

pub fn intro_message() {
  println!(
    "{} - {}\n{}\n{}\n{}{}{}",
    crate_name!(),
    crate_version!(),
    crate_authors!(),
    crate_description!(),
    "Using '.exit' or '.quit' to quit.\n",
    "Using '.help' for usage hints.\n",
    "Using '.open FILENAME' to reopen on a persistent database."
  );
}
