use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
  // java deps
  let output = if cfg!(target_os = "windows") {
    Command::new("mvn.cmd")
      .arg("dependency:list")
      .output()
      .unwrap()
  } else if cfg!(target_os = "macos") {
    Command::new("mvn").arg("dependency:list").output().unwrap()
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("mvn dependency:list")
      .output()
      .unwrap()
  };

  let ls_la_list = String::from_utf8(output.stdout).unwrap();

  println!("mvn output: {}", ls_la_list);

  let binding = ls_la_list
    .replace("[INFO]    ", "")
    .replace(":jar:", ":")
    .replace(":runtime", "")
    .replace(":compile", "");

  let out: Vec<&str> = binding
    .split("The following files have been resolved:")
    .collect();

  let out2: Vec<&str> = out[1].split("[INFO]").collect();

  let out3: Vec<&str> = out2[0].split("\n").collect();

  let mut out_deps: String = "".to_string();

  for d in out3 {
    let out4: Vec<&str> = d.split(" -- ").collect();
    let out5 = out4[0].trim();
    if out5.len() > 3 {
      out_deps += "
  let dbx_artifact = MavenArtifact::from(\"";
      out_deps += out5;
      out_deps += "\");
  jvm.deploy_artifact(&dbx_artifact).unwrap();";
    }
  }

  let dest_path = Path::new("src").join("deps.rs");
  fs::write(
    &dest_path,
    "use j4rs::{Jvm, MavenArtifact};
pub fn deps(jvm: &Jvm) {"
      .to_owned()
      + &out_deps
      + "\n}",
  )
  .unwrap();
}
