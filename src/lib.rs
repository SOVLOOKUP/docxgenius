#![feature(fs_try_exists)]
#![deny(clippy::all)]

// auto import deps
include!(concat!(env!("OUT_DIR"), "/deps.rs"));
use std::{env, fs, path::PathBuf};

use j4rs::{InvocationArg, Jvm, JvmBuilder, MavenArtifact};
use napi_derive::napi;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

// Dependencies Jar
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/target/debug/jassets"]
struct Asset;

// todo
#[napi(object)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MyData {
  pub story_name: String,
  pub story_author: String,
  pub story_source: String,
}

// render docx template
#[napi]
pub fn render(tpl_path: String, out_path: String, my_data: MyData) -> napi::Result<()> {
  let poitl_path = env::temp_dir().join("poitl");
  dump(&poitl_path);
  let base_path = poitl_path.to_str().unwrap();
  let jvm: Jvm = JvmBuilder::new().with_base_path(base_path).build().unwrap();
  deps(&jvm);
  let _ = render1(&jvm, &tpl_path, &out_path, &my_data);
  Ok(())
}

// dump dependencies Jar
fn dump(poitl_path: &PathBuf) {
  let jars_path = poitl_path.join("jassets");
  let _ = fs::create_dir_all(&jars_path);

  for item in Asset::iter() {
    let name = item.to_string();
    let binding = Asset::get(&name).unwrap();
    let file = binding.data.as_ref();

    let jar_path = jars_path.join(name);
    if let Ok(false) = fs::try_exists(&jar_path) {
      let _ = fs::write(jar_path, file);
    }
  }
}

fn render1(
  jvm: &Jvm,
  tpl_path: &str,
  out_path: &str,
  my_data: &MyData,
) -> j4rs::errors::Result<()> {
  let tpl_path_args = InvocationArg::try_from(tpl_path)?;

  let compile_template =
    jvm.invoke_static("com.deepoove.poi.XWPFTemplate", "compile", &[tpl_path_args])?;

  let datas = InvocationArg::new(my_data, "java.util.HashMap");

  let result = jvm.invoke(&compile_template, "render", &[datas])?;

  let out = InvocationArg::try_from(out_path)?;

  jvm.invoke(&result, "writeToFile", &[out])?;

  Ok(())
}
