/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
 *
 * Copyright (c) 2022, 郑昱笙
 * All rights reserved.
 */

#include <cassert>
#include <fstream>
#include <iostream>
#include <string>

#include "eunomia/eunomia-bpf.hpp"

#define TASK_COMM_LEN 16
#define NAME_MAX 255

struct event {
	/* user terminology for pid: */
	unsigned long long ts;
	int pid;
	int uid;
	int ret;
	int flags;
	char comm[TASK_COMM_LEN];
	char fname[NAME_MAX];
};



int main(int argc, char **argv)
{
  return 0;
}