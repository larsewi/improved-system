#ifndef __IMPROVED_H__
#define __IMPROVED_H__

#define SQUASH = 1

extern int isys_init(const char *work_dir);
extern int isys_commit(void);
extern int isys_diff(const char *block, int flags);

#endif /* __IMPROVED_H__ */
