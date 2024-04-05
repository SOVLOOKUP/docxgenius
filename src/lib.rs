mod deps;

use std::{env, fs, path::PathBuf};

use j4rs::{Instance, InvocationArg, Jvm, JvmBuilder, MavenArtifactRepo, MavenSettings};
use napi_derive::napi;
use rust_embed::RustEmbed;
use walkdir::WalkDir;

// Dependencies Jar
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/java/jassets"]
struct Jassets;

// Dependencies deps
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/java/deps"]
struct Deps;

#[napi]
pub struct DocxTemplate {
  jvm: Jvm,
  instance: Instance,
}

// dump dependencies
fn dump(poitl_path: &PathBuf) {
  let jars_path = poitl_path.join("jassets");
  dump_assets::<Jassets>(&jars_path);

  let deps_path = poitl_path.join("deps");
  dump_assets::<Deps>(&deps_path);
}

fn dump_assets<T: RustEmbed>(path: &PathBuf) {
  fs::create_dir_all(path).unwrap();

  let path_iter: Vec<String> = WalkDir::new(path)
    .into_iter()
    .map(|i| i.unwrap().file_name().to_string_lossy().into_owned())
    .collect();

  for item in T::iter() {
    let name = item.to_string();

    // 删除过时依赖
    // 拓展名
    let rev_name = name.chars().rev().collect::<String>();
    let (target, _) = rev_name.split_once(".").unwrap();
    let ext = target.chars().rev().collect::<String>();

    // 依赖名称
    let (_, target) = rev_name.split_once("-").unwrap();
    let pkg = target.chars().rev().collect::<String>();

    let pkg_ = pkg.clone() + "-";
    let _ = path_iter
      .clone()
      .into_iter()
      .filter(|entry| entry.starts_with(&pkg))
      .filter(|entry| entry.ends_with(&ext))
      .filter(|entry| !(entry.eq(&name)))
      .filter(|entry| !entry.replace(&pkg_, "").contains("-"))
      .map(|entry| fs::remove_file(path.join(&entry)).unwrap())
      .collect::<Vec<()>>();

    // dump 依赖
    let binding = T::get(&name).unwrap();
    let file = binding.data.as_ref();

    let file_path = path.join(name);

    if !file_path.exists() {
      let _ = fs::write(&file_path, file);
    }
  }
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
    deps::deps(&jvm);

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

  #[napi]
  pub fn template_meta(&self, template: String) -> Vec<String> {
    let template_args = InvocationArg::try_from(template).unwrap();

    let out_byte = self
      .jvm
      .invoke(&self.instance, "templateMeta", &[template_args])
      .unwrap();

    self.jvm.to_rust(out_byte).unwrap()
  }
}
