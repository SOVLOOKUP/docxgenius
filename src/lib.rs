#![deny(clippy::all)]
#![feature(fs_try_exists)]
// auto import deps
include!(concat!(env!("OUT_DIR"), "/deps.rs"));
use std::{env, fs, path::PathBuf};

use j4rs::{
  Instance, InvocationArg, Jvm, JvmBuilder, MavenArtifact, MavenArtifactRepo, MavenSettings,
};
use napi_derive::napi;
use rust_embed::RustEmbed;

// Dependencies Jar
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/target/debug/jassets"]
struct Jassets;

// Dependencies deps
#[cfg(feature = "java_callback")]
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/target/debug/deps"]
struct Deps;

// dump dependencies Jar
fn dump(poitl_path: &PathBuf) {
  let jars_path = poitl_path.join("jassets");
  let _ = fs::create_dir_all(&jars_path);

  for item in Jassets::iter() {
    let name = item.to_string();
    let binding = Jassets::get(&name).unwrap();
    let file = binding.data.as_ref();

    let jar_path = jars_path.join(name);
    if let Ok(false) = fs::try_exists(&jar_path) {
      let _ = fs::write(jar_path, file);
    }
  }

  #[cfg(feature = "java_callback")]
  {
    let deps_path = poitl_path.join("deps");
    let _ = fs::create_dir_all(&deps_path);

    for item in Deps::iter() {
      let name = item.to_string();
      let binding = Deps::get(&name).unwrap();
      let file = binding.data.as_ref();

      let dep_path = deps_path.join(name);
      if let Ok(false) = fs::try_exists(&dep_path) {
        let _ = fs::write(dep_path, file);
      }
    }
  }
}

#[napi]
pub struct DocxTemplate {
  jvm: Jvm,
  instance: Instance,
}

#[napi]
impl DocxTemplate {
  #[napi(constructor)]
  pub fn new() -> Self {
    let poitl_path = env::temp_dir().join("poitl");
    dump(&poitl_path);
    let base_path = poitl_path.to_str().unwrap();
    let jvm: Jvm = JvmBuilder::new()
      .with_maven_settings(MavenSettings::new(vec![MavenArtifactRepo::from(
        "jitpack.io::https://www.jitpack.io",
      )]))
      .with_base_path(base_path)
      .build()
      .unwrap();
    deps(&jvm);

    let instance = jvm
      .create_instance("com.github.SOVLOOKUP.docx.template.DocxTemplate", &[])
      .unwrap();

    return DocxTemplate { jvm, instance };
  }

  fn _render_file(
    &self,
    tpl_path: &str,
    out_path: &str,
    json_data: &str,
  ) -> j4rs::errors::Result<()> {
    let tpl_path_args = InvocationArg::try_from(tpl_path)?;
    let out_path_args = InvocationArg::try_from(out_path)?;
    let json_data_args = InvocationArg::try_from(json_data)?;

    self.jvm.invoke(
      &self.instance,
      "run",
      &[tpl_path_args, out_path_args, json_data_args],
    )?;

    Ok(())
  }

  fn _render_byte(&self, template: Vec<i8>, json_data: &str) -> j4rs::errors::Result<Vec<i8>> {
    let json_data_args = InvocationArg::try_from(json_data)?;

    let args: Vec<InvocationArg> = template
      .iter()
      .map(|i| InvocationArg::try_from(i).unwrap())
      .collect();

    let arr_instance = self.jvm.create_java_array("java.lang.Byte", &args)?;

    let instance = self.jvm.invoke(
      &self.instance,
      "run_byte",
      &[InvocationArg::try_from(arr_instance)?, json_data_args],
    )?;

    let out: Vec<i8> = self.jvm.to_rust(instance)?;

    Ok(out)
  }

  #[napi]
  pub fn render_file(&self, tpl_path: String, out_path: String, json_data: String) {
    let _ = self._render_file(&tpl_path, &out_path, &json_data);
  }

  #[napi]
  pub fn render_byte(&self, template: Vec<i8>, json_data: String) -> Vec<i8> {
    self._render_byte(template, &json_data).unwrap()
  }
}
