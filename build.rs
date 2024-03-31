extern crate napi_build;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
  // napi
  napi_build::setup();

  // java deps
  let mut mvn: String = "mvn".to_string();

  if cfg!(target_os = "windows") {
    mvn += ".cmd";
  }

  let output = Command::new(mvn)
    .arg("dependency:list")
    .output()
    .expect("命令执行异常错误提示");

  let ls_la_list = String::from_utf8(output.stdout).unwrap();

  let binding = ls_la_list
    .replace("[INFO]    ", "")
    .replace(":jar:", ":")
    .replace(":compile", "");

  let out: Vec<&str> = binding
    .split("The following files have been resolved:")
    .collect();

  let out2: Vec<&str> = out[1].split("[INFO]").collect();

  let out3: Vec<&str> = out2[0].split("\n").collect();

  let mut out_deps: String = "".to_string();

  for d in out3 {
    let out4: Vec<&str> = d.split(" -- ").collect();
    let out5 = out4[0];
    if out5.len() > 3 {
      out_deps += "
    let dbx_artifact = MavenArtifact::from(\"";
      out_deps += out5;
      out_deps += "\");
    jvm.deploy_artifact(&dbx_artifact).unwrap();";
    }
  }

  let out_dir = env::var_os("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("deps.rs");
  fs::write(
    &dest_path,
    "pub fn deps(jvm: &Jvm) {".to_owned() + &out_deps + "\n}",
  )
  .unwrap();

  println!("cargo:rerun-if-changed=pom.xml");
}
