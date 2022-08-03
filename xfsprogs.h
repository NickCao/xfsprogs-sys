#include <string.h>
#include <xfs/xfs.h>

int geometry(int fd, struct xfs_fsop_geom *fsgeo);
int growfs_data(int fd, xfs_growfs_data_t *fsgrow);
