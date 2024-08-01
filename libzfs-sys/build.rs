// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate bindgen;
extern crate pkg_config;

use std::env;

fn main() {
    if cfg!(target_os = "macos") {
        return;
    }

    let out_file = env::current_dir().unwrap().join("src").join("bindings.rs");

    env::set_var("LIBCLANG_PATH", "/opt/llvm-5.0.0/lib64/");

    pkg_config::Config::new()
        .atleast_version("0.7.13")
        .probe("libzfs")
        .unwrap();
    println!("cargo:rustc-link-lib=zpool");

    // Skip building if bindings already exist.
    // If you want to rebuild, delete the bindings file.
    if out_file.exists() {
        return;
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .constified_enum_module("boolean")
        .allowlist_var("vdev_stat_t")
        .allowlist_type("vdev_stat_t")
        .allowlist_var("ZPOOL_MAXPROPLEN")
        .allowlist_var("ZPOOL_CONFIG_POOL_NAME")
        .allowlist_var("ZPOOL_CONFIG_TYPE")
        .allowlist_var("ZPOOL_CONFIG_VDEV_TREE")
        .allowlist_var("ZPOOL_CONFIG_CHILDREN")
        .allowlist_var("ZPOOL_CONFIG_SPARES")
        .allowlist_var("ZPOOL_CONFIG_L2CACHE")
        .allowlist_var("ZPOOL_CONFIG_PATH")
        .allowlist_var("ZPOOL_CONFIG_PHYS_PATH")
        .allowlist_var("ZPOOL_CONFIG_DEVID")
        .allowlist_var("ZPOOL_CONFIG_WHOLE_DISK")
        .allowlist_var("ZPOOL_CONFIG_IS_LOG")
        .allowlist_var("ZPOOL_CONFIG_HOSTID")
        .allowlist_var("ZPOOL_CONFIG_HOSTNAME")
        .allowlist_var("ZPOOL_CONFIG_GUID")
        .allowlist_var("ZPOOL_CONFIG_AUX_STATE")
        .allowlist_var("ZPOOL_CONFIG_VDEV_STATS")
        .allowlist_var("VDEV_TYPE_ROOT")
        .allowlist_var("VDEV_TYPE_MIRROR")
        .allowlist_var("VDEV_TYPE_REPLACING")
        .allowlist_var("VDEV_TYPE_RAIDZ")
        .allowlist_var("VDEV_TYPE_DISK")
        .allowlist_var("VDEV_TYPE_FILE")
        .allowlist_var("VDEV_TYPE_MISSING")
        .allowlist_var("VDEV_TYPE_HOLE")
        .allowlist_var("VDEV_TYPE_SPARE")
        .allowlist_var("VDEV_TYPE_LOG")
        .allowlist_var("VDEV_TYPE_L2CACHE")
        .allowlist_var("ZPROP_VALUE")
        .allowlist_var("ZFS_MAXPROPLEN")
        .allowlist_var("ZFS_MAX_DATASET_NAME_LEN")
        .allowlist_type("zpool_prop_t")
        .constified_enum_module("zpool_prop_t")
        .allowlist_type("pool_state_t")
        .constified_enum_module("pool_state")
        .bitfield_enum("zfs_type_t")
        .opaque_type("libzfs_handle_t")
        .blocklist_type("nvlist_t")
        .blocklist_type("nvlist")
        .allowlist_function("libzfs_init")
        .allowlist_function("libzfs_fini")
        .allowlist_function("thread_init")
        .allowlist_function("thread_fini")
        .allowlist_function("zpool_import")
        .allowlist_function("zpool_export")
        .allowlist_function("zpool_search_import")
        .allowlist_function("zpool_iter")
        .allowlist_function("zpool_open_canfail")
        .allowlist_function("zpool_close")
        .allowlist_function("zpool_get_name")
        .allowlist_function("zpool_get_state")
        .allowlist_function("zpool_pool_state_to_name")
        .allowlist_function("zpool_get_prop_int")
        .allowlist_function("zpool_get_prop")
        .allowlist_function("zpool_get_config")
        .allowlist_function("zpool_get_handle")
        .allowlist_function("zpool_state_to_name")
        .allowlist_function("zfs_open")
        .allowlist_function("zfs_close")
        .allowlist_function("zfs_iter_filesystems")
        .allowlist_function("zfs_get_name")
        .allowlist_function("zfs_get_user_props")
        .allowlist_function("zfs_get_type")
        .allowlist_function("zfs_type_to_name")
        .allowlist_function("zfs_path_to_zhandle")
        .allowlist_function("zpool_disable_datasets")
        .allowlist_function("libzfs_error_description")
        .allowlist_function("zfs_prop_get")
        .allowlist_function("zfs_expand_proplist")
        .allowlist_function("zfs_prop_to_name")
        .allowlist_function("zfs_validate_name")
        .allowlist_function("zprop_free_list")
        .clang_arg("-I/usr/lib/gcc/x86_64-redhat-linux/4.8.2/include/")
        .clang_arg("-I/usr/src/zfs-0.7.13/lib/libspl/include/")
        .clang_arg("-I/usr/src/zfs-0.7.13/include/")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to src.
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
