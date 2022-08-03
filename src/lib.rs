use anyhow::anyhow;
use std::os::unix::prelude::AsRawFd;
mod bindings;
pub use bindings::{xfs_fsop_geom, xfs_growfs_data_t};

pub fn geometry(file: &std::fs::File) -> anyhow::Result<xfs_fsop_geom> {
    unsafe {
        let mut fsgeo: xfs_fsop_geom = std::mem::zeroed();
        if bindings::geometry(file.as_raw_fd(), &mut fsgeo as *mut xfs_fsop_geom) < 0 {
            Err(anyhow!("xfrog_geometry"))
        } else {
            Ok(fsgeo)
        }
    }
}

pub fn growfs_data(file: &std::fs::File, mut fsgrow: xfs_growfs_data_t) -> anyhow::Result<()> {
    unsafe {
        if bindings::growfs_data(file.as_raw_fd(), &mut fsgrow as *mut xfs_growfs_data_t) < 0 {
            Err(anyhow!("growfs_data"))
        } else {
            Ok(())
        }
    }
}
