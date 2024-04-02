#![deny(clippy::all)]
#![feature(fs_try_exists)]
// auto import deps
include!(concat!(env!("OUT_DIR"), "/deps.rs"));
use std::{env, fs, path::PathBuf};

use j4rs::{
  Instance, InvocationArg, JavaClass, Jvm, JvmBuilder, MavenArtifact, MavenArtifactRepo,
  MavenSettings,
};
use napi::bindgen_prelude::Int8Array;
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
    DocxTemplate::dump(&poitl_path);
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

    DocxTemplate { jvm, instance }
  }

  #[napi]
  pub fn render_file(&self, tpl_path: String, out_path: String, json_data: String) {
    let tpl_path_args = InvocationArg::try_from(tpl_path).unwrap();
    let out_path_args = InvocationArg::try_from(out_path).unwrap();
    let json_data_args = InvocationArg::try_from(json_data).unwrap();

    self
      .jvm
      .invoke(
        &self.instance,
        "renderFile",
        &[tpl_path_args, out_path_args, json_data_args],
      )
      .unwrap();
  }

  #[napi]
  pub fn render_byte(&self, template: Int8Array, json_data: String) -> Int8Array {
    let array = self
      .jvm
      .java_list(JavaClass::Byte, template.to_vec())
      .unwrap();

    let template_args = InvocationArg::try_from(array).unwrap();
    let json_data_args = InvocationArg::try_from(json_data).unwrap();

    let out_byte = self
      .jvm
      .invoke(
        &self.instance,
        "renderByte",
        &[template_args, json_data_args],
      )
      .unwrap();

    let o: Vec<i8> = self.jvm.to_rust(out_byte).unwrap();

    o.into()
  }

  #[napi]
  pub fn render_base64(&self, template: String, json_data: String) -> String {
    let template_args = InvocationArg::try_from(template).unwrap();
    let json_data_args = InvocationArg::try_from(json_data).unwrap();

    let out_byte = self
      .jvm
      .invoke(
        &self.instance,
        "renderBase64",
        &[template_args, json_data_args],
      )
      .unwrap();

    self.jvm.to_rust(out_byte).unwrap()
  }

  // dump dependencies Jar todo 合二为一
  fn dump(poitl_path: &PathBuf) {
    let jars_path = poitl_path.join("jassets");

    // todo 计算比对 hash
    if let Ok(true) = fs::try_exists(&jars_path) {
      fs::remove_dir_all(&jars_path).unwrap();
    }

    fs::create_dir_all(&jars_path).unwrap();

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

      if let Ok(true) = fs::try_exists(&deps_path) {
        fs::remove_dir_all(&deps_path).unwrap();
      }

      fs::create_dir_all(&deps_path).unwrap();

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
}
