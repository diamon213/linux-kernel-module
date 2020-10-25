#include <linux/module.h>
#include <linux/slab.h>

MODULE_AUTHOR("diamon");
MODULE_DESCRIPTION("A simple kernel module");
MODULE_VERSION("0.1.0");

extern int init_module(void);
extern void cleanup_module(void);
