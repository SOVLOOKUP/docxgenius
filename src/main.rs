mod deps;
use j4rs::{Jvm, JvmBuilder, MavenArtifactRepo, MavenSettings};
use std::fs::remove_dir_all;

const JAVA_DEPS_HOME: &str = "java";

fn main() {
  let jvm: Jvm = JvmBuilder::new()
    .with_maven_settings(MavenSettings::new(vec![MavenArtifactRepo::from(
      "jitpack.io::https://www.jitpack.io",
    )]))
    .build()
    .unwrap();

  deps::deps(&jvm);

  let _ = remove_dir_all(JAVA_DEPS_HOME);
  let _ = Jvm::copy_j4rs_libs_under(JAVA_DEPS_HOME);
}
