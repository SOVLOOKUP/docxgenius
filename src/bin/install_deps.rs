use docxtemplate::deps::deps;
use j4rs::{Jvm, JvmBuilder, MavenArtifactRepo, MavenSettings};

fn main() {
  let jvm: Jvm = JvmBuilder::new()
    .with_maven_settings(MavenSettings::new(vec![MavenArtifactRepo::from(
      "jitpack.io::https://www.jitpack.io",
    )]))
    .build()
    .unwrap();

  deps(&jvm);
}
