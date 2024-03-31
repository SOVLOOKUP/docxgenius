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

  // todo
  // https://github.com/Sayi/poi-tl/blob/master/poi-tl-cli/src/main/java/com/deepoove/poi/cli/CLI.java
  //   ConfigureBuilder builder = Configure.builder();
  // GsonHandler gsonHandler = new DefaultGsonHandler() {
  //     @Override
  //     protected RuntimeTypeAdapterFactory<RenderData> createRenderTypeAdapter(boolean readable) {
  //         return super.createRenderTypeAdapter(readable).registerSubtype(MarkdownRenderData.class, "markdown")
  //                 .registerSubtype(HighlightRenderData.class, "code")
  //                 .registerSubtype(FileMarkdownRenderData.class, "markdown-file");
  //     }

  //     @Override
  //     protected List<RuntimeTypeAdapterFactory<?>> createTypeAdapters(boolean readable) {
  //         List<RuntimeTypeAdapterFactory<?>> typeAdapter = super.createTypeAdapters(readable);
  //         typeAdapter.add(RuntimeTypeAdapterFactory.of(MarkdownRenderData.class, "type", readable)
  //                 .registerSubtype(MarkdownRenderData.class, "markdown"));
  //         typeAdapter.add(RuntimeTypeAdapterFactory.of(HighlightRenderData.class, "type", readable)
  //                 .registerSubtype(HighlightRenderData.class, "code"));
  //         typeAdapter.add(RuntimeTypeAdapterFactory.of(MarkdownRenderData.class, "type", readable)
  //                 .registerSubtype(MarkdownRenderData.class, "markdown")
  //                 .registerSubtype(FileMarkdownRenderData.class, "markdown-file"));
  //         return typeAdapter;
  //     }
  // };
  // GsonPreRenderDataCastor gsonPreRenderDataCastor = new GsonPreRenderDataCastor();
  // gsonPreRenderDataCastor.setGsonHandler(gsonHandler);
  // builder.addPreRenderDataCastor(gsonPreRenderDataCastor);
  // builder.addPlugin(':', new CommentRenderPolicy())
  //         .addPlugin(';', new AttachmentRenderPolicy())
  //         .addPlugin('~', new HighlightRenderPolicy())
  //         .addPlugin('-', new MarkdownRenderPolicy());
  // builder.bind("toc", new TOCRenderPolicy());

  // Configure configure = builder.build();
  // try {
  //     String jsonStr = ""; // 这里从 js 传过来即可

  //     XWPFTemplate.compile(template, configure)
  //             .render(gsonHandler.castJsonToType(jsonStr, TYPE))
  //             .writeToFile(output);

  Ok(())
}
