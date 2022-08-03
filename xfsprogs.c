#include "xfsprogs.h"

int geometry(int fd, struct xfs_fsop_geom *fsgeo) {
  int ret;
  memset(fsgeo, 0, sizeof(*fsgeo));
  ret = ioctl(fd, XFS_IOC_FSGEOMETRY, fsgeo);
  if (!ret)
    return 0;
  ret = ioctl(fd, XFS_IOC_FSGEOMETRY_V4, fsgeo);
  if (!ret)
    return 0;
  ret = ioctl(fd, XFS_IOC_FSGEOMETRY_V1, fsgeo);
  if (!ret)
    return 0;
  return -errno;
}

int growfs_data(int fd, xfs_growfs_data_t *fsgrow) {
  return ioctl(fd, XFS_IOC_FSGROWFSDATA, fsgrow);
}
