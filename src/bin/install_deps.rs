use j4rs::{Jvm, JvmBuilder, MavenArtifact, MavenArtifactRepo, MavenSettings};
include!(concat!(env!("OUT_DIR"), "/deps.rs"));

fn main() {
  let jvm: Jvm = JvmBuilder::new()
    .with_maven_settings(MavenSettings::new(vec![MavenArtifactRepo::from(
      "jitpack.io::https://www.jitpack.io",
    )]))
    .build()
    .unwrap();
  deps(&jvm);
}
