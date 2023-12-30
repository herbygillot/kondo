use std::path::Path;

use enum_dispatch::enum_dispatch;

mod node;
mod rust;
mod unity;

use node::NodeProject;
use rust::RustProject;
use unity::UnityProject;

#[enum_dispatch]
#[derive(Debug, Clone, Copy)]
pub enum ProjectEnum {
    RustProject,
    NodeProject,
    UnityProject,
}

impl ProjectEnum {
    pub const ALL: [Self; 3] = [
        Self::RustProject(RustProject),
        Self::NodeProject(NodeProject),
        Self::UnityProject(UnityProject),
    ];
}

#[enum_dispatch(ProjectEnum)]
pub trait Project {
    fn name(&self) -> &str;
    fn is_project(&self, root_dir: &Path) -> bool;
    fn is_artifact(&self, path: &Path) -> bool;
}

// #[cfg(test)]
// mod tests {
//     use crate::test::TestDirectoryBuilder;

//     use super::*;

//     #[test]
//     fn bramm() {
//         let td = TestDirectoryBuilder::default()
//             .file("package.json")
//             .file("src/main.js")
//             .artifact("node_modules/foo")
//             .artifact("node_modules/bar")
//             .build()
//             .unwrap();

//         assert!(NodeProject.is_project(&pp.root));

//         pp.artifacts.iter().for_each(|p| {
//             std::fs::remove_file(p).unwrap();
//         });

//         // do clean

//         assert!(pp.fully_clean());
//         // assert!(node_p.artifact_bytes(), 20);
//     }
// }