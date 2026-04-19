use super::support::{json_files_with_prefix, read_json_file};
use crate::dnd5e::normalize::{
    normalize_class, normalize_class_feature, normalize_subclass, normalize_subclass_feature,
};
use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::ClassFile;
use crate::dnd5e::write;
use dllm_client::DbConnection;
use std::path::Path;
use tracing::info;

pub fn seed_classes(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let class_dir = data_dir.join("class");
    info!(?class_dir, "reading classes");

    let mut report = SectionReport::new("classes");

    for (_, path) in json_files_with_prefix(&class_dir, "class-") {
        let Some(class_file) = read_json_file::<ClassFile>(&path, &mut report) else {
            continue;
        };

        for raw_class in class_file.classes {
            report.note_seen();
            let class = normalize_class(raw_class, &mut report);
            let item_name = format!("{} [{}]", class.name, class.source);

            match write::class(conn, class) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded classes");
    report
}

pub fn seed_subclasses(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let class_dir = data_dir.join("class");
    info!(?class_dir, "reading subclasses");

    let mut report = SectionReport::new("subclasses");

    for (_, path) in json_files_with_prefix(&class_dir, "class-") {
        let Some(class_file) = read_json_file::<ClassFile>(&path, &mut report) else {
            continue;
        };

        for raw_subclass in class_file.subclasses {
            report.note_seen();
            let subclass = normalize_subclass(raw_subclass, &mut report);
            let item_name = format!(
                "{} ({}) [{}]",
                subclass.name, subclass.class_name, subclass.source
            );

            match write::subclass(conn, subclass) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded subclasses");
    report
}

pub fn seed_class_features(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let class_dir = data_dir.join("class");
    info!(?class_dir, "reading class features");

    let mut report = SectionReport::new("class_features");

    for (_, path) in json_files_with_prefix(&class_dir, "class-") {
        let Some(class_file) = read_json_file::<ClassFile>(&path, &mut report) else {
            continue;
        };

        for raw_feature in class_file.class_features {
            report.note_seen();
            let feature = normalize_class_feature(raw_feature, &mut report);
            let item_name = format!(
                "{} ({}) [{}]",
                feature.name, feature.class_name, feature.source
            );

            match write::class_feature(conn, feature) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded class features");
    report
}

pub fn seed_subclass_features(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let class_dir = data_dir.join("class");
    info!(?class_dir, "reading subclass features");

    let mut report = SectionReport::new("subclass_features");

    for (_, path) in json_files_with_prefix(&class_dir, "class-") {
        let Some(class_file) = read_json_file::<ClassFile>(&path, &mut report) else {
            continue;
        };

        for raw_feature in class_file.subclass_features {
            report.note_seen();
            let feature = normalize_subclass_feature(raw_feature, &mut report);
            let item_name = format!(
                "{} ({}/{}) [{}]",
                feature.name, feature.class_name, feature.subclass_short_name, feature.source
            );

            match write::subclass_feature(conn, feature) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded subclass features");
    report
}
